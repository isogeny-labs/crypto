//! Original-bound optional initial channel. The returned state owns every stage;
//! callers must transfer it through parent delivery, not stringify/drop errors.
use super::*;
use crate::ml_dataset::types::{CaseConfig, CaseData};
use initial_native::{checkpoint, NativeInitial};

pub(in crate::ml_dataset::solvers) const REPORT_CAP: usize = 512;
pub(in crate::ml_dataset::solvers) const REPORT_WORK: u64 = 16 * REPORT_CAP as u64 + 4096;
pub(in crate::ml_dataset::solvers) const REPORT_BYTES: u64 = 16 * 1024;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum Eligibility {
    Absent,
    Requested,
    Unsupported,
}
pub(in crate::ml_dataset::solvers) fn eligibility(c: &CaseConfig) -> Eligibility {
    if c.channels
        .as_ref()
        .and_then(|r| r.sat_initial.as_ref())
        .is_none()
    {
        return Eligibility::Absent;
    }
    if c.solver == "sat" && profile(&c.family, c.n, c.m) {
        Eligibility::Requested
    } else {
        Eligibility::Unsupported
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum Stage {
    Original,
    ReportAdmission,
    Map,
    MapPublication,
    Native,
    Checkpoint,
}
/// One initial pattern; exact coverage is carried even when the pattern fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) struct Coverage {
    pub pattern: u32,
    pub total: u32,
    pub checked: u32,
}
#[allow(clippy::large_enum_variant)]
pub(in crate::ml_dataset::solvers) enum Owned<'a> {
    Admission {
        failure: AdmissionFailure<'a>,
        sink: capture::CaptureEmitter,
    },
    Map {
        owner: OriginalOwner<'a>,
        sink: capture::CaptureEmitter,
    },
    Native(initial_native::Failure<'a>),
    Checkpoint(checkpoint::Failed<'a>),
    Ready(checkpoint::WithLedger<'a>),
}
pub(in crate::ml_dataset::solvers) struct Prepared<'a> {
    pub owner: Owned<'a>,
    pub coverage: Coverage,
    pub stage: Stage,
    pub stop: Option<Stop>,
    /// Only true if the original ledger paid the bounded parent-outcome owner.
    pub report_admitted: bool,
}
/// Caller has selected Eligibility::Requested from frozen config. This function
/// never opens another sink or creates a second producer Budget.
pub(in crate::ml_dataset::solvers) fn prepare<'a>(
    original: &'a VerifiedWorkerOriginal<'a>,
    case: &CaseData,
    context: &execution::SolverExecutionContext,
    mut sink: capture::CaptureEmitter,
    limits: &capture::CaptureLimits,
) -> Prepared<'a> {
    let coverage = Coverage {
        pattern: 0,
        total: original
            .config
            .m
            .checked_sub(2)
            .and_then(|n| u32::try_from(n).ok())
            .and_then(|n| 1u32.checked_shl(n))
            .unwrap_or(0),
        checked: 0,
    };
    let mut owner = match OriginalOwner::admit(original, case, context, &sink, limits) {
        Ok(o) => o,
        Err(failure) => {
            let stop = match &failure {
                AdmissionFailure::BeforeBudget(_) => None,
                AdmissionFailure::Retained(f) => f.first_stop(),
            };
            return Prepared {
                owner: Owned::Admission { failure, sink },
                coverage,
                stage: Stage::Original,
                stop,
                report_admitted: false,
            };
        }
    };
    // Exact bounded parent output only. The actual checkpoint is reused as the
    // terminal capture receipt: no second capture event/root or sink.finish().
    if let Err(stop) = owner
        .budget
        .reserve_checking(REPORT_WORK, REPORT_BYTES, Instant::now())
    {
        return Prepared {
            owner: Owned::Map { owner, sink },
            coverage,
            stage: Stage::ReportAdmission,
            stop: Some(stop),
            report_admitted: false,
        };
    }
    let map_limits = provenance::Limits {
        variables: 1_000_000,
        codes: owner.math.codes.len(),
        domain_clauses: 1_000_000,
        work: owner.budget.native_work_limit,
        bytes: owner.budget.memory_limit,
    };
    if let Err(stop) = owner.build_map(0, map_limits) {
        let stop = owner.budget.fail::<()>(stop).unwrap_err();
        return Prepared {
            owner: Owned::Map { owner, sink },
            coverage,
            stage: Stage::Map,
            stop: Some(stop),
            report_admitted: true,
        };
    }
    if let Err(stop) = owner.persist_map(&mut sink) {
        let stop = owner.budget.fail::<()>(stop).unwrap_err();
        return Prepared {
            owner: Owned::Map { owner, sink },
            coverage,
            stage: Stage::MapPublication,
            stop: Some(stop),
            report_admitted: true,
        };
    }
    let checker = provenance::initial::Limits {
        work: owner.budget.native_work_limit,
        bytes: owner.budget.memory_limit,
        events: 1_000_000,
        constraints: 1_000_000,
    };
    let native = match NativeInitial::prepare(owner, sink, checker) {
        Ok(n) => n,
        Err(f) => {
            let stop = match &f {
                initial_native::Failure::BeforeNative { stop, .. }
                | initial_native::Failure::Retained { stop, .. } => *stop,
            };
            return Prepared {
                owner: Owned::Native(f),
                coverage,
                stage: Stage::Native,
                stop: Some(stop),
                report_admitted: true,
            };
        }
    };
    match checkpoint::finish(native) {
        Ok(owner) => Prepared {
            owner: Owned::Ready(owner),
            coverage: Coverage {
                checked: 1,
                ..coverage
            },
            stage: Stage::Checkpoint,
            stop: None,
            report_admitted: true,
        },
        Err(f) => {
            let stop = f.stop;
            Prepared {
                owner: Owned::Checkpoint(f),
                coverage,
                stage: Stage::Checkpoint,
                stop: Some(stop),
                report_admitted: true,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requested_cohort_is_not_filtered_to_m3() {
        for family in ["binary", "koblitz"] {
            for n in [23, 31, 47, 61] {
                for m in 2..=15 {
                    for _twist in [false, true] {
                        assert!(profile(family, n, m));
                        assert_eq!(1u32 << (m - 2), 2u32.pow((m - 2) as u32));
                    }
                }
            }
        }
        for (family, n, m) in [
            ("prime", 23, 3),
            ("binary", 32, 3),
            ("koblitz", 46, 3),
            ("binary", 23, 1),
            ("binary", 23, 16),
        ] {
            assert!(!profile(family, n, m));
        }
    }
}

/// Parent-finalization custody. All variable-sized data and original ledgers
/// move here; the borrowed token is replaced only by its fixed authenticated
/// hashes. No stored acknowledgement is recreated from metadata.
pub(in crate::ml_dataset::solvers) struct Custody {
    public_sha256: [u8; 32],
    factor_sha256: [u8; 32],
    budget: Option<Budget>,
    decoded: Option<DecodedParts>,
    map: Option<MapParts>,
    native: Option<NativeInitial<'static>>,
    checkpoint: Option<checkpoint::WithLedger<'static>>,
    sink: Option<capture::CaptureEmitter>,
    pub coverage: Coverage,
    pub stage: Stage,
    pub stop: Option<Stop>,
    pub report_admitted: bool,
    report_taken: bool,
    admission_stage: Option<AdmissionStage>,
}
struct MapParts {
    binding: [u8; 32],
    launch_sha256: [u8; 32],
    math: Math,
    checked: Option<provenance::CheckedMap>,
    retained_unchecked: Option<provenance::Map>,
    pattern: Option<u32>,
    retained_bytes: Vec<u8>,
    map_ack: Option<capture::CommittedEventPosition>,
    publication_attempted: bool,
    map_published: bool,
    capture_limits: capture::CaptureLimits,
}
impl Prepared<'_> {
    pub(in crate::ml_dataset::solvers) fn detach(self) -> Custody {
        let mut c = Custody {
            public_sha256: [0; 32],
            factor_sha256: [0; 32],
            budget: None,
            decoded: None,
            map: None,
            native: None,
            checkpoint: None,
            sink: None,
            coverage: self.coverage,
            stage: self.stage,
            stop: self.stop,
            report_admitted: self.report_admitted,
            report_taken: false,
            admission_stage: None,
        };
        match self.owner {
            Owned::Admission { failure, sink } => {
                c.sink = Some(sink);
                if let AdmissionFailure::BeforeBudget(stage) = &failure {
                    c.admission_stage = Some(*stage);
                }
                if let AdmissionFailure::Retained(f) = failure {
                    c.admission_stage = Some(f.stage);
                    c.public_sha256 = f.original.public_sha256;
                    c.factor_sha256 = f.original.factor_base_sha256;
                    c.budget = Some(f.budget);
                    c.decoded = Some(f.decoded);
                }
            }
            Owned::Map { owner, sink } => c.take_map(owner, sink),
            Owned::Native(initial_native::Failure::BeforeNative { owner, sink, .. }) => {
                c.take_map(owner, sink)
            }
            Owned::Native(initial_native::Failure::Retained { stage, .. }) => {
                c.native = Some(stage.detach())
            }
            Owned::Checkpoint(f) => c.checkpoint = Some(f.owner.detach()),
            Owned::Ready(o) => c.checkpoint = Some(o.detach()),
        }
        c
    }
}
impl Custody {
    fn take_map(&mut self, o: OriginalOwner<'_>, sink: capture::CaptureEmitter) {
        let OriginalOwner {
            original,
            budget,
            binding,
            launch_sha256,
            math,
            checked,
            retained_unchecked,
            pattern,
            retained_bytes,
            map_ack,
            publication_attempted,
            map_published,
            capture_limits,
        } = o;
        self.public_sha256 = original.public_sha256;
        self.factor_sha256 = original.factor_base_sha256;
        self.budget = Some(budget);
        self.sink = Some(sink);
        self.map = Some(MapParts {
            binding,
            launch_sha256,
            math,
            checked,
            retained_unchecked,
            pattern,
            retained_bytes,
            map_ack,
            publication_attempted,
            map_published,
            capture_limits,
        });
    }
    /// Bounded parent outcome is allocated only under the prior REPORT reservation.
    /// The checkpoint is the sole terminal capture receipt; parent result.json
    /// finalization retains its existing root-owned metadata scope.
    pub(in crate::ml_dataset::solvers) fn outcome(&mut self) -> Option<Value> {
        if !self.report_admitted || self.report_taken {
            return None;
        }
        self.report_taken = true;
        let receipt = self
            .checkpoint
            .as_ref()
            .and_then(|o| o.acknowledged_receipt());
        let verified = self.stop.is_none() && self.coverage.checked == 1 && receipt.is_some();
        let mut value = serde_json::json!({"adapter_id":"sat-initial-v1","strategy":"sat-initial-v1","status":"partial","solver_completed":false,"scalar_recovered_without_witness":false,"initial_checked":verified,"pattern":self.coverage.pattern,"patterns_total":self.coverage.total,"patterns_checked":if verified{1}else{0},"first_stop":self.stop});
        if let Some(r) = receipt {
            value["checkpoint_sequence"] = serde_json::json!(r.event_count);
            value["checkpoint_sha256"] = serde_json::json!(&r.last_event_sha256);
        }
        assert!(
            super::super::worker_terminal_encoded_size(&value, REPORT_CAP).is_ok(),
            "closed SAT terminal owner exceeded admitted schema"
        );
        Some(value)
    }
}

#[cfg(test)]
mod shared_receipt_tests {
    use super::*;
    #[test]
    fn real_compact_shared_receipt_preserves_original_owner_and_caps() {
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
            let sink = capture::CaptureEmitter::create(
                &f.root.join("live"),
                f.binding.clone(),
                caps.clone(),
            )
            .unwrap();
            let prepared = prepare(&token, &case, &context, sink, &caps);
            assert!(
                matches!(&prepared.owner, Owned::Ready(_)),
                "required original-budget shared receipt failed: {:?} {:?}",
                prepared.stage,
                prepared.stop
            );
            assert_eq!(
                prepared.coverage,
                Coverage {
                    pattern: 0,
                    total: 2,
                    checked: 1
                }
            );
            let mut custody = prepared.detach();
            let before = std::fs::read(f.root.join("live/committed.json")).unwrap();
            let outcome = custody.outcome().expect("admitted compact parent outcome");
            assert_eq!(outcome["initial_checked"], true);
            assert_eq!(outcome["solver_completed"], false);
            assert!(serde_json::to_vec(&outcome).unwrap().len() <= REPORT_CAP);
            assert!(
                custody.outcome().is_none(),
                "one paid outcome cannot be allocated twice"
            );
            assert_eq!(
                before,
                std::fs::read(f.root.join("live/committed.json")).unwrap(),
                "no duplicate terminal capture publication"
            );
            let head: capture::CommittedHead = serde_json::from_slice(&before).unwrap();
            assert_eq!(outcome["checkpoint_sequence"], head.event_count);
            assert_eq!(outcome["checkpoint_sha256"], head.last_event_sha256);
        }
    }
    #[test]
    fn maximum_closed_parent_outcome_fits_admitted_bound() {
        let v = serde_json::json!({"adapter_id":"sat-initial-v1","strategy":"sat-initial-v1","status":"partial","solver_completed":false,"scalar_recovered_without_witness":false,"initial_checked":true,"pattern":u32::MAX,"patterns_total":u32::MAX,"patterns_checked":u32::MAX,"first_stop":"Serialization","checkpoint_sequence":u64::MAX,"checkpoint_sha256":"f".repeat(64)});
        assert!(serde_json::to_vec(&v).unwrap().len() <= REPORT_CAP);
    }
}
