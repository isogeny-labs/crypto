//! Read-only central initial-prefix verification. No generator, clock renewal,
//! producer-budget creation, mutable original factory, or exporter callback.
use super::*;
use super::boolean_rows::receipt as row_wire;
use provenance::boolean_equations::{self as row_equations, matrix_rows as row_matrix};
#[path = "sat_original_math_preflight.rs"]
mod original_math;
use crate::ml_dataset::{
    storage,
    types::{Artifact, CaseConfig, CaseData},
};
use std::path::Path;
#[path = "sat_producer_ledger.rs"]
mod ledger;
#[path = "sat_initial_replay.rs"]
mod transcript;
#[path = "sat_search_central.rs"]
mod search;
/// Must be supplied by the separately authorized replay caller, never inferred
/// from a successful producer prefix or reset after a failure.
pub(in crate::ml_dataset::solvers) struct Policy {
    pub work: u64,
    pub bytes: u64,
    pub deadline: Instant,
    pub stream_bytes: u64,
    pub payload_bytes: u64,
}
struct ReplayBudget {
    policy: Policy,
    work: u64,
    bytes: u64,
    first_stop: Option<Stop>,
}
impl DecodeBudget for ReplayBudget {
    fn reserve_checking(&mut self, w: u64, b: u64, now: Instant) -> Result<(), Stop> {
        DecodeBudget::check(self, now)?;
        if self
            .work
            .checked_add(w)
            .filter(|v| *v <= self.policy.work)
            .is_none()
        {
            return self.fail(Stop::Work);
        }
        if self
            .bytes
            .checked_add(b)
            .filter(|v| *v <= self.policy.bytes)
            .is_none()
        {
            return self.fail(Stop::Bytes);
        }
        self.work += w;
        self.bytes += b;
        Ok(())
    }
    fn check(&mut self, now: Instant) -> Result<(), Stop> {
        if let Some(s) = self.first_stop {
            return Err(s);
        }
        if now >= self.policy.deadline {
            return self.fail(Stop::Deadline);
        }
        Ok(())
    }
    fn fail<T>(&mut self, s: Stop) -> Result<T, Stop> {
        Err(*self.first_stop.get_or_insert(s))
    }
}
impl Admission for ReplayBudget {
    fn reserve(&mut self, q: Quote) -> Result<(), provenance::Error> {
        self.reserve_checking(q.work, q.bytes, Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
    fn check(&mut self) -> Result<(), provenance::Error> {
        DecodeBudget::check(self, Instant::now()).map_err(|_| provenance::Error::Admission)
    }
}
/// Every post-admission return transfers this frame. No borrowed verification
/// seal survives moving it; the success type below is minted only at the end.
pub(in crate::ml_dataset::solvers) struct Frame {
    budget: ReplayBudget,
    row_mode: Option<RowReplay>,
    search_mode: Option<search::Replay>,
    config: Vec<u8>,
    public: Vec<u8>,
    factor: Vec<u8>,
    case: Option<CaseData>,
    parts: DecodedParts,
    map_bytes: Vec<u8>,
    map: Option<provenance::CheckedMap>,
    raw_map: Option<provenance::Map>,
    transcript: Option<transcript::Transcript>,
    checkpoint: Option<ledger::Checkpoint>,
    pending_line: Vec<u8>,
    head: Option<capture::CommittedHead>,
    diagnostic: Option<String>,
    visited: Endpoint,
    math_refusal: Option<original_math::Error>,
    math_plan: Option<original_math::Plan>,
    pending_read: Option<retained_read::Pending>,
    head_raw: Vec<u8>,
    checked_head_raw: Vec<u8>,
    final_head_raw: Vec<u8>,
    checked_head: Option<capture::CommittedHead>,
    payload_check_raw: Vec<u8>,
    payload_check_complete: bool,
    terminal_raw: Vec<u8>,
    terminal: Option<Value>,
    terminal_head_raw: Vec<u8>,
    original_capture: Option<(capture::AttemptBinding, capture::CaptureLimits)>,
    revalidation_raw: Vec<u8>,
    revalidation_payload: Vec<u8>,
    revalidation_head: Option<capture::CommittedHead>,
    revalidation_config: Option<CaseConfig>,
    artifact_fingerprint: Option<[u8; 32]>,
    measurement_policy_hash: Option<[u8; 32]>,
}
impl Frame {
    pub(in crate::ml_dataset::solvers) fn usage(&self) -> (u64, u64) {
        (self.budget.work, self.budget.bytes)
    }
    pub(in crate::ml_dataset::solvers) fn has_checked_map(&self) -> bool {
        self.map.is_some()
    }
}
pub(in crate::ml_dataset::solvers) enum Outcome {
    Verified(VerifiedInitialCapture),
    IncompleteMissingProducerLedger(Frame),
    UnknownOriginalMath(Frame),
}
pub(in crate::ml_dataset::solvers) struct Failed {
    pub frame: Frame,
    pub stop: Stop,
}
pub(in crate::ml_dataset::solvers) struct VerifiedInitialCapture {
    frame: Frame,
}
impl VerifiedInitialCapture {
    /// Deliberately no export or training release method. This is initial-only.
    pub(in crate::ml_dataset::solvers) fn initial_and_ledger_consistent(&self) -> bool {
        self.frame.budget.first_stop.is_none()
    }
}
/// Whole terminal admission remains initial-only: it never grants search export.
pub(in crate::ml_dataset::solvers) struct TerminalVerified {
    pub capture: VerifiedInitialCapture,
    pub declared_initial: bool,
}
fn terminal_fields(
    config: &CaseConfig,
    outcome: &Value,
    pattern: u32,
    head: &capture::CommittedHead,
) -> Result<bool, &'static str> {
    if outcome["solver_completed"] != false
        || outcome["scalar_recovered_without_witness"] == true
        || outcome["ecdlp_pipeline_completed"] == true
        || outcome["status"] == "verified"
    {
        return Err("initial-only terminal overclaims solver scope");
    }
    let declared = outcome.get("initial_checked").is_some()
        || outcome["adapter_id"] == "sat-initial-v1"
        || outcome["strategy"] == "sat-initial-v1";
    if !declared {
        // Supervisor censoring replaces the worker terminal object. Its checked
        // capture remains evidence, but there is no admitted worker success claim.
        if !matches!(
            outcome["status"].as_str(),
            Some("timeout" | "worker_failed" | "trace_budget_exhausted" | "error")
        ) {
            return Err("missing SAT initial terminal declaration");
        }
        return Ok(false);
    }
    let total = config
        .m
        .checked_sub(2)
        .and_then(|n| u32::try_from(n).ok())
        .and_then(|n| 1u32.checked_shl(n))
        .ok_or("invalid SAT pattern count")?;
    if super::worker::eligibility(config) != super::worker::Eligibility::Requested
        || pattern != 0
        || outcome["adapter_id"] != "sat-initial-v1"
        || outcome["strategy"] != "sat-initial-v1"
        || outcome["status"] != "partial"
        || outcome["scalar_recovered_without_witness"] != false
        || outcome["initial_checked"] != true
        || outcome["pattern"].as_u64() != Some(pattern as u64)
        || outcome["patterns_total"].as_u64() != Some(total as u64)
        || outcome["patterns_checked"].as_u64() != Some(1)
        || outcome.get("first_stop") != Some(&Value::Null)
        || outcome["checkpoint_sequence"].as_u64() != Some(head.event_count)
        || outcome["checkpoint_sha256"].as_str() != Some(head.last_event_sha256.as_str())
        || head.stop.is_some()
    {
        return Err("SAT terminal differs from checked initial/checkpoint coverage");
    }
    Ok(true)
}
#[cfg(test)]
pub(in crate::ml_dataset::solvers) fn check_terminal_fields_for_control(
    config: &CaseConfig,
    outcome: &Value,
    head: &capture::CommittedHead,
) -> bool {
    terminal_fields(config, outcome, 0, head) == Ok(true)
}
impl VerifiedInitialCapture {
    /// Consume the same admitted replay owner. Failure is terminal and retains
    /// all original/map/transcript/checkpoint and terminal raw/parsed owners.
    pub(in crate::ml_dataset::solvers) fn join_terminal(
        mut self,
        dir: &Path,
        config: &CaseConfig,
        outcome: &Value,
        artifacts: &[Artifact],
    ) -> Result<TerminalVerified, Failed> {
        let result = (|| -> Result<bool, String> {
            self.frame
                .budget
                .reserve_checking(8192, 4096, Instant::now())
                .map_err(|_| "terminal join admission refused")?;
            let map = self
                .frame
                .map
                .as_ref()
                .ok_or("terminal missing checked map")?;
            let head = self
                .frame
                .head
                .as_ref()
                .ok_or("terminal missing checked head")?;
            let declared = terminal_fields(config, outcome, map.pattern(), head)?;
            if declared {
                let mut candidates = artifacts
                    .iter()
                    .filter(|a| a.path == "result.json" || a.role == "solver-outcome");
                let artifact = candidates
                    .next()
                    .ok_or("missing worker terminal artifact")?;
                if candidates.next().is_some()
                    || artifact.path != "result.json"
                    || artifact.role != "solver-outcome"
                    || artifact.encoding != "json"
                    || artifact.bytes > 32768
                {
                    return Err("worker terminal artifact identity".into());
                }
                self.frame.terminal_raw = read(
                    dir,
                    "result.json",
                    artifact.bytes,
                    &mut self.frame.budget,
                    &mut self.frame.pending_read,
                )?;
                if self.frame.terminal_raw.len() as u64 != artifact.bytes
                    || hex::encode(Sha256::digest(&self.frame.terminal_raw)) != artifact.sha256
                {
                    return Err("worker terminal artifact bytes differ".into());
                }
                let n = self.frame.terminal_raw.len() as u64;
                self.frame
                    .budget
                    .reserve_checking(
                        n.checked_mul(256)
                            .and_then(|n| n.checked_add(1024))
                            .ok_or("terminal parse overflow")?,
                        n.checked_mul(128).ok_or("terminal owner overflow")?,
                        Instant::now(),
                    )
                    .map_err(|_| "terminal parse admission refused")?;
                self.frame.terminal = Some(
                    serde_json::from_slice(&self.frame.terminal_raw).map_err(|e| e.to_string())?,
                );
                let stored = self.frame.terminal.as_ref().unwrap();
                terminal_fields(config, stored, map.pattern(), head)?;
                for field in [
                    "adapter_id",
                    "strategy",
                    "status",
                    "solver_completed",
                    "scalar_recovered_without_witness",
                    "initial_checked",
                    "pattern",
                    "patterns_total",
                    "patterns_checked",
                    "first_stop",
                    "checkpoint_sequence",
                    "checkpoint_sha256",
                ] {
                    if stored.get(field) != outcome.get(field) {
                        return Err("manifest/worker terminal SAT fields differ".into());
                    }
                }
            }
            // Pin the joined terminal to the same capture endpoint once more;
            // retain bytes even on mutation or parse refusal.
            self.frame.terminal_head_raw = read(
                dir,
                "live/committed.json",
                32768,
                &mut self.frame.budget,
                &mut self.frame.pending_read,
            )?;
            let n = self.frame.terminal_head_raw.len() as u64;
            self.frame
                .budget
                .reserve_checking(
                    n.checked_mul(256).ok_or("terminal head work overflow")?,
                    n.checked_mul(128).ok_or("terminal head owner overflow")?,
                    Instant::now(),
                )
                .map_err(|_| "terminal head admission refused")?;
            let after: capture::CommittedHead =
                serde_json::from_slice(&self.frame.terminal_head_raw).map_err(|e| e.to_string())?;
            if &after != head {
                return Err("capture changed during terminal join".into());
            }
            DecodeBudget::check(&mut self.frame.budget, Instant::now())
                .map_err(|_| "terminal deadline refused")?;
            Ok(declared)
        })();
        match result {
            Ok(declared_initial) => Ok(TerminalVerified {
                capture: self,
                declared_initial,
            }),
            Err(error) => {
                self.frame.diagnostic = Some(error);
                let stop = self
                    .frame
                    .budget
                    .fail::<()>(Stop::NativeFailure)
                    .unwrap_err();
                Err(Failed {
                    frame: self.frame,
                    stop,
                })
            }
        }
    }
}

#[path = "sat_initial_read.rs"]
mod retained_read;
use retained_read::read;
fn admitted_head(raw: &[u8], budget: &mut ReplayBudget) -> Result<capture::CommittedHead, String> {
    let len = raw.len() as u64;
    budget
        .reserve_checking(
            len.checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .ok_or("head scan overflow")?,
            32768,
            Instant::now(),
        )
        .map_err(|_| "head scan refused")?;
    let shape = provenance::gate_wire::json_shape::inspect(raw, || {
        DecodeBudget::check(budget, Instant::now())
            .map_err(|_| provenance::gate_wire::json_shape::Error::Admission)
    })
    .map_err(|_| "head shape refused")?;
    budget
        .reserve_checking(
            len.checked_mul(256)
                .and_then(|n| n.checked_add(1024))
                .ok_or("head parse overflow")?,
            shape
                .owner_bytes()
                .map_err(|_| "head owner overflow")?
                .checked_mul(2)
                .ok_or("head owner overflow")?,
            Instant::now(),
        )
        .map_err(|_| "head owner admission refused")?;
    serde_json::from_slice(raw).map_err(|e| e.to_string())
}
fn capture_buffer_owners(head: &capture::CommittedHead, dir: &Path) -> Result<u64, String> {
    // Cumulative geometric raw line and pending-line buffers: four copies of
    // capacity growth each. Event hashes/path bookkeeping are charged separately.
    // Head parsing and bounded payload read/verification are separately admitted.
    let node = (16
        * (std::mem::size_of::<String>()
            + std::mem::size_of::<Artifact>()
            + std::mem::size_of::<u64>()
            + std::mem::size_of::<usize>())
        + 256) as u64;
    let path = (dir.as_os_str().len() as u64)
        .checked_add(256)
        .and_then(|n| n.checked_mul(512))
        .ok_or("capture path overflow")?;
    [
        head.byte_end.checked_mul(8),
        head.event_count.checked_mul(1024),
        head.payload_count.checked_mul(
            node.checked_add(4 * std::mem::size_of::<(u64, Artifact)>() as u64)
                .ok_or("payload owner overflow")?,
        ),
        Some(path),
        Some(65536),
    ]
    .into_iter()
    .try_fold(0u64, |sum, n| {
        sum.checked_add(n.ok_or("capture owner overflow")?)
            .ok_or("capture owner overflow")
    })
    .map_err(str::to_owned)
}
/// Historical verification uses LaunchContext::validate through execution::verify,
/// never execution::admit (which would inspect today's clock as producer entry).
fn verify_mode(
    dir: &Path,
    config: &CaseConfig,
    artifacts: &[Artifact],
    policy: Policy,
    row_mode: Option<RowReplay>,
    search_mode: Option<search::Replay>,
) -> Result<Outcome, Failed> {
    let mut frame = Frame {
        row_mode,
        search_mode,
        budget: ReplayBudget {
            policy,
            work: 0,
            bytes: 0,
            first_stop: None,
        },
        config: Vec::new(),
        public: Vec::new(),
        factor: Vec::new(),
        case: None,
        parts: DecodedParts::default(),
        map_bytes: Vec::new(),
        map: None,
        raw_map: None,
        transcript: None,
        checkpoint: None,
        pending_line: Vec::new(),
        head: None,
        diagnostic: None,
        visited: Endpoint::default(),
        math_refusal: None,
        math_plan: None,
        pending_read: None,
        head_raw: Vec::new(),
        checked_head_raw: Vec::new(),
        final_head_raw: Vec::new(),
        checked_head: None,
        payload_check_raw: Vec::new(),
        payload_check_complete: false,
        terminal_raw: Vec::new(),
        terminal: None,
        terminal_head_raw: Vec::new(),
        original_capture: None,
        revalidation_raw: Vec::new(),
        revalidation_payload: Vec::new(),
        revalidation_head: None,
        revalidation_config: None,
        artifact_fingerprint: None,
        measurement_policy_hash: None,
    };
    // No reporting allocation is permitted until this fixed reservation succeeds.
    if let Err(stop) = frame.budget.reserve_checking(65536, 65536, Instant::now()) {
        return Err(Failed { frame, stop });
    }
    let result = run(dir, config, artifacts, &mut frame);
    match result {
        Ok(true) => Ok(Outcome::Verified(VerifiedInitialCapture { frame })),
        Ok(false) => Ok(Outcome::IncompleteMissingProducerLedger(frame)),
        Err(error) => {
            frame.diagnostic = Some(error);
            let unknown = matches!(
                frame.math_refusal,
                Some(
                    original_math::Error::UnsupportedPolicy
                        | original_math::Error::ActiveTally
                        | original_math::Error::UnsupportedMembership
                        | original_math::Error::UnsupportedCertificate
                )
            );
            let stop = frame
                .budget
                .fail::<()>(if unknown {
                    Stop::InvalidLimits
                } else {
                    Stop::NativeFailure
                })
                .unwrap_err();
            if unknown {
                Ok(Outcome::UnknownOriginalMath(frame))
            } else {
                Err(Failed { frame, stop })
            }
        }
    }
}
fn run(
    dir: &Path,
    config: &CaseConfig,
    artifacts: &[Artifact],
    f: &mut Frame,
) -> Result<bool, String> {
    // Fixed report/join memory is reserved before helpers can produce Strings.

    if !profile(&config.family, config.n, config.m) || config.solver != "sat" {
        return Err("unsupported original SAT profile".into());
    }
    if artifacts.len() > 4096 {
        return Err("artifact count cap".into());
    }
    let artifact_bytes = artifacts.iter().try_fold(0u64, |n, a| {
        n.checked_add(a.bytes).ok_or("artifact byte overflow")
    })?;
    f.budget
        .reserve_checking(
            artifact_bytes
                .checked_mul(64)
                .ok_or("manifest quote overflow")?,
            2 * 1024 * 1024,
            Instant::now(),
        )
        .map_err(|_| "manifest admission refused".to_owned())?;
    storage::verify_manifest(dir, artifacts)?;
    execution::verify(dir, artifacts)?;
    crate::ml_dataset::verify_capture_context(dir, artifacts)?;
    let launch = execution::load(dir)?.ok_or("missing historical launch")?;
    let (binding, caps) =
        crate::ml_dataset::admit_capture_context(dir)?.ok_or("missing capture binding")?;
    if launch.binding != binding {
        return Err("launch/capture mismatch".into());
    }
    f.budget
        .reserve_checking(8192, 16384, Instant::now())
        .map_err(|_| "original context owner admission refused")?;
    f.original_capture = Some((binding.clone(), caps.clone()));
    f.artifact_fingerprint = Some(hash_artifact_descriptors(artifacts, &mut f.budget)?);
    f.revalidation_raw = read(
        dir,
        "measurement-policy.json",
        65536,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    f.measurement_policy_hash = Some(Sha256::digest(&f.revalidation_raw).into());
    f.config = read(
        dir,
        "config.json",
        65536,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    if serde_json::from_slice::<CaseConfig>(&f.config).map_err(|e| e.to_string())? != *config {
        return Err("saved config mismatch".into());
    }
    f.public = read(
        dir,
        "public.json",
        65536,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    f.factor = read(
        dir,
        "factor-base.json",
        131072,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    for (path, role, raw) in [
        ("public.json", "public-instance", f.public.as_slice()),
        ("factor-base.json", "factor-base", f.factor.as_slice()),
    ] {
        let mut joined = artifacts
            .iter()
            .filter(|a| a.path == path || a.role == role);
        let a = joined.next().ok_or("missing original artifact")?;
        if joined.next().is_some()
            || a.path != path
            || a.role != role
            || a.encoding != "json"
            || a.bytes != raw.len() as u64
            || a.sha256 != hex::encode(Sha256::digest(raw))
        {
            return Err("original artifact identity mismatch".into());
        }
    }
    let total = f
        .public
        .len()
        .checked_add(f.factor.len())
        .ok_or("original length overflow")? as u64;
    f.budget
        .reserve_checking(
            total.checked_mul(64).ok_or("original parse overflow")?,
            total.checked_mul(128).ok_or("original parse overflow")?,
            Instant::now(),
        )
        .map_err(|_| "original parse admission refused".to_owned())?;
    f.case = Some(CaseData {
        public_instance: serde_json::from_slice(&f.public).map_err(|e| e.to_string())?,
        factor_base: serde_json::from_slice(&f.factor).map_err(|e| e.to_string())?,
        private_witness: Value::Null,
    });
    let case = f.case.as_ref().unwrap();
    // Pay the allocation-free structural scan before inspecting candidate/cert
    // arrays. The plan is independent of saved counters and acceptance labels.
    f.budget
        .reserve_checking(
            total.checked_mul(8).ok_or("math shape scan overflow")?,
            4096,
            Instant::now(),
        )
        .map_err(|_| "math shape scan refused".to_owned())?;
    let plan = match original_math::plan(
        config,
        &case.public_instance,
        &case.factor_base,
        f.budget.policy.deadline,
    ) {
        Ok(p) => p,
        Err(e) => {
            f.math_refusal = Some(e);
            return Err("original math shape is not admitted".into());
        }
    };
    f.math_plan = Some(plan);
    let plan = f.math_plan.as_ref().unwrap();
    f.budget
        .reserve_checking(plan.work, plan.bytes, Instant::now())
        .map_err(|_| "public mathematics admission refused".to_owned())?;
    // The bounded v2 verifier itself checks public mathematics exactly once;
    // do not redundantly run verify_public ahead of its mandatory verification.
    let result = crate::ml_dataset::curves::verify_factor_base_config_bounded(
        config,
        &case.public_instance,
        &case.factor_base,
        plan.bounds,
    );
    DecodeBudget::check(&mut f.budget, Instant::now())
        .map_err(|_| "public mathematics deadline refused".to_owned())?;
    result?;
    let public_sha256: [u8; 32] = Sha256::digest(&f.public).into();
    let factor_sha256: [u8; 32] = Sha256::digest(&f.factor).into();
    // Read-only token mint after mathematical verification and exact file custody.
    // Never calls verify_and_store_worker_originals or reads a scalar witness.
    let token = VerifiedWorkerOriginal {
        config,
        public: &case.public_instance,
        factor_base: &case.factor_base,
        root: dir,
        binding: Some(&binding),
        public_sha256,
        factor_base_sha256: factor_sha256,
        public_bytes: f.public.len() as u64,
        factor_base_bytes: f.factor.len() as u64,
    };
    decode_into(&token, &mut f.budget, &mut f.parts)
        .map_err(|_| "original decode refused".to_owned())?;
    let mut hash = Sha256::new();
    hash.update(b"ml-dataset.sat-original-owner/v1\0");
    hash.update(binding.digest()?.as_bytes());
    hash.update(public_sha256);
    hash.update(factor_sha256);
    let original_binding: [u8; 32] = hash.finalize().into();
    let mut launch_hasher = ReplayLaunchHash {
        hash: Sha256::new(),
        bytes: 0,
    };
    serde_json::to_writer(&mut launch_hasher, &launch).map_err(|e| e.to_string())?;
    let launch_hash = hex::encode(launch_hasher.hash.finalize());
    f.head_raw = read(
        dir,
        "live/committed.json",
        caps.metadata_reserve_bytes,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    let head = admitted_head(&f.head_raw, &mut f.budget)?;
    if head.byte_end > f.budget.policy.stream_bytes
        || head.payload_bytes > f.budget.policy.payload_bytes
    {
        return Err("replay stream/payload cap".into());
    }
    if head.payload_count != if f.row_mode.is_some() {2} else {1} {
        return Err("initial capture requires exactly one original map payload".into());
    }
    f.head = Some(head.clone());
    f.budget
        .reserve_checking(
            head.byte_end
                .checked_add(head.payload_bytes)
                .and_then(|n| n.checked_mul(64))
                .ok_or("capture quote overflow")?,
            // Retain the inherited aggregate floor until generic container
            // ownership and every hidden traversal owner have independent review.
            capture_buffer_owners(&head, dir)?.max(
                head.byte_end
                    .checked_mul(128)
                    .and_then(|n| n.checked_add(2 * 1024 * 1024))
                    .ok_or("capture aggregate overflow")?,
            ),
            Instant::now(),
        )
        .map_err(|_| "capture admission refused".to_owned())?;
    let binding_hash = binding.digest()?;
    let mut marker = None;
    let mut checkpoint_receipt = None;
    let authenticated =
        capture::verify_live_committed_prefix_owned(dir, &binding, &caps, &head, &mut |request| {
            let (phase, line) = match request {
                capture::OwnedReplayRequest::Head(_) => {
                    if f.checked_head.is_some() {
                        return Err("head owner cannot be replaced".into());
                    }
                    f.checked_head_raw = read(
                        dir,
                        "live/committed.json",
                        caps.metadata_reserve_bytes,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    f.checked_head = Some(admitted_head(&f.checked_head_raw, &mut f.budget)?);
                    return Ok(capture::OwnedReplayReply::Head(
                        f.checked_head.as_ref().unwrap().clone(),
                    ));
                }
                capture::OwnedReplayRequest::Payload(_, artifact) => {
                    if f.payload_check_complete {
                        let r=f.row_mode.as_mut().ok_or("payload verifier owner cannot be replaced")?;
                        if r.payload_checked || !r.payload_raw.is_empty(){return Err("row payload owner cannot be replaced".into())}
                        let path=format!("live/{}",artifact.path);
                        r.payload_raw=read(dir,&path,artifact.bytes,&mut f.budget,&mut f.pending_read)?;
                        DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"row payload hash deadline")?;
                        let verified=capture::verify_replay_payload_bytes(artifact,&r.payload_raw)?;
                        DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"row payload hash deadline")?;
                        r.payload_checked=true;
                        return Ok(capture::OwnedReplayReply::Payload(verified))
                    }
                    if !f.payload_check_raw.is_empty(){return Err("payload verifier owner cannot be replaced".into())}
                    let path = format!("live/{}", artifact.path);
                    // Actual-length descriptor read enforces declared byte cap plus
                    // a prepaid sentinel; no unbounded hash_file EOF traversal.
                    f.payload_check_raw = read(
                        dir,
                        &path,
                        artifact.bytes,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    DecodeBudget::check(&mut f.budget, Instant::now())
                        .map_err(|_| "payload hash deadline")?;
                    let verified =
                        capture::verify_replay_payload_bytes(artifact, &f.payload_check_raw)?;
                    DecodeBudget::check(&mut f.budget, Instant::now())
                        .map_err(|_| "payload hash deadline")?;
                    f.payload_check_complete = true;
                    return Ok(capture::OwnedReplayReply::Payload(verified));
                }
                capture::OwnedReplayRequest::Record(phase, line) => (phase, line),
            };
            if phase == capture::RecordPhase::BeforeRecordDecode {
                f.visited
                    .admit_line(line.len(), &head)
                    .map_err(|_| "visited prefix exceeds admitted head")?;
                // Preserve exact pending raw line under the prior whole-stream reserve.
                f.pending_line.clear();
                f.pending_line.extend_from_slice(line);
                f.budget
                    .reserve_checking(
                        (line.len() as u64)
                            .checked_mul(8)
                            .and_then(|v| v.checked_add(1024))
                            .ok_or("shape work overflow")?,
                        32768,
                        Instant::now(),
                    )
                    .map_err(|_| "capture shape admission refused")?;
                let shape = provenance::gate_wire::json_shape::inspect(line, || {
                    DecodeBudget::check(&mut f.budget, Instant::now())
                        .map_err(|_| provenance::gate_wire::json_shape::Error::Admission)
                })
                .map_err(|_| "capture shape refused")?;
                // Both capture check_record and this visitor deserialize. Additional
                // payload clone/typed conversion is covered by four materializations.
                let owners = shape
                    .owner_bytes()
                    .map_err(|_| "capture shape overflow")?
                    .checked_mul(4)
                    .ok_or("capture owner overflow")?;
                f.budget
                    .reserve_checking(0, owners, Instant::now())
                    .map_err(|_| "capture typed owner admission refused")?;
                return Ok(capture::OwnedReplayReply::Done);
            }
            let row: capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| e.to_string())?;
            let digest = hex::encode(Sha256::digest(line));
            f.visited.last_hash = Some(digest.clone());
            if row.event["kind"] == "capture_payload_committed" && row.event["artifact"]["role"] == row_wire::ROLE {
                let r=f.row_mode.as_mut().ok_or("row receipt in initial-only capture")?;
                if r.stored.is_some() || f.map.is_none(){return Err("row receipt order".into())}
                let artifact:Artifact=serde_json::from_value(row.event["artifact"].clone()).map_err(|e|e.to_string())?;
                if artifact.encoding!="json" || artifact.bytes>f.budget.policy.payload_bytes{return Err("row artifact cap/encoding".into())}
                f.visited.payload(artifact.bytes,&head).map_err(|_|"row payload exceeds admitted head")?;
                let path=format!("live/{}",artifact.path);
                r.raw=read(dir,&path,artifact.bytes,&mut f.budget,&mut f.pending_read)?;
                if r.raw.len() as u64!=artifact.bytes || hex::encode(Sha256::digest(&r.raw))!=artifact.sha256{return Err("row artifact digest".into())}
                f.budget.reserve_checking(artifact.bytes.checked_mul(256).ok_or("row decode work overflow")?,artifact.bytes.checked_mul(128).ok_or("row decode memory overflow")?,Instant::now()).map_err(|_|"row decode admission")?;
                r.stored=Some(serde_json::from_slice(&r.raw).map_err(|e|e.to_string())?);
                r.receipt=Some((row.sequence,digest));
                r.artifact=Some(artifact);
            } else if row.event["kind"] == "capture_payload_committed" {
                if f.map.is_some() || f.transcript.is_some() {
                    return Err("duplicate/interleaved initial map".into());
                }
                let a: Artifact = serde_json::from_value(row.event["artifact"].clone())
                    .map_err(|e| e.to_string())?;
                if a.role != "sat-original-gate-map"
                    || a.encoding != "json"
                    || a.bytes > f.budget.policy.payload_bytes
                {
                    return Err("map artifact mismatch".into());
                }
                f.visited
                    .payload(a.bytes, &head)
                    .map_err(|_| "payload exceeds admitted head")?;
                let path = format!("live/{}", a.path);
                f.map_bytes = read(dir, &path, a.bytes, &mut f.budget, &mut f.pending_read)?;
                if f.map_bytes.len() as u64 != a.bytes
                    || hex::encode(Sha256::digest(&f.map_bytes)) != a.sha256
                {
                    return Err("map hash mismatch".into());
                }
                let m = f.parts.complete.as_ref().unwrap();
                // Canonical basename supplies only the expected pattern. The
                // independent typed map verifier authenticates its actual identity.
                let (pattern, gate_wire) =
                    map_path_pattern(&a.path, config.m).map_err(|_| "map path/pattern mismatch")?;
                let original = provenance::Original {
                    binding: original_binding,
                    a: &m.a,
                    b: &m.b,
                    irr: &m.irr,
                    twist: config.twist,
                    m: config.m,
                    target: m.target.as_ref(),
                    pattern,
                    codes: &m.codes,
                };
                let check = provenance::Limits {
                    variables: 1_000_000,
                    codes: m.codes.len(),
                    domain_clauses: 1_000_000,
                    work: f.budget.policy.work,
                    bytes: f.budget.policy.bytes,
                };
                if gate_wire {
                    let decoded = provenance::gate_wire::decode_checked(
                        &f.map_bytes,
                        &original,
                        provenance::gate_wire::DecodeLimits {
                            wire_bytes: usize::try_from(a.bytes)
                                .map_err(|_| "map usize overflow")?,
                            work: f.budget.policy.work,
                            bytes: f.budget.policy.bytes,
                        },
                        check,
                        &mut f.budget,
                    );
                    match decoded {
                        Ok(map) => f.map = Some(map),
                        Err(e) => {
                            f.raw_map = e.retained;
                            return Err("imported gate map refused".into());
                        }
                    }
                } else {
                    // Preserve the full-map typed parse's existing prepaid bound.
                    f.budget
                        .reserve_checking(
                            a.bytes.checked_mul(256).ok_or("map parse overflow")?,
                            a.bytes.checked_mul(128).ok_or("map parse overflow")?,
                            Instant::now(),
                        )
                        .map_err(|_| "full map parse admission refused".to_owned())?;
                    // Historical full-map v1 is independently checked, never reinterpreted.
                    let raw: provenance::Map =
                        serde_json::from_slice(&f.map_bytes).map_err(|e| e.to_string())?;
                    match provenance::check_imported(&original, raw, check, &mut f.budget) {
                        Ok(map) => f.map = Some(map),
                        Err(e) => {
                            f.raw_map = e.retained;
                            return Err("imported full map refused".into());
                        }
                    }
                }
                if let Some(r)=f.row_mode.as_mut(){
                    r.map_receipt=Some((row.sequence,digest.clone()));
                    r.map_artifact=Some(a);
                } else {
                f.transcript = Some(
                    transcript::Transcript::new(
                        transcript::Expected {
                            binding_sha256: &binding_hash,
                            original_binding,
                            pattern,
                            map_sequence: row.sequence,
                            map_event_sha256: &digest,
                        },
                        transcript::Limits {
                            line_bytes: caps.max_event_bytes as usize,
                            stream_bytes: caps.max_stream_bytes,
                            events: 1_000_000,
                            batches: 1_000_000,
                            decoded_bytes: 64 * 1024 * 1024,
                            decoded_slots: 8_000_000,
                            decoded_words: 8_000_000,
                        },
                        &mut f.budget,
                    )
                    .map_err(|_| "transcript admission refused")?,
                );
                }
            } else if row.event["kind"] == "binary_sat_initial" {
                if f.row_mode.is_some(){return Err("initial event in row-only capture".into())}
                if row.event["payload"]["schema"] == ledger::SCHEMA {
                    if f.checkpoint.is_some() || marker.is_none() {
                        return Err("ledger order mismatch".into());
                    }
                    f.checkpoint = Some(
                        serde_json::from_value(row.event["payload"].clone())
                            .map_err(|e| e.to_string())?,
                    );
                    checkpoint_receipt = Some((row.sequence, digest));
                } else {
                    if f.checkpoint.is_some() {
                        return Err("initial event after ledger".into());
                    }
                    let t = f.transcript.as_mut().ok_or("initial before original map")?;
                    t.feed(line, &mut f.budget)
                        .map_err(|_| "initial transcript refused")?;
                    if let Some((sequence, hash)) = t.marker_position() {
                        marker = Some((sequence, hash.to_owned()));
                    }
                }
            } else if row.event["kind"] == "binary_sat_search" {
                let replay = f.search_mode.as_mut().ok_or("search event in initial-only mode")?;
                let checkpoint = checkpoint_receipt.as_ref().ok_or("search before initial ledger")?;
                let map = f.map.as_ref().ok_or("search before checked map")?;
                let math = f.parts.complete.as_ref().ok_or("search before original math")?;
                replay.feed(line, &binding_hash, original_binding, map, &math.signed,
                    math.signed_target, checkpoint, &mut f.budget)?;
            } else {
                return Err("unsupported event in initial-only capture".into());
            }
            Ok(capture::OwnedReplayReply::Done)
        })?
        .ok_or("missing authenticated capture")?;
    f.visited
        .exact(&head)
        .map_err(|_| "visited endpoint differs from admitted head")?;
    if authenticated.payloads.len() != if f.row_mode.is_some(){2}else{1} {
        return Err("initial map custody count mismatch".into());
    }
    let complete = if f.row_mode.is_some() {
        verify_row_math(f,config,original_binding,&launch_hash,[config.work_limit.checked_mul(1024).ok_or("work overflow")?.min(16_777_216),launch.memory_bytes,launch.trace_bytes,config.work_limit])?;
        true
    } else {
    let map = f.map.as_ref().ok_or("no original map")?;
    f.transcript
        .as_ref()
        .ok_or("no initial transcript")?
        .verify(
            map,
            provenance::initial::Limits {
                work: f.budget.policy.work,
                bytes: f.budget.policy.bytes,
                events: 1_000_000,
                constraints: 1_000_000,
            },
            &mut f.budget,
        )
        .map_err(|_| "initial mathematics refused")?;
    let complete = if let Some(c) = &f.checkpoint {
        let marker = marker.as_ref().ok_or("no marker")?;
        let receipt = checkpoint_receipt.as_ref().ok_or("no ledger receipt")?;
        let limits = [
            config
                .work_limit
                .checked_mul(1024)
                .ok_or("work overflow")?
                .min(16_777_216),
            launch.memory_bytes,
            launch.trace_bytes,
            config.work_limit,
        ];
        ledger::verify(
            c,
            &ledger::Bound {
                original_binding,
                pattern: map.pattern(),
                launch_sha256: &launch_hash,
                limits,
                owner_ordinal: 0,
                covered: ledger::Receipt {
                    sequence: marker.0,
                    hash: &marker.1,
                },
            },
            ledger::Receipt {
                sequence: receipt.0,
                hash: &receipt.1,
            },
            None,
        )
        .map_err(|_| "producer ledger inconsistent")?;
        if c.first_stop.is_some() {
            return Err("stopped producer cannot mint completed initial receipt".into());
        }
        true
    } else {
        false
    };
    complete
    };
    let complete = if let Some(search) = f.search_mode.as_ref() {
        if complete {
            search.finish(f.checkpoint.as_ref().ok_or("search lacks initial ledger")?, &head)?
        } else { false }
    } else { complete };
    // Visitor effects remain provisional until every immutable join is repeated.
    f.budget
        .reserve_checking(
            artifact_bytes
                .checked_mul(64)
                .ok_or("final join quote overflow")?,
            2 * 1024 * 1024,
            Instant::now(),
        )
        .map_err(|_| "final join admission refused".to_owned())?;
    storage::verify_manifest(dir, artifacts)?;
    execution::verify(dir, artifacts)?;
    crate::ml_dataset::verify_capture_context(dir, artifacts)?;
    let final_capture =
        crate::ml_dataset::admit_capture_context(dir)?.ok_or("capture context disappeared")?;
    if final_capture.0 != binding || final_capture.1 != caps {
        return Err("capture context changed during replay".into());
    }
    if capture::AttemptBinding::from_cell_files(dir)? != binding
        || execution::load(dir)?.as_ref() != Some(&launch)
    {
        return Err("original context changed during replay".into());
    }
    f.final_head_raw = read(
        dir,
        "live/committed.json",
        caps.metadata_reserve_bytes,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    if f.final_head_raw != f.head_raw {
        return Err("capture head changed during replay".into());
    }
    DecodeBudget::check(&mut f.budget, Instant::now()).map_err(|_| "replay deadline refused")?;
    Ok(complete)
}

// Covered by fixed 65536 join quote: at most 2048 canonical bytes, no Vec.
struct ReplayLaunchHash {
    hash: Sha256,
    bytes: usize,
}
impl std::io::Write for ReplayLaunchHash {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self
            .bytes
            .checked_add(bytes.len())
            .filter(|n| *n <= 2048)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        self.hash.update(bytes);
        self.bytes = next;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn admitted_replay_refusal_is_cumulative_and_sticky() {
        let mut b = ReplayBudget {
            policy: Policy {
                work: 20,
                bytes: 20,
                deadline: Instant::now() + std::time::Duration::from_secs(30),
                stream_bytes: 100,
                payload_bytes: 100,
            },
            work: 0,
            bytes: 0,
            first_stop: None,
        };
        b.reserve_checking(11, 7, Instant::now()).unwrap();
        assert_eq!(b.reserve_checking(10, 1, Instant::now()), Err(Stop::Work));
        assert_eq!((b.work, b.bytes), (11, 7));
        assert_eq!(b.fail::<()>(Stop::NativeFailure), Err(Stop::Work));
        assert_eq!(b.reserve_checking(0, 0, Instant::now()), Err(Stop::Work));
    }
    #[test]
    fn launch_canonical_hash_has_a_real_hard_cap() {
        use std::io::Write;
        let mut h = ReplayLaunchHash {
            hash: Sha256::new(),
            bytes: 0,
        };
        h.write_all(&[0; 2048]).unwrap();
        assert!(h.write_all(&[1]).is_err());
        assert_eq!(h.bytes, 2048);
    }
    #[test]
    fn requested_profile_persisted_initial_and_ledger_require_real_success() {
        use crate::ml_dataset::{solvers, telemetry};
        for twist in [false, true] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, twist, "sat",
            );
            execution::create(&f.root).unwrap();
            let mut artifacts = Vec::new();
            let case = telemetry::measured(&f.root, &mut artifacts, "curve-construction", || {
                crate::ml_dataset::curves::generate(&f.config)
            })
            .unwrap();
            let token = solvers::verify_and_store_worker_originals(
                &f.config,
                &case,
                &f.root,
                Some(&f.binding),
                &mut artifacts,
            )
            .unwrap();
            let context = execution::admit(&f.root).unwrap().unwrap();
            let (_, caps) = crate::ml_dataset::admit_capture_context(&f.root)
                .unwrap()
                .unwrap();
            let mut sink = capture::CaptureEmitter::create(
                &f.root.join("live"),
                f.binding.clone(),
                caps.clone(),
            )
            .unwrap();
            let mut owner = match OriginalOwner::admit(&token, &case, &context, &sink, &caps) {
                Ok(o) => o,
                Err(_) => panic!("required original owner"),
            };
            let limits = provenance::Limits {
                variables: 100_000,
                codes: owner.math.codes.len(),
                domain_clauses: 1_000_000,
                work: owner.budget.native_work_limit,
                bytes: owner.budget.memory_limit,
            };
            owner.build_map(0, limits).expect("required original map");
            owner
                .persist_map(&mut sink)
                .expect("required original map publication");
            let limits = provenance::initial::Limits {
                work: owner.budget.native_work_limit,
                bytes: owner.budget.memory_limit,
                events: 100_000,
                constraints: 1_000_000,
            };
            let stage =
                match super::super::initial_native::NativeInitial::prepare(owner, sink, limits) {
                    Ok(s) => s,
                    Err(_) => panic!("required native initial"),
                };
            let _retained_owner = match super::super::initial_native::checkpoint::finish(stage) {
                Ok(o) => o,
                Err(e) => panic!("required same-budget producer checkpoint: {:?}", e.stop),
            };
            for (path, role) in crate::ml_dataset::CAPTURE_CONTEXT
                .into_iter()
                .chain([(execution::PATH, execution::ROLE)])
            {
                let (sha256, bytes) = crate::store::hash_file(&f.root.join(path)).unwrap();
                artifacts.push(Artifact {
                    path: path.into(),
                    role: role.into(),
                    encoding: "json".into(),
                    sha256,
                    bytes,
                });
            }
            let head_before = std::fs::read(f.root.join("live/committed.json")).unwrap();
            // Test-only independent replay allowance. Producer caps and original
            // execution deadline above remain exactly the fixture's originals.
            let policy = Policy {
                work: 1_000_000_000_000,
                bytes: 8_000_000_000,
                deadline: Instant::now() + std::time::Duration::from_secs(120),
                stream_bytes: caps.max_stream_bytes,
                payload_bytes: caps.max_stream_bytes,
            };
            match verify(&f.root, &f.config, &artifacts, policy) {
                Ok(Outcome::Verified(seal)) => assert!(seal.initial_and_ledger_consistent()),
                Ok(Outcome::UnknownOriginalMath(_)) => {
                    panic!("required fixture mathematics supported")
                }
                Ok(Outcome::IncompleteMissingProducerLedger(_)) => {
                    panic!("required acknowledged producer ledger")
                }
                Err(f) => panic!(
                    "required historical replay: {:?} {:?}",
                    f.stop, f.frame.diagnostic
                ),
            }
            assert_eq!(
                head_before,
                std::fs::read(f.root.join("live/committed.json")).unwrap()
            );
        }
    }
}

/// Actual visitation state retained even when the provisional callback fails.
#[derive(Default)]
struct Endpoint {
    bytes: u64,
    events: u64,
    last_hash: Option<String>,
    payloads: u64,
    payload_bytes: u64,
}
impl Endpoint {
    fn admit_line(&mut self, len: usize, h: &capture::CommittedHead) -> Result<(), ()> {
        let bytes = self
            .bytes
            .checked_add(u64::try_from(len).map_err(|_| ())?)
            .filter(|v| *v <= h.byte_end)
            .ok_or(())?;
        let events = self
            .events
            .checked_add(1)
            .filter(|v| *v <= h.event_count)
            .ok_or(())?;
        self.bytes = bytes;
        self.events = events;
        Ok(())
    }
    fn payload(&mut self, len: u64, h: &capture::CommittedHead) -> Result<(), ()> {
        let count = self
            .payloads
            .checked_add(1)
            .filter(|v| *v <= h.payload_count)
            .ok_or(())?;
        let bytes = self
            .payload_bytes
            .checked_add(len)
            .filter(|v| *v <= h.payload_bytes)
            .ok_or(())?;
        self.payloads = count;
        self.payload_bytes = bytes;
        Ok(())
    }
    fn exact(&self, h: &capture::CommittedHead) -> Result<(), ()> {
        if self.bytes != h.byte_end
            || self.events != h.event_count
            || self.last_hash.as_deref() != Some(h.last_event_sha256.as_str())
            || self.payloads != h.payload_count
            || self.payload_bytes != h.payload_bytes
        {
            Err(())
        } else {
            Ok(())
        }
    }
}
#[cfg(test)]
mod endpoint_tests {
    use super::*;
    fn head() -> capture::CommittedHead {
        capture::CommittedHead {
            schema: "crypto.ml-dataset.capture-head/v1".into(),
            binding_sha256: "b".repeat(64),
            event_count: 2,
            byte_end: 30,
            last_event_sha256: "a".repeat(64),
            payload_count: 1,
            payload_bytes: 17,
            stop: None,
        }
    }
    fn endpoint() -> Endpoint {
        Endpoint {
            bytes: 30,
            events: 2,
            last_hash: Some("a".repeat(64)),
            payloads: 1,
            payload_bytes: 17,
        }
    }
    #[test]
    fn exact_endpoint_rejects_each_semantic_substitution() {
        let a = head();
        assert_eq!(endpoint().exact(&a), Ok(()));
        for field in 0..5 {
            let mut e = endpoint();
            match field {
                0 => e.bytes -= 1,
                1 => e.events -= 1,
                2 => e.last_hash = Some("c".repeat(64)),
                3 => e.payloads -= 1,
                _ => e.payload_bytes -= 1,
            }
            assert_eq!(e.exact(&a), Err(()));
        }
    }
    #[test]
    fn aba_restored_head_cannot_certify_visited_b() {
        let a = head();
        let mut b = a.clone();
        b.last_event_sha256 = "c".repeat(64);
        let mut observed = endpoint();
        observed.last_hash = Some(b.last_event_sha256.clone());
        assert_eq!(observed.exact(&b), Ok(()));
        let restored = a.clone();
        assert_eq!(restored, a);
        assert_eq!(observed.exact(&restored), Err(()));
    }
    #[test]
    fn refusal_precedes_pending_line_or_payload_materialization() {
        let a = head();
        let mut e = Endpoint::default();
        e.admit_line(20, &a).unwrap();
        assert_eq!(e.admit_line(11, &a), Err(()));
        assert_eq!((e.bytes, e.events), (20, 1));
        e.payload(17, &a).unwrap();
        assert_eq!(e.payload(1, &a), Err(()));
        assert_eq!((e.payloads, e.payload_bytes), (1, 17));
        e.admit_line(10, &a).unwrap();
        assert_eq!(e.admit_line(0, &a), Err(()));
    }
}

fn map_path_pattern(path: &str, m: usize) -> Result<(u32, bool), ()> {
    if !(2..=15).contains(&m) {
        return Err(());
    }
    let name = path
        .strip_prefix("payloads/sat-original-pattern-")
        .ok_or(())?;
    let (digits, gate) = if let Some(p) = name.strip_suffix(".gate.json") {
        (p, true)
    } else {
        (name.strip_suffix(".json").ok_or(())?, false)
    };
    if digits.is_empty()
        || !digits.bytes().all(|b| b.is_ascii_digit())
        || (digits.len() > 1 && digits.starts_with('0'))
    {
        return Err(());
    }
    let pattern = digits.parse::<u32>().map_err(|_| ())?;
    if pattern >= 1u32 << (m - 2) {
        return Err(());
    }
    Ok((pattern, gate))
}
#[cfg(test)]
mod map_path_tests {
    use super::*;
    #[test]
    fn canonical_pattern_is_only_expectation_not_map_authority() {
        assert_eq!(
            map_path_pattern("payloads/sat-original-pattern-0.gate.json", 2),
            Ok((0, true))
        );
        assert_eq!(
            map_path_pattern("payloads/sat-original-pattern-8191.json", 15),
            Ok((8191, false))
        );
        for s in [
            "payloads/sat-original-pattern-01.gate.json",
            "payloads/sat-original-pattern-+1.gate.json",
            "payloads/sat-original-pattern-.gate.json",
            "payloads/sat-original-pattern-8192.json",
            "payloads/sat-original-pattern-4294967296.json",
            "sat-original-pattern-0.gate.json",
            "payloads/sat-original-pattern-0.gate.json/extra",
        ] {
            assert!(map_path_pattern(s, 15).is_err());
        }
        assert!(map_path_pattern("payloads/sat-original-pattern-1.gate.json", 2).is_err());
    }
}

impl TerminalVerified {
    pub(in crate::ml_dataset::solvers) fn usage(&self) -> (u64, u64) {
        self.capture.frame.usage()
    }
    /// Recheck every persisted input against the already checked original and
    /// exact capture chain. All costs accrue to this same retained replay owner.
    pub(in crate::ml_dataset::solvers) fn revalidate(
        &mut self,
        dir: &Path,
        config: &CaseConfig,
        outcome: &Value,
        artifacts: &[Artifact],
    ) -> Result<(), Stop> {
        let expected_declared = self.declared_initial;
        let f = &mut self.capture.frame;
        if let Some(stop) = f.budget.first_stop {
            return Err(stop);
        }
        let result = (|| -> Result<(), String> {
            f.budget
                .reserve_checking(8192, 16384, Instant::now())
                .map_err(|_| "revalidation report/context admission refused")?;
            if artifacts.len() > 4096 {
                return Err("revalidation artifact count".into());
            }
            let artifact_bytes = artifacts.iter().try_fold(0u64, |n, a| {
                n.checked_add(a.bytes)
                    .ok_or("revalidation artifact overflow")
            })?;
            let head = f.head.as_ref().ok_or("missing checked revalidation head")?;
            // Retain the whole aggregate floor. No optimistic owner discount.
            f.budget
                .reserve_checking(
                    head.byte_end
                        .checked_add(head.payload_bytes)
                        .and_then(|n| n.checked_mul(64))
                        .ok_or("revalidation capture work overflow")?,
                    head.byte_end
                        .checked_mul(128)
                        .and_then(|n| n.checked_add(2 * 1024 * 1024))
                        .ok_or("revalidation capture owner overflow")?,
                    Instant::now(),
                )
                .map_err(|_| "revalidation capture admission refused")?;
            let head = head.clone();
            let (binding, caps) = f
                .original_capture
                .as_ref()
                .ok_or("missing original capture context")?
                .clone();

            for pass in 0..2 {
                f.budget
                    .reserve_checking(
                        artifact_bytes
                            .checked_mul(64)
                            .ok_or("revalidation manifest work overflow")?,
                        2 * 1024 * 1024,
                        Instant::now(),
                    )
                    .map_err(|_| "revalidation manifest admission refused")?;
                if Some(hash_artifact_descriptors(artifacts, &mut f.budget)?)
                    != f.artifact_fingerprint
                {
                    return Err("revalidation artifact descriptors changed".into());
                }
                for a in artifacts {
                    f.revalidation_raw =
                        read(dir, &a.path, a.bytes, &mut f.budget, &mut f.pending_read)?;
                    if f.revalidation_raw.len() as u64 != a.bytes
                        || hex::encode(Sha256::digest(&f.revalidation_raw)) != a.sha256
                    {
                        return Err("revalidation artifact bytes changed".into());
                    }
                }
                // AttemptBinding records exact raw hashes, including formatting.
                // These paths may be omitted from small component-fixture manifests.
                for (path, expected) in [
                    ("run.json", &binding.run_sha256),
                    ("plan.json", &binding.plan_sha256),
                    ("protocol.json", &binding.protocol_sha256),
                    ("attempt.json", &binding.attempt_sha256),
                ] {
                    f.revalidation_raw = read(
                        dir,
                        path,
                        crate::ml_dataset::JSON_INPUT_MAX_BYTES,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    if hex::encode(Sha256::digest(&f.revalidation_raw)) != *expected {
                        return Err("revalidation bound context bytes changed".into());
                    }
                }
                f.revalidation_raw = read(
                    dir,
                    "measurement-policy.json",
                    65536,
                    &mut f.budget,
                    &mut f.pending_read,
                )?;
                if Some(<[u8; 32]>::from(Sha256::digest(&f.revalidation_raw)))
                    != f.measurement_policy_hash
                {
                    return Err("revalidation measurement policy changed".into());
                }
                for (path, expected) in [
                    ("config.json", f.config.as_slice()),
                    ("public.json", f.public.as_slice()),
                    ("factor-base.json", f.factor.as_slice()),
                ] {
                    f.revalidation_raw = read(
                        dir,
                        path,
                        expected.len() as u64,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    if f.revalidation_raw.as_slice() != expected {
                        return Err("revalidation original bytes changed".into());
                    }
                }
                let n = f.config.len() as u64;
                f.budget
                    .reserve_checking(
                        n.checked_mul(256)
                            .ok_or("revalidation config work overflow")?,
                        n.checked_mul(128)
                            .ok_or("revalidation config owner overflow")?,
                        Instant::now(),
                    )
                    .map_err(|_| "revalidation config admission refused")?;
                f.revalidation_config =
                    Some(serde_json::from_slice(&f.config).map_err(|e| e.to_string())?);
                if f.revalidation_config.as_ref() != Some(config) {
                    return Err("revalidation caller config differs".into());
                }
                let declared = terminal_fields(
                    config,
                    outcome,
                    f.map.as_ref().ok_or("missing checked map")?.pattern(),
                    &head,
                )?;
                if declared != expected_declared {
                    return Err("revalidation terminal declaration changed".into());
                }
                if declared {
                    let mut joined = artifacts
                        .iter()
                        .filter(|a| a.path == "result.json" || a.role == "solver-outcome");
                    let a = joined
                        .next()
                        .ok_or("revalidation missing terminal artifact")?;
                    if joined.next().is_some()
                        || a.path != "result.json"
                        || a.role != "solver-outcome"
                        || a.encoding != "json"
                        || a.bytes != f.terminal_raw.len() as u64
                        || a.sha256 != hex::encode(Sha256::digest(&f.terminal_raw))
                    {
                        return Err("revalidation terminal descriptor changed".into());
                    }
                    f.revalidation_raw = read(
                        dir,
                        "result.json",
                        a.bytes,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    if f.revalidation_raw != f.terminal_raw {
                        return Err("revalidation terminal bytes changed".into());
                    }
                    let stored = f.terminal.as_ref().ok_or("missing retained terminal")?;
                    for key in [
                        "adapter_id",
                        "strategy",
                        "status",
                        "solver_completed",
                        "scalar_recovered_without_witness",
                        "initial_checked",
                        "pattern",
                        "patterns_total",
                        "patterns_checked",
                        "first_stop",
                        "checkpoint_sequence",
                        "checkpoint_sha256",
                    ] {
                        if stored.get(key) != outcome.get(key) {
                            return Err("revalidation manifest terminal changed".into());
                        }
                    }
                }
                if pass == 0 {
                    let authenticated=capture::verify_live_committed_prefix_owned(dir,&binding,&caps,&head,&mut |request| {
                        match request {
                            capture::OwnedReplayRequest::Head(_) => {
                                f.revalidation_raw=read(dir,"live/committed.json",caps.metadata_reserve_bytes,&mut f.budget,&mut f.pending_read)?;
                                f.revalidation_head=Some(revalidation_head(&f.revalidation_raw,&mut f.budget)?);
                                Ok(capture::OwnedReplayReply::Head(f.revalidation_head.as_ref().unwrap().clone()))
                            }
                            capture::OwnedReplayRequest::Record(phase,line) => {
                                if phase==capture::RecordPhase::BeforeRecordDecode {
                                    f.pending_line.clear();f.pending_line.extend_from_slice(line);
                                    f.budget.reserve_checking((line.len() as u64).checked_mul(8).and_then(|n|n.checked_add(1024)).ok_or("revalidation scan overflow")?,32768,Instant::now())
                                        .map_err(|_|"revalidation scan refused")?;
                                    let shape=provenance::gate_wire::json_shape::inspect(line,||DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|provenance::gate_wire::json_shape::Error::Admission))
                                        .map_err(|_|"revalidation shape refused")?;
                                    f.budget.reserve_checking(0,shape.owner_bytes().map_err(|_|"revalidation owner overflow")?.checked_mul(4).ok_or("revalidation owner overflow")?,Instant::now())
                                        .map_err(|_|"revalidation record owner refused")?;
                                }
                                Ok(capture::OwnedReplayReply::Done)
                            }
                            capture::OwnedReplayRequest::Payload(_,a) => {
                                let path=format!("live/{}",a.path);
                                f.revalidation_payload=read(dir,&path,a.bytes,&mut f.budget,&mut f.pending_read)?;
                                DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"revalidation payload deadline")?;
                                let proof=capture::verify_replay_payload_bytes(a,&f.revalidation_payload)?;
                                DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"revalidation payload deadline")?;
                                Ok(capture::OwnedReplayReply::Payload(proof))
                            }
                        }
                    })?.ok_or("revalidation capture disappeared")?;
                    if authenticated.payloads.len() != 1 {
                        return Err("revalidation payload count changed".into());
                    }
                }
                f.revalidation_raw = read(
                    dir,
                    "live/committed.json",
                    caps.metadata_reserve_bytes,
                    &mut f.budget,
                    &mut f.pending_read,
                )?;
                f.revalidation_head = Some(revalidation_head(&f.revalidation_raw, &mut f.budget)?);
                if f.revalidation_head.as_ref() != Some(&head) {
                    return Err("revalidation capture endpoint changed".into());
                }
                DecodeBudget::check(&mut f.budget, Instant::now())
                    .map_err(|_| "revalidation deadline refused")?;
            }
            Ok(())
        })();
        match result {
            Ok(()) => Ok(()),
            Err(error) => {
                f.diagnostic = Some(error);
                f.budget.fail(Stop::NativeFailure)
            }
        }
    }
}
fn revalidation_head(
    raw: &[u8],
    budget: &mut ReplayBudget,
) -> Result<capture::CommittedHead, String> {
    let n = raw.len() as u64;
    budget
        .reserve_checking(
            n.checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .ok_or("revalidation head scan overflow")?,
            32768,
            Instant::now(),
        )
        .map_err(|_| "revalidation head scan refused")?;
    let shape = provenance::gate_wire::json_shape::inspect(raw, || {
        DecodeBudget::check(budget, Instant::now())
            .map_err(|_| provenance::gate_wire::json_shape::Error::Admission)
    })
    .map_err(|_| "revalidation head shape refused")?;
    budget
        .reserve_checking(
            n.checked_mul(256)
                .and_then(|n| n.checked_add(1024))
                .ok_or("revalidation head parse overflow")?,
            shape
                .owner_bytes()
                .map_err(|_| "revalidation head owner overflow")?
                .checked_mul(2)
                .ok_or("revalidation head owner overflow")?,
            Instant::now(),
        )
        .map_err(|_| "revalidation head admission refused")?;
    serde_json::from_slice(raw).map_err(|e| e.to_string())
}

fn hash_artifact_descriptors(
    artifacts: &[Artifact],
    budget: &mut ReplayBudget,
) -> Result<[u8; 32], String> {
    if artifacts.len() > 4096 {
        return Err("artifact fingerprint count".into());
    }
    let bound = artifacts.iter().try_fold(2u64, |n, a| {
        let chars = a
            .path
            .len()
            .checked_add(a.role.len())
            .and_then(|v| v.checked_add(a.encoding.len()))
            .and_then(|v| v.checked_add(a.sha256.len()))
            .ok_or("artifact fingerprint length overflow")? as u64;
        n.checked_add(
            chars
                .checked_mul(6)
                .and_then(|v| v.checked_add(128))
                .ok_or("artifact fingerprint bound overflow")?,
        )
        .ok_or("artifact fingerprint bound overflow")
    })?;
    budget
        .reserve_checking(
            bound
                .checked_mul(8)
                .ok_or("artifact fingerprint work overflow")?,
            0,
            Instant::now(),
        )
        .map_err(|_| "artifact fingerprint admission refused")?;
    struct HashWriter {
        hash: Sha256,
        remaining: u64,
    }
    impl std::io::Write for HashWriter {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.remaining = self
                .remaining
                .checked_sub(b.len() as u64)
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::WriteZero))?;
            self.hash.update(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = HashWriter {
        hash: Sha256::new(),
        remaining: bound,
    };
    serde_json::to_writer(&mut writer, artifacts).map_err(|e| e.to_string())?;
    DecodeBudget::check(budget, Instant::now()).map_err(|_| "artifact fingerprint deadline")?;
    Ok(writer.hash.finalize().into())
}

pub(in crate::ml_dataset::solvers) struct RowLimits {
    pub expansion: row_equations::Limits,
    pub checker: row_matrix::CheckerLimits,
}
struct RowReplay {
    limits:RowLimits,raw:Vec<u8>,stored:Option<row_wire::Stored>,
    artifact:Option<Artifact>,map_artifact:Option<Artifact>,
    receipt:Option<(u64,String)>,map_receipt:Option<(u64,String)>,
    payload_raw:Vec<u8>,payload_checked:bool,
    arithmetic:Option<row_matrix::OwnedReplay>,
}
impl RowReplay {
    fn new(limits:RowLimits)->Self{Self{limits,raw:Vec::new(),stored:None,artifact:None,map_artifact:None,receipt:None,map_receipt:None,payload_raw:Vec::new(),payload_checked:false,arithmetic:None}}
}
pub(in crate::ml_dataset::solvers) struct VerifiedRow {frame:Frame}
pub(in crate::ml_dataset::solvers) enum RowOutcome {Verified(VerifiedRow),UnknownOriginalMath(Frame),Incomplete(Frame)}
pub(in crate::ml_dataset::solvers) fn verify(dir:&Path,config:&CaseConfig,artifacts:&[Artifact],policy:Policy)->Result<Outcome,Failed>{verify_mode(dir,config,artifacts,policy,None,None)}
pub(in crate::ml_dataset::solvers) fn verify_boolean_row(dir:&Path,config:&CaseConfig,outcome:&Value,artifacts:&[Artifact],policy:Policy,limits:RowLimits)->Result<RowOutcome,Failed>{
    match verify_mode(dir,config,artifacts,policy,Some(RowReplay::new(limits)),None)? {
        Outcome::Verified(mut verified)=>{
            if let Err(error)=join_row_terminal(dir,outcome,artifacts,&mut verified.frame){
                verified.frame.diagnostic=Some(error);
                let stop=verified.frame.budget.fail::<()>(Stop::InvalidLimits).unwrap_err();
                return Err(Failed{frame:verified.frame,stop})
            }
            Ok(RowOutcome::Verified(VerifiedRow{frame:verified.frame}))
        },
        Outcome::UnknownOriginalMath(frame)=>Ok(RowOutcome::UnknownOriginalMath(frame)),
        Outcome::IncompleteMissingProducerLedger(frame)=>Ok(RowOutcome::Incomplete(frame)),
    }
}
impl VerifiedRow {
    pub(in crate::ml_dataset::solvers) fn usage(&self)->(u64,u64){self.frame.usage()}
    pub(in crate::ml_dataset::solvers) fn audit(&self)->&row_matrix::Audit{self.frame.row_mode.as_ref().unwrap().arithmetic.as_ref().unwrap().audit().unwrap()}
}
fn verify_row_math(f:&mut Frame,config:&CaseConfig,original_binding:[u8;32],launch_hash:&str,limits:[u64;4])->Result<(),String>{
    let r=f.row_mode.as_mut().ok_or("row mode missing")?;
    let stored=r.stored.as_mut().ok_or("row receipt missing")?;
    let m=&stored.metadata;
    let map=f.map.as_ref().ok_or("row original map missing")?;
    let map_artifact=r.map_artifact.as_ref().ok_or("map descriptor missing")?;
    let map_receipt=r.map_receipt.as_ref().ok_or("map receipt missing")?;
    let receipt=r.receipt.as_ref().ok_or("row event receipt missing")?;
    let artifact=r.artifact.as_ref().ok_or("row descriptor missing")?;
    if stored.schema!=row_wire::SCHEMA || m.original_binding!=original_binding || hex::encode(m.launch_sha256)!=launch_hash
        || m.pattern!=map.pattern() || m.pattern>=1u32.checked_shl((config.m-2) as u32).ok_or("pattern overflow")?
        ||m.window!=0||m.variables!=map.variables()||m.limits!=limits||m.phase>=6||m.row>=m.generators
        ||m.first_stop.is_some()||m.solver_completed||m.full_span_search||m.scalar_recovered
        ||artifact.path!=format!("payloads/{}",row_wire::path(m.pattern,m.row))
        ||m.map_sequence!=map_receipt.0||hex::encode(m.map_event_sha256)!=map_receipt.1
        ||receipt.0!=map_receipt.0.checked_add(1).ok_or("receipt overflow")?
        ||m.map_bytes!=map_artifact.bytes||hex::encode(m.map_sha256)!=map_artifact.sha256
        ||m.wire_bytes!=stored.wire.len() as u64||<[u8;32]>::from(Sha256::digest(stored.wire.as_bytes()))!=m.wire_sha256{
        return Err("row original/map/wire/receipt identity mismatch".into())
    }
    let t=m.totals;let l=m.limits;
    if t[0].checked_add(t[2]).filter(|n|*n<=l[0]).is_none()
        ||t[1].checked_add(t[4]).filter(|n|*n<=l[1]).is_none()
        ||t[5]>l[2]||t[6]>l[3]||t[3]>t[2]
        ||m.phase_native[0]>t[0]||m.phase_native[1]>t[1]
        ||m.phase_limits[0]>l[0]||m.phase_limits[1]>l[1]
        ||m.phase_native[0]>m.phase_limits[0]||m.phase_native[1]>m.phase_limits[1]{return Err("row producer ledger range mismatch".into())}
    // Check the known self-publication minimum, without claiming full native
    // cost chronology can be independently reconstructed from a checkpoint.
    let cap=row_wire::cap(stored.wire.as_bytes()).ok_or("row wire alphabet")?;
    let publication_min=cap.checked_mul(48).and_then(|v|v.checked_add(65536)).ok_or("row quote overflow")?;
    if t[2]<publication_min{return Err("row ledger omits known publication charge".into())}
    let map=f.map.take().unwrap();
    let expanded=row_equations::expand_owned(map,r.limits.expansion,&mut f.budget);
    let generators=expanded.equations().len() as u64;
    let complete=expanded.status().is_ok();
    let raw=std::mem::take(&mut stored.wire).into_bytes();
    r.arithmetic=Some(row_matrix::replay_owned(expanded,raw,r.limits.checker,f.budget.policy.deadline,&mut f.budget));
    if !complete||generators!=m.generators{return Err("row ordered generator expansion mismatch".into())}
    if r.arithmetic.as_ref().unwrap().failure().is_some(){return Err("row source-bound arithmetic refused".into())}
    Ok(())
}

fn join_row_terminal(dir:&Path,outcome:&Value,artifacts:&[Artifact],f:&mut Frame)->Result<(),String>{
    f.budget.reserve_checking(8192,4096,Instant::now()).map_err(|_|"row terminal admission")?;
    let mut matches=artifacts.iter().filter(|a|a.path=="result.json");
    let artifact=matches.next().ok_or("row result missing")?;
    if matches.next().is_some()||artifact.role!="solver-outcome"||artifact.encoding!="json"||artifact.bytes>row_wire::REPORT_CAP{return Err("row result descriptor".into())}
    f.terminal_raw=read(dir,"result.json",artifact.bytes,&mut f.budget,&mut f.pending_read)?;
    if f.terminal_raw.len() as u64!=artifact.bytes||hex::encode(Sha256::digest(&f.terminal_raw))!=artifact.sha256{return Err("row result digest".into())}
    f.budget.reserve_checking(artifact.bytes.checked_mul(256).ok_or("row result quote overflow")?,artifact.bytes.checked_mul(128).ok_or("row result quote overflow")?,Instant::now()).map_err(|_|"row result parse admission")?;
    f.terminal=Some(serde_json::from_slice(&f.terminal_raw).map_err(|e|e.to_string())?);
    if f.terminal.as_ref()!=Some(outcome){return Err("row result differs from caller manifest".into())}
    let row=f.row_mode.as_ref().ok_or("row seal missing")?;
    let m=&row.stored.as_ref().ok_or("row metadata missing")?.metadata;
    let receipt=row.receipt.as_ref().ok_or("row receipt missing")?;
    if outcome["adapter_id"]!="original-unit-boolean-row-v1"||outcome["strategy"]!="original-unit-boolean-row-v1"
        ||outcome["status"]!="partial"||outcome["finite_row_receipt_committed"]!=true
        ||outcome["solver_completed"]!=false||outcome["scalar_recovered_without_witness"]!=false
        ||outcome["ecdlp_pipeline_completed"]!=false||outcome["full_span_search"]!=false
        ||outcome["initial_checked"]==true||outcome["pattern"].as_u64()!=Some(m.pattern as u64)
        ||outcome["row"].as_u64()!=Some(m.row)||outcome["receipt_sequence"].as_u64()!=Some(receipt.0)
        ||outcome["receipt_sha256"].as_str()!=Some(receipt.1.as_str()){return Err("row terminal claim mismatch".into())}
    f.terminal_head_raw=read(dir,"live/committed.json",65536,&mut f.budget,&mut f.pending_read)?;
    if f.terminal_head_raw!=f.head_raw{return Err("row head changed during terminal join".into())}
    DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"row terminal deadline".to_owned())
}

// Distinct original-owned search result. No initial-only terminal/release API.
pub(in crate::ml_dataset::solvers) struct VerifiedSearch { frame: Frame }
pub(in crate::ml_dataset::solvers) enum SearchOutcome {
    Verified(VerifiedSearch), Incomplete(Frame), UnknownOriginalMath(Frame),
}
pub(in crate::ml_dataset::solvers) fn verify_search(
    dir: &Path, config: &CaseConfig, artifacts: &[Artifact], policy: Policy,
    event_limit: usize,
) -> Result<SearchOutcome, Failed> {
    match verify_mode(dir, config, artifacts, policy, None, Some(search::Replay::new(event_limit)))? {
        Outcome::Verified(v) => Ok(SearchOutcome::Verified(VerifiedSearch {frame:v.frame})),
        Outcome::IncompleteMissingProducerLedger(f) => Ok(SearchOutcome::Incomplete(f)),
        Outcome::UnknownOriginalMath(f) => Ok(SearchOutcome::UnknownOriginalMath(f)),
    }
}
impl VerifiedSearch {
    pub(in crate::ml_dataset::solvers) fn model_relation_checked(&self) -> bool {
        self.frame.budget.first_stop.is_none() && self.frame.search_mode.as_ref()
            .is_some_and(|s| s.model_relation_checked())
    }
    pub(in crate::ml_dataset::solvers) fn native_unsat_uncertified(&self) -> bool {
        self.frame.budget.first_stop.is_none() && self.frame.search_mode.as_ref()
            .is_some_and(|s| s.native_unsat_uncertified())
    }
}


// Report joins confer only producer-report/custody scope; no causal completion.
fn search_report_join(f:&mut Frame,dir:&Path,outcome:&Value,artifacts:&[Artifact])->Result<(),String>{
    f.budget.reserve_checking(65536,65536,Instant::now()).map_err(|_|"search report admission")?;
    let mut candidates=artifacts.iter().filter(|a|a.path=="result.json" || a.role=="solver-outcome");
    let a=candidates.next().ok_or("missing search result artifact")?;
    if candidates.next().is_some() || a.path!="result.json" || a.role!="solver-outcome" || a.encoding!="json" || a.bytes>4096 {
        return Err("search result descriptor".into())
    }
    f.terminal_raw=read(dir,"result.json",a.bytes,&mut f.budget,&mut f.pending_read)?;
    if f.terminal_raw.len() as u64!=a.bytes || hex::encode(Sha256::digest(&f.terminal_raw))!=a.sha256 {
        return Err("search result artifact bytes".into())
    }
    f.terminal=Some(serde_json::from_slice(&f.terminal_raw).map_err(|_|"search result decode")?);
    let stored=f.terminal.as_ref().unwrap();
    if stored!=outcome || stored["adapter_id"]!="sat-search-v1" || stored["strategy"]!="sat-search-v1"
        || stored["status"]!="partial" || stored["solver_completed"]!=false
        || stored["scalar_recovered_without_witness"]!=false || stored["all_patterns_negative"]!=false {
        return Err("search producer report identity or overclaim".into())
    }
    let head=f.head.as_ref().ok_or("search report missing authenticated head")?;
    if stored["terminal_sequence"].as_u64()!=Some(head.event_count)
        || stored["terminal_sha256"].as_str()!=Some(head.last_event_sha256.as_str()) {
        return Err("search report terminal acknowledgment".into())
    }
    f.search_mode.as_ref().ok_or("missing search mode")?.check_report(&stored["search_report"])?;
    let mut telemetry=artifacts.iter().filter(|a|a.path=="telemetry/solver.json");
    let t=telemetry.next().ok_or("search requires completed solver telemetry")?;
    if telemetry.next().is_some()||t.encoding!="json"||t.bytes>65536{return Err("search solver telemetry descriptor".into())}
    f.budget.reserve_checking(t.bytes.checked_mul(256).ok_or("telemetry quote")?,t.bytes.checked_mul(128).ok_or("telemetry quote")?,Instant::now()).map_err(|_|"search telemetry admission")?;
    f.terminal_head_raw=read(dir,&t.path,t.bytes,&mut f.budget,&mut f.pending_read)?;
    if f.terminal_head_raw.len() as u64!=t.bytes||hex::encode(Sha256::digest(&f.terminal_head_raw))!=t.sha256{return Err("search solver telemetry bytes".into())}
    let measured:Value=serde_json::from_slice(&f.terminal_head_raw).map_err(|_|"search solver telemetry parse")?;
    if measured["returned_ok"]!=true{return Err("search solver telemetry failed".into())}
    crate::ml_dataset::telemetry::checked::verify_artifacts(dir,artifacts)?;
    execution::verify(dir,artifacts)?;
    DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"search report post-join deadline".into())
}
impl VerifiedSearch {
    pub(in crate::ml_dataset::solvers) fn join_report(&mut self,dir:&Path,outcome:&Value,artifacts:&[Artifact])->Result<(),Stop>{
        match search_report_join(&mut self.frame,dir,outcome,artifacts) {
            Ok(())=>Ok(()),Err(error)=>{self.frame.diagnostic=Some(error);self.frame.budget.fail(Stop::NativeFailure)}
        }
    }
    pub(in crate::ml_dataset::solvers) fn project_restricted(&mut self)->Result<Value,Stop>{
        let f=&mut self.frame;
        f.budget.reserve_checking(65536,65536,Instant::now())?;
        if f.terminal.is_none(){return f.budget.fail(Stop::Phase)}
        let value=f.search_mode.as_ref().ok_or(Stop::Phase)?.restricted_projection();
        DecodeBudget::check(&mut f.budget,Instant::now())?;
        Ok(value)
    }
}

impl VerifiedSearch {
    pub(in crate::ml_dataset::solvers) fn usage(&self) -> (u64, u64) {
        self.frame.usage()
    }
    /// Recheck every persisted input against the already checked original and
    /// exact capture chain. All costs accrue to this same retained replay owner.
    pub(in crate::ml_dataset::solvers) fn revalidate(
        &mut self,
        dir: &Path,
        config: &CaseConfig,
        outcome: &Value,
        artifacts: &[Artifact],
    ) -> Result<(), Stop> {
        let f = &mut self.frame;
        if let Some(stop) = f.budget.first_stop {
            return Err(stop);
        }
        let result = (|| -> Result<(), String> {
            f.budget
                .reserve_checking(8192, 16384, Instant::now())
                .map_err(|_| "revalidation report/context admission refused")?;
            if artifacts.len() > 4096 {
                return Err("revalidation artifact count".into());
            }
            let artifact_bytes = artifacts.iter().try_fold(0u64, |n, a| {
                n.checked_add(a.bytes)
                    .ok_or("revalidation artifact overflow")
            })?;
            let head = f.head.as_ref().ok_or("missing checked revalidation head")?;
            // Retain the whole aggregate floor. No optimistic owner discount.
            f.budget
                .reserve_checking(
                    head.byte_end
                        .checked_add(head.payload_bytes)
                        .and_then(|n| n.checked_mul(64))
                        .ok_or("revalidation capture work overflow")?,
                    head.byte_end
                        .checked_mul(128)
                        .and_then(|n| n.checked_add(2 * 1024 * 1024))
                        .ok_or("revalidation capture owner overflow")?,
                    Instant::now(),
                )
                .map_err(|_| "revalidation capture admission refused")?;
            let head = head.clone();
            let (binding, caps) = f
                .original_capture
                .as_ref()
                .ok_or("missing original capture context")?
                .clone();

            for pass in 0..2 {
                f.budget
                    .reserve_checking(
                        artifact_bytes
                            .checked_mul(64)
                            .ok_or("revalidation manifest work overflow")?,
                        2 * 1024 * 1024,
                        Instant::now(),
                    )
                    .map_err(|_| "revalidation manifest admission refused")?;
                if Some(hash_artifact_descriptors(artifacts, &mut f.budget)?)
                    != f.artifact_fingerprint
                {
                    return Err("revalidation artifact descriptors changed".into());
                }
                for a in artifacts {
                    f.revalidation_raw =
                        read(dir, &a.path, a.bytes, &mut f.budget, &mut f.pending_read)?;
                    if f.revalidation_raw.len() as u64 != a.bytes
                        || hex::encode(Sha256::digest(&f.revalidation_raw)) != a.sha256
                    {
                        return Err("revalidation artifact bytes changed".into());
                    }
                }
                // AttemptBinding records exact raw hashes, including formatting.
                // These paths may be omitted from small component-fixture manifests.
                for (path, expected) in [
                    ("run.json", &binding.run_sha256),
                    ("plan.json", &binding.plan_sha256),
                    ("protocol.json", &binding.protocol_sha256),
                    ("attempt.json", &binding.attempt_sha256),
                ] {
                    f.revalidation_raw = read(
                        dir,
                        path,
                        crate::ml_dataset::JSON_INPUT_MAX_BYTES,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    if hex::encode(Sha256::digest(&f.revalidation_raw)) != *expected {
                        return Err("revalidation bound context bytes changed".into());
                    }
                }
                f.revalidation_raw = read(
                    dir,
                    "measurement-policy.json",
                    65536,
                    &mut f.budget,
                    &mut f.pending_read,
                )?;
                if Some(<[u8; 32]>::from(Sha256::digest(&f.revalidation_raw)))
                    != f.measurement_policy_hash
                {
                    return Err("revalidation measurement policy changed".into());
                }
                for (path, expected) in [
                    ("config.json", f.config.as_slice()),
                    ("public.json", f.public.as_slice()),
                    ("factor-base.json", f.factor.as_slice()),
                ] {
                    f.revalidation_raw = read(
                        dir,
                        path,
                        expected.len() as u64,
                        &mut f.budget,
                        &mut f.pending_read,
                    )?;
                    if f.revalidation_raw.as_slice() != expected {
                        return Err("revalidation original bytes changed".into());
                    }
                }
                let n = f.config.len() as u64;
                f.budget
                    .reserve_checking(
                        n.checked_mul(256)
                            .ok_or("revalidation config work overflow")?,
                        n.checked_mul(128)
                            .ok_or("revalidation config owner overflow")?,
                        Instant::now(),
                    )
                    .map_err(|_| "revalidation config admission refused")?;
                f.revalidation_config =
                    Some(serde_json::from_slice(&f.config).map_err(|e| e.to_string())?);
                if f.revalidation_config.as_ref() != Some(config) {
                    return Err("revalidation caller config differs".into());
                }
                search_report_join(f, dir, outcome, artifacts)?;
                if pass == 0 {
                    let authenticated=capture::verify_live_committed_prefix_owned(dir,&binding,&caps,&head,&mut |request| {
                        match request {
                            capture::OwnedReplayRequest::Head(_) => {
                                f.revalidation_raw=read(dir,"live/committed.json",caps.metadata_reserve_bytes,&mut f.budget,&mut f.pending_read)?;
                                f.revalidation_head=Some(revalidation_head(&f.revalidation_raw,&mut f.budget)?);
                                Ok(capture::OwnedReplayReply::Head(f.revalidation_head.as_ref().unwrap().clone()))
                            }
                            capture::OwnedReplayRequest::Record(phase,line) => {
                                if phase==capture::RecordPhase::BeforeRecordDecode {
                                    f.pending_line.clear();f.pending_line.extend_from_slice(line);
                                    f.budget.reserve_checking((line.len() as u64).checked_mul(8).and_then(|n|n.checked_add(1024)).ok_or("revalidation scan overflow")?,32768,Instant::now())
                                        .map_err(|_|"revalidation scan refused")?;
                                    let shape=provenance::gate_wire::json_shape::inspect(line,||DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|provenance::gate_wire::json_shape::Error::Admission))
                                        .map_err(|_|"revalidation shape refused")?;
                                    f.budget.reserve_checking(0,shape.owner_bytes().map_err(|_|"revalidation owner overflow")?.checked_mul(4).ok_or("revalidation owner overflow")?,Instant::now())
                                        .map_err(|_|"revalidation record owner refused")?;
                                }
                                Ok(capture::OwnedReplayReply::Done)
                            }
                            capture::OwnedReplayRequest::Payload(_,a) => {
                                let path=format!("live/{}",a.path);
                                f.revalidation_payload=read(dir,&path,a.bytes,&mut f.budget,&mut f.pending_read)?;
                                DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"revalidation payload deadline")?;
                                let proof=capture::verify_replay_payload_bytes(a,&f.revalidation_payload)?;
                                DecodeBudget::check(&mut f.budget,Instant::now()).map_err(|_|"revalidation payload deadline")?;
                                Ok(capture::OwnedReplayReply::Payload(proof))
                            }
                        }
                    })?.ok_or("revalidation capture disappeared")?;
                    if authenticated.payloads.len() != 1 {
                        return Err("revalidation payload count changed".into());
                    }
                }
                f.revalidation_raw = read(
                    dir,
                    "live/committed.json",
                    caps.metadata_reserve_bytes,
                    &mut f.budget,
                    &mut f.pending_read,
                )?;
                f.revalidation_head = Some(revalidation_head(&f.revalidation_raw, &mut f.budget)?);
                if f.revalidation_head.as_ref() != Some(&head) {
                    return Err("revalidation capture endpoint changed".into());
                }
                DecodeBudget::check(&mut f.budget, Instant::now())
                    .map_err(|_| "revalidation deadline refused")?;
            }
            Ok(())
        })();
        match result {
            Ok(()) => Ok(()),
            Err(error) => {
                f.diagnostic = Some(error);
                f.budget.fail(Stop::NativeFailure)
            }
        }
    }
}
