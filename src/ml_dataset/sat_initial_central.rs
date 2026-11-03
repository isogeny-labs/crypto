//! Read-only central initial-prefix verification. No generator, clock renewal,
//! producer-budget creation, mutable original factory, or exporter callback.
use super::*;
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
#[path = "sat_initial_read.rs"]
mod retained_read;
use retained_read::read;
/// Historical verification uses LaunchContext::validate through execution::verify,
/// never execution::admit (which would inspect today's clock as producer entry).
pub(in crate::ml_dataset::solvers) fn verify(
    dir: &Path,
    config: &CaseConfig,
    artifacts: &[Artifact],
    policy: Policy,
) -> Result<Outcome, Failed> {
    let mut frame = Frame {
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
    let head_raw = read(
        dir,
        "live/committed.json",
        caps.metadata_reserve_bytes,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    let head: capture::CommittedHead =
        serde_json::from_slice(&head_raw).map_err(|e| e.to_string())?;
    if head.byte_end > f.budget.policy.stream_bytes
        || head.payload_bytes > f.budget.policy.payload_bytes
    {
        return Err("replay stream/payload cap".into());
    }
    f.head = Some(head.clone());
    f.budget
        .reserve_checking(
            head.byte_end
                .checked_add(head.payload_bytes)
                .and_then(|n| n.checked_mul(64))
                .ok_or("capture quote overflow")?,
            head.byte_end
                .checked_mul(128)
                .and_then(|n| n.checked_add(2 * 1024 * 1024))
                .ok_or("capture allocation overflow")?,
            Instant::now(),
        )
        .map_err(|_| "capture admission refused".to_owned())?;
    let binding_hash = binding.digest()?;
    let mut marker = None;
    let mut checkpoint_receipt = None;
    let authenticated = capture::verify_live_committed_prefix_visit_at_head_preflight(
        dir,
        &binding,
        &caps,
        &head,
        &mut |phase, line| {
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
                return Ok(());
            }
            let row: capture::EventRecord =
                serde_json::from_slice(line).map_err(|e| e.to_string())?;
            let digest = hex::encode(Sha256::digest(line));
            f.visited.last_hash = Some(digest.clone());
            if row.event["kind"] == "capture_payload_committed" {
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
            } else if row.event["kind"] == "binary_sat_initial" {
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
            } else {
                return Err("unsupported event in initial-only capture".into());
            }
            Ok(())
        },
    )?
    .ok_or("missing authenticated capture")?;
    f.visited
        .exact(&head)
        .map_err(|_| "visited endpoint differs from admitted head")?;
    if authenticated.payloads.len() != 1 {
        return Err("initial map custody count mismatch".into());
    }
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
    let after = read(
        dir,
        "live/committed.json",
        caps.metadata_reserve_bytes,
        &mut f.budget,
        &mut f.pending_read,
    )?;
    if after != head_raw {
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
