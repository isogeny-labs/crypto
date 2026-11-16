//! Private original-owner bridge. A fixed unit window gives a finite row span,
//! not a complete F4/F5 search. No caller-supplied map or raw polynomial enters.
use super::*;
use provenance::boolean_equations::{self as equations, matrix_rows as rows};
use std::mem::size_of;

// Empty monomial is one. Static storage needs no per-attempt allocation.
static UNIT_WINDOW: [equations::Monomial; 1] = [Vec::new()];
#[derive(Clone, Copy)]
pub(in crate::ml_dataset::solvers) struct Caps {
    pub expansion: equations::Limits,
    pub reduction: rows::Limits,
    pub checker: rows::CheckerLimits,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum Failure {
    Admission(Stop),
    MissingMap,
    Expansion(equations::Failure),
    Reduction(rows::Failure),
    Certification(rows::Failure),
    Publication(Stop),
}
enum State<'s> {
    Before,
    Expanded(equations::Expansion<'s>),
    Reduced(rows::BoundRows<'s, 'static>),
    Certified(rows::Certificate<'s, 'static>),
}
/// The enclosing original owner is immutably borrowed through its exact map and
/// original token, and mutably borrowed through its ledger for this entire life.
/// Dropping this view cannot refund the original ledger or clear its first stop.
/// A parent must keep it until evidence/custody transfer; this increment does not
/// implement that later publication operation.
pub(in crate::ml_dataset::solvers) struct Attempt<'s, 'o> {
    original: &'s VerifiedWorkerOriginal<'o>,
    map: Option<&'s provenance::CheckedMap>,
    ledger: &'s mut Budget,
    state: State<'s>,
    failure: Option<Failure>,
    deadline: Instant,
}
impl Attempt<'_, '_> {
    pub(in crate::ml_dataset::solvers) fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub(in crate::ml_dataset::solvers) fn first_stop(&self) -> Option<Stop> {
        self.ledger.stopped
    }
    pub(in crate::ml_dataset::solvers) fn charged(&self) -> (u64, u64) {
        (self.ledger.checking_work, self.ledger.adapter_bytes)
    }
    pub(in crate::ml_dataset::solvers) fn certificate(
        &self,
    ) -> Option<&rows::Certificate<'_, 'static>> {
        match &self.state {
            State::Certified(c) => Some(c),
            _ => None,
        }
    }
    fn stop(&mut self, failure: Failure, stop: Stop) {
        self.failure = Some(failure);
        let _: Result<(), Stop> = self.ledger.fail(stop);
    }
}
impl<'o> OriginalOwner<'o> {
    /// One-shot, caller-selected row in the full unit-window reduction. Local
    /// limits only restrict operations; they never replace the original ledger.
    pub(super) fn produce_boolean_row<'s>(&'s mut self, row: usize, caps: Caps) -> Attempt<'s, 'o> {
        let repeated = self.boolean_attempted;
        self.boolean_attempted = true;
        let deadline = if matches!(
            self.budget.phase,
            super::super::Phase::Setup | super::super::Phase::Construction
        ) {
            self.budget.construction_deadline
        } else {
            self.budget.deadline
        };
        let mut out = Attempt {
            original: self.original,
            map: self.checked.as_ref(),
            ledger: &mut self.budget,
            state: State::Before,
            failure: None,
            deadline,
        };
        if repeated {
            out.stop(Failure::Admission(Stop::Phase), Stop::Phase);
            return out;
        }
        // Fixed retained control-frame charge precedes any algorithm allocation.
        if let Err(s) =
            out.ledger
                .reserve_checking(64, size_of::<Attempt<'s, 'o>>() as u64, Instant::now())
        {
            out.failure = Some(Failure::Admission(s));
            return out;
        }
        let Some(map) = out.map else {
            out.stop(Failure::MissingMap, Stop::Phase);
            return out;
        };
        let expanded = equations::expand(map, caps.expansion, &mut Admit(&mut *out.ledger));
        if let Err(failure) = expanded.status() {
            out.state = State::Expanded(expanded);
            out.stop(Failure::Expansion(failure), Stop::NativeFailure);
            return out;
        }
        let reduced = rows::reduce(
            expanded,
            &UNIT_WINDOW,
            caps.reduction,
            &mut Admit(&mut *out.ledger),
        );
        if let Some(failure) = reduced.failure() {
            out.state = State::Reduced(reduced);
            out.stop(Failure::Reduction(failure), Stop::NativeFailure);
            return out;
        }
        let certificate = rows::certify(
            reduced,
            row,
            caps.reduction,
            caps.checker,
            deadline,
            &mut Admit(&mut *out.ledger),
        );
        let failure = certificate.failure();
        out.state = State::Certified(certificate);
        if let Some(failure) = failure {
            out.stop(Failure::Certification(failure), Stop::NativeFailure);
        } else if let Err(stop) = out.ledger.check(Instant::now()) {
            out.failure = Some(Failure::Admission(stop));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::super::verified_worker_original_tests::Fixture;
    use super::*;
    pub(super) fn caps() -> Caps {
        Caps {
            expansion: equations::Limits {
                variables: 4096,
                generators: 4096,
                terms: 4096,
                degree: 64,
                work: 4_000_000,
                bytes: 8_000_000,
            },
            reduction: rows::Limits {
                variables: 4096,
                generators: 4096,
                window: 1,
                candidates: 4096,
                basis: 4096,
                terms: 4096,
                degree: 64,
                proof: 4096,
                wire: 65536,
                work: 4_000_000,
                bytes: 8_000_000,
            },
            checker: rows::CheckerLimits {
                variables: 4096,
                generators: 4096,
                polynomial_terms: 4096,
                proof_terms: 4096,
                accumulator_entries: 4096,
                caps: [
                    65536, 262144, 262144, 262144, 262144, 65536, 1024, 1048576, 262144,
                ],
            },
        }
    }
    fn with_owner(twist: bool, action: impl FnOnce(&mut OriginalOwner<'_>)) {
        let f = Fixture::with_ordinary_runtime_shape(23, twist, "sat");
        execution::create(&f.root).unwrap();
        let mut registry = Vec::new();
        let generated = crate::ml_dataset::telemetry::measured(
            &f.root,
            &mut registry,
            "curve-construction",
            || crate::ml_dataset::curves::generate(&f.config),
        )
        .unwrap();
        let token = super::super::super::super::verify_and_store_worker_originals(
            &f.config,
            &generated,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let context = execution::admit(&f.root).unwrap().unwrap();
        let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
            .unwrap()
            .unwrap();
        let sink = capture::CaptureEmitter::create(
            &f.root.join("live"),
            f.binding.clone(),
            limits.clone(),
        )
        .unwrap();
        let mut owner = match OriginalOwner::admit(&token, &generated, &context, &sink, &limits) {
            Ok(owner) => owner,
            Err(_) => panic!("genuine checked factory admission required"),
        };
        owner
            .build_map(
                1,
                provenance::Limits {
                    variables: 4096,
                    codes: 16,
                    domain_clauses: 4096,
                    work: 4_000_000,
                    bytes: 8_000_000,
                },
            )
            .unwrap();
        action(&mut owner);
        assert_eq!(
            sink.committed_event_count(),
            0,
            "row construction is not capture publication"
        );
    }
    #[test]
    fn raw_n23_original_expansion_cap_keeps_exact_prefix_and_original_owner() {
        with_owner(false, |owner| {
            let before = (owner.budget.checking_work, owner.budget.adapter_bytes);
            let deadline = owner.budget.construction_deadline;
            let original = owner.original as *const _;
            let map = owner.checked.as_ref().unwrap() as *const _;
            let after;
            {
                let attempt = owner.produce_boolean_row(0, caps());
                assert_eq!(attempt.failure(), Some(Failure::Expansion(equations::Failure::Unknown(equations::Unknown::Cap))));
                assert_eq!(attempt.first_stop(), Some(Stop::NativeFailure));
                assert_eq!(attempt.original as *const _, original);
                assert_eq!(attempt.deadline, deadline);
                assert!(attempt.certificate().is_none());
                let State::Expanded(expansion) = &attempt.state else { panic!("actual raw expansion prefix required") };
                assert_eq!(expansion.source() as *const _, map);
                assert_eq!(expansion.equations().len(), 97);
                assert_eq!(expansion.charged().work, 3_994_150);
                assert_eq!(expansion.charged().bytes, 1_514_872);
                let (partial, scratch, stage, row, literal) = expansion.retained();
                assert_eq!((stage, row, literal), (equations::Stage::Domain, 5, 13));
                assert!(!partial.is_empty()); assert!(!scratch.is_empty());
                after = attempt.charged();
                assert!(after.0 > before.0 && after.1 > before.1);
            }
            assert_eq!((owner.budget.checking_work, owner.budget.adapter_bytes), after);
            let retry = owner.produce_boolean_row(0, caps());
            assert_eq!(retry.failure(), Some(Failure::Admission(Stop::Phase)));
            assert_eq!(retry.first_stop(), Some(Stop::NativeFailure));
            assert_eq!(retry.charged(), after);
        });
    }
    #[test]
    fn actual_local_expansion_refusal_keeps_map_prefix_and_same_original_ledger() {
        with_owner(false, |owner| {
            let mut limits = caps();
            limits.expansion.terms = 0;
            let before = owner.budget.checking_work;
            let attempt = owner.produce_boolean_row(0, limits);
            assert!(matches!(attempt.failure(), Some(Failure::Expansion(_))));
            assert!(matches!(attempt.state, State::Expanded(_)));
            assert!(attempt.map.is_some());
            assert!(attempt.charged().0 > before);
            assert_eq!(attempt.first_stop(), Some(Stop::NativeFailure));
        });
    }
    #[test]
    fn original_deadline_refusal_precedes_expansion_and_is_not_renewed() {
        with_owner(false, |owner| {
            owner.budget.construction_deadline = Instant::now();
            let before = (owner.budget.checking_work, owner.budget.adapter_bytes);
            let attempt = owner.produce_boolean_row(0, caps());
            assert_eq!(attempt.failure(), Some(Failure::Admission(Stop::Deadline)));
            assert_eq!(attempt.first_stop(), Some(Stop::Deadline));
            assert!(matches!(attempt.state, State::Before));
            assert_eq!(attempt.charged(), before);
        });
    }
    #[test]
    fn original_profile_keeps_future_m4_through_m15_and_both_field_ranges() {
        for family in ["binary", "koblitz"] {
            for n in (23..=31).chain(47..=61) {
                for m in 4..=15 {
                    assert!(profile(family, n, m));
                }
            }
        }
    }
}

/// Owned worker custody, with no lifetime tied to the local original token/map.
/// These are the existing admitted source fields moved once, never reconstructed.
struct OriginalParts {
    public_sha256: [u8; 32],
    factor_sha256: [u8; 32],
    budget: Budget,
    binding: [u8; 32],
    launch_sha256: [u8; 32],
    math: Math,
    checked: Option<provenance::CheckedMap>,
    unchecked: Option<provenance::Map>,
    pattern: Option<u32>,
    map_bytes: Vec<u8>,
    map_ack: Option<capture::CommittedEventPosition>,
    publication_attempted: bool,
    map_published: bool,
    capture_limits: capture::CaptureLimits,
    boolean_attempted: bool,
}
enum OwnedState {
    Before,
    Expanded(equations::OwnedExpansion),
    Reduced(rows::OwnedRows),
    Certified(rows::OwnedCertificate),
}
pub(in crate::ml_dataset::solvers) struct OwnedAttempt {
    original: OriginalParts,
    sink: capture::CaptureEmitter,
    state: OwnedState,
    failure: Option<Failure>,
    deadline: Instant,
    diagnostic: Option<String>,
    selected_row: usize,
    publication: Publication,
}
impl OwnedAttempt {
    pub(in crate::ml_dataset::solvers) fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub(in crate::ml_dataset::solvers) fn charged(&self) -> (u64, u64) {
        (
            self.original.budget.checking_work,
            self.original.budget.adapter_bytes,
        )
    }
    pub(in crate::ml_dataset::solvers) fn first_stop(&self) -> Option<Stop> {
        self.original.budget.stopped
    }
    pub(in crate::ml_dataset::solvers) fn certificate(&self) -> Option<&rows::OwnedCertificate> {
        match &self.state {
            OwnedState::Certified(c) => Some(c),
            _ => None,
        }
    }
    fn stop(&mut self, failure: Failure, stop: Stop) {
        self.failure = Some(failure);
        let _: Result<(), Stop> = self.original.budget.fail(stop);
    }
    fn run(&mut self, row: usize, caps: Caps) {
        let Some(map) = self.original.checked.take() else {
            self.stop(Failure::MissingMap, Stop::Phase);
            return;
        };
        let expansion =
            equations::expand_owned(map, caps.expansion, &mut Admit(&mut self.original.budget));
        if let Err(f) = expansion.status() {
            self.state = OwnedState::Expanded(expansion);
            self.stop(Failure::Expansion(f), Stop::NativeFailure);
            return;
        }
        let reduced = rows::reduce_owned(
            expansion,
            &UNIT_WINDOW,
            caps.reduction,
            &mut Admit(&mut self.original.budget),
        );
        if let Some(f) = reduced.failure() {
            self.state = OwnedState::Reduced(reduced);
            self.stop(Failure::Reduction(f), Stop::NativeFailure);
            return;
        }
        let certificate = rows::certify_owned(
            reduced,
            row,
            caps.reduction,
            caps.checker,
            self.deadline,
            &mut Admit(&mut self.original.budget),
        );
        let failure = certificate.failure();
        self.state = OwnedState::Certified(certificate);
        if let Some(f) = failure {
            self.stop(Failure::Certification(f), Stop::NativeFailure)
        } else if let Err(s) = self.original.budget.check(Instant::now()) {
            self.failure = Some(Failure::Admission(s))
        }
    }
}
impl OriginalOwner<'_> {
    /// Consumes the original owner and actual bound sink. It always returns
    /// custody, including on initial frame admission or sink-identity failure.
    pub(super) fn into_boolean_row(
        mut self,
        sink: capture::CaptureEmitter,
        row: usize,
        caps: Caps,
    ) -> OwnedAttempt {
        let repeated = self.boolean_attempted;
        self.boolean_attempted = true;
        let deadline = if matches!(
            self.budget.phase,
            super::super::Phase::Setup | super::super::Phase::Construction
        ) {
            self.budget.construction_deadline
        } else {
            self.budget.deadline
        };
        let mut diagnostic = None;
        let admission = (|| -> Result<(), Stop> {
            if repeated {
                return self.budget.fail(Stop::Phase);
            }
            self.budget.reserve_checking(
                8192,
                size_of::<OwnedAttempt>() as u64 + 65536,
                Instant::now(),
            )?;
            let Some(binding) = self.original.binding else {
                return self.budget.fail(Stop::InvalidLimits);
            };
            if let Err(error) = sink.verify_directory(&self.original.root.join("live")) {
                diagnostic = Some(error);
                return self.budget.fail(Stop::Observer);
            }
            if let Err(error) = sink.verify_attempt_limits(binding, &self.capture_limits) {
                diagnostic = Some(error);
                return self.budget.fail(Stop::Observer);
            }
            self.budget.check(Instant::now())
        })();
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
            boolean_attempted,
        } = self;
        let mut out = OwnedAttempt {
            original: OriginalParts {
                public_sha256: original.public_sha256,
                factor_sha256: original.factor_base_sha256,
                budget,
                binding,
                launch_sha256,
                math,
                checked,
                unchecked: retained_unchecked,
                pattern,
                map_bytes: retained_bytes,
                map_ack,
                publication_attempted,
                map_published,
                capture_limits,
                boolean_attempted,
            },
            sink,
            state: OwnedState::Before,
            failure: None,
            deadline,
            diagnostic,
            selected_row: row,
            publication: Publication::default(),
        };
        if let Err(s) = admission {
            out.failure = Some(Failure::Admission(s));
            return out;
        }
        out.run(row, caps);
        out
    }
}

#[cfg(test)]
mod owned_tests {
    use super::super::super::super::verified_worker_original_tests::Fixture;
    use super::tests::caps;
    use super::*;
    use crate::ml_dataset::solvers;
    fn genuine_owned(twist: bool, refuse_wire: bool) -> OwnedAttempt { genuine_owned_fixture(twist,refuse_wire).0 }
    fn genuine_owned_fixture(twist: bool, refuse_wire: bool) -> (OwnedAttempt,Fixture,Vec<crate::ml_dataset::types::Artifact>) {
        let f = Fixture::with_ordinary_runtime_shape(23, twist, "sat");
        execution::create(&f.root).unwrap();
        let mut registry = Vec::new();
        let generated = crate::ml_dataset::telemetry::measured(
            &f.root,
            &mut registry,
            "curve-construction",
            || crate::ml_dataset::curves::generate(&f.config),
        )
        .unwrap();
        let token = solvers::verify_and_store_worker_originals(
            &f.config,
            &generated,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let context = execution::admit(&f.root).unwrap().unwrap();
        let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
            .unwrap()
            .unwrap();
        let mut sink = capture::CaptureEmitter::create(
            &f.root.join("live"),
            f.binding.clone(),
            limits.clone(),
        )
        .unwrap();
        let mut owner = match OriginalOwner::admit(&token, &generated, &context, &sink, &limits) {
            Ok(o) => o,
            Err(_) => panic!("genuine original required"),
        };
        owner
            .build_map(
                1,
                provenance::Limits {
                    variables: 4096,
                    codes: 16,
                    domain_clauses: 4096,
                    work: 4_000_000,
                    bytes: 8_000_000,
                },
            )
            .unwrap();
        owner.persist_map(&mut sink).unwrap();
        let before = (owner.budget.checking_work, owner.budget.adapter_bytes);
        let deadline = owner.budget.construction_deadline;
        let mut c = caps();
        if refuse_wire {
            c.reduction.wire = 0
        }
        let result = owner.into_boolean_row(sink, 0, c);
        assert!(result.charged().0 > before.0);
        assert_eq!(result.deadline, deadline);
        assert!(result.original.map_ack.is_some());
        assert!(!result.original.map_bytes.is_empty());
        for (path,role) in crate::ml_dataset::CAPTURE_CONTEXT.into_iter().chain([(execution::PATH,execution::ROLE)]) {
            let(sha256,bytes)=crate::store::hash_file(&f.root.join(path)).unwrap();
            registry.push(crate::ml_dataset::types::Artifact{path:path.into(),role:role.into(),encoding:"json".into(),sha256,bytes});
        }
        // The token and generated case drop; retain Fixture for filesystem followups.
        (result,f,registry)
    }
    #[test]
    fn raw_n23_owned_expansion_cap_retains_map_ledger_and_blocks_publication() {
        let (mut owner, fixture, mut registry) = genuine_owned_fixture(false, false);
        assert!(fixture.root.join("public.json").is_file());
        assert_eq!(owner.failure(), Some(Failure::Expansion(equations::Failure::Unknown(equations::Unknown::Cap))));
        assert_eq!(owner.first_stop(), Some(Stop::NativeFailure));
        assert!(owner.certificate().is_none());
        let OwnedState::Expanded(expansion) = &owner.state else { panic!("actual owned raw expansion prefix required") };
        assert_eq!(expansion.equations().len(), 97);
        assert_eq!(expansion.charged().work, 3_994_150);
        assert_eq!(expansion.charged().bytes, 1_514_872);
        assert_eq!(expansion.source().variables(), 115);
        let charges = owner.charged();
        let deadline = owner.deadline;
        let acknowledgement = owner.original.map_ack.clone().expect("actual map receipt required");
        let map_pointer = owner.original.map_bytes.as_ptr();
        assert!(!owner.original.map_bytes.is_empty());
        let head = std::fs::read(fixture.root.join("live/committed.json")).unwrap();
        assert!(owner.outcome().is_none());
        let owner = owner.publish();
        assert_eq!(owner.charged(), charges);
        assert!(!owner.publication.attempted); assert!(!owner.publication.committed);
        let output = solvers::WorkerSolveOutput::from_original_boolean_rows(owner);
        let mut retained = solvers::WorkerResidualCustody::default();
        let registry_count = registry.len();
        assert!(crate::ml_dataset::finish_retained_worker_output(&fixture.root, &mut registry, output, Ok(()), &mut retained).is_err());
        let kept = retained.boolean_rows.as_ref().unwrap();
        assert_eq!(kept.charged(), charges); assert_eq!(kept.deadline, deadline);
        assert_eq!(kept.first_stop(), Some(Stop::NativeFailure));
        assert_eq!(kept.original.map_ack.as_ref(), Some(&acknowledgement));
        assert_eq!(kept.original.map_bytes.as_ptr(), map_pointer);
        assert!(matches!(kept.state, OwnedState::Expanded(_)));
        assert!(retained.boolean_rows_report_unavailable());
        assert_eq!(registry.len(), registry_count);
        assert_eq!(head, std::fs::read(fixture.root.join("live/committed.json")).unwrap());
    }







}

#[path="boolean_row_receipt.rs"]
pub(in crate::ml_dataset::solvers) mod receipt;
#[derive(Default)]
struct Publication {
    attempted:bool,metadata:Option<receipt::Metadata>,bytes:Vec<u8>,
    ack:Option<capture::CommittedEventPosition>,committed:bool,report_taken:bool,
    path:Option<String>,capture_error:Option<capture::CaptureError>,
}
struct ReceiptWriter<'a>{bytes:&'a mut Vec<u8>,cap:usize}
impl std::io::Write for ReceiptWriter<'_>{
    fn write(&mut self,b:&[u8])->std::io::Result<usize>{
        if self.bytes.len().checked_add(b.len()).filter(|n|*n<=self.cap).is_none(){return Err(std::io::ErrorKind::InvalidData.into())}
        self.bytes.extend_from_slice(b);Ok(b.len())
    }
    fn flush(&mut self)->std::io::Result<()>{Ok(())}
}
impl OwnedAttempt {
    pub(in crate::ml_dataset::solvers) fn publish(mut self)->Self {
        if self.failure.is_some(){return self}
        if let Err(stop)=self.publish_inner(){self.stop(Failure::Publication(stop),stop)}
        self
    }
    fn publish_inner(&mut self)->Result<(),Stop>{
        if self.publication.attempted{return Err(Stop::Phase)}
        self.publication.attempted=true;
        let OwnedState::Certified(certificate)=&self.state else{return Err(Stop::Phase)};
        if certificate.failure().is_some()||certificate.audit().is_none_or(|a|a.result.is_err())||!self.original.map_published{return Err(Stop::Phase)}
        let raw=certificate.raw().ok_or(Stop::Phase)?;
        // Pay the actual-wire scan/hash before inspecting its canonical alphabet.
        self.original.budget.reserve_checking((raw.len() as u64).checked_add(self.original.map_bytes.len() as u64).and_then(|n|n.checked_mul(16)).ok_or(Stop::Overflow)?,65536,Instant::now())?;
        let cap=receipt::cap(raw).ok_or(Stop::Serialization)?;
        if cap>self.original.capture_limits.max_payload_bytes{return Err(Stop::Bytes)}
        let cap_usize=usize::try_from(cap).map_err(|_|Stop::Overflow)?;
        let before=self.sink.committed_event_position();
        if self.original.map_ack.as_ref()!=Some(&before){return Err(Stop::Observer)}
        let storage=super::super::quote_parent_storage_length(cap,false,&self.original.capture_limits)?;
        let report=super::super::quote_parent_storage_length(receipt::REPORT_CAP,true,&self.original.capture_limits)?;
        let encoding=cap.checked_mul(16).ok_or(Stop::Overflow)?;
        self.original.budget.reserve_checking(storage.work.checked_add(report.work).and_then(|w|w.checked_add(encoding)).and_then(|w|w.checked_add(16*receipt::REPORT_CAP)).ok_or(Stop::Overflow)?,storage.bytes.checked_add(report.bytes).and_then(|b|b.checked_add(cap)).and_then(|b|b.checked_add(262144)).ok_or(Stop::Overflow)?,Instant::now())?;
        self.original.budget.reserve_trace(storage.trace.checked_add(report.trace).ok_or(Stop::Overflow)?,Instant::now())?;
        // No paid operation remains outside this bound before the snapshot is encoded.
        self.publication.bytes.try_reserve_exact(cap_usize).map_err(|_|Stop::Allocation)?;
        let mut map_event=[0;32];hex::decode_to_slice(&before.last_event_sha256,&mut map_event).map_err(|_|Stop::Serialization)?;
        let row=u64::try_from(self.selected_row).map_err(|_|Stop::Overflow)?;
        self.publication.path=Some(receipt::path(self.original.pattern.ok_or(Stop::Phase)?,row));
        let b=&self.original.budget;
        if let Some(stop)=b.stopped{return Err(stop)}
        self.publication.metadata=Some(receipt::Metadata{
            original_binding:self.original.binding,launch_sha256:self.original.launch_sha256,
            pattern:self.original.pattern.unwrap(),row,window:0,
            variables:certificate.rows().source().variables(),generators:certificate.rows().source().equations().len() as u64,
            map_sha256:Sha256::digest(&self.original.map_bytes).into(),map_bytes:self.original.map_bytes.len() as u64,
            map_sequence:before.event_count,map_event_sha256:map_event,
            wire_sha256:Sha256::digest(raw).into(),wire_bytes:raw.len() as u64,
            limits:[b.native_work_limit,b.memory_limit,b.trace_limit,b.addition_limit],
            totals:[b.native.work,b.native.bytes,b.checking_work,b.checking_performed_work,b.adapter_bytes,b.trace_bytes,b.additions],
            phase:b.phase as u8,phase_native:[b.phase_native.work,b.phase_native.bytes],phase_limits:[b.phase_limits.work,b.phase_limits.bytes],
            first_stop:b.stopped,solver_completed:false,full_span_search:false,scalar_recovered:false,
        });
        let wire=receipt::Wire{schema:receipt::SCHEMA,metadata:self.publication.metadata.as_ref().unwrap(),wire:std::str::from_utf8(raw).map_err(|_|Stop::Serialization)?};
        serde_json::to_writer(ReceiptWriter{bytes:&mut self.publication.bytes,cap:cap_usize},&wire).map_err(|_|Stop::Serialization)?;
        self.original.budget.check(Instant::now())?;
        let result=self.sink.payload_borrowed(self.publication.path.as_ref().unwrap(),receipt::ROLE,"json",&self.publication.bytes);
        if let Err(error)=result{self.publication.capture_error=Some(error);return Err(Stop::Observer)}
        let after=self.sink.committed_event_position();
        let exact=after.event_count==before.event_count.checked_add(1).ok_or(Stop::Overflow)?&&after.binding_sha256==before.binding_sha256&&after.byte_end>before.byte_end;
        self.publication.ack=Some(after);
        if !exact{return Err(Stop::Observer)}
        self.original.budget.check(Instant::now())?;
        self.publication.committed=true;Ok(())
    }
    pub(in crate::ml_dataset::solvers) fn outcome(&mut self)->Option<Value>{
        if !self.publication.committed||self.failure.is_some()||self.publication.report_taken{return None}
        if let Err(stop)=self.original.budget.check(Instant::now()){self.stop(Failure::Publication(stop),stop);return None}
        self.publication.report_taken=true;
        let ack=self.publication.ack.as_ref()?;
        Some(serde_json::json!({"adapter_id":"original-unit-boolean-row-v1","strategy":"original-unit-boolean-row-v1","status":"partial","finite_row_receipt_committed":true,"solver_completed":false,"scalar_recovered_without_witness":false,"ecdlp_pipeline_completed":false,"full_span_search":false,"pattern":self.original.pattern,"row":self.selected_row,"receipt_sequence":ack.event_count,"receipt_sha256":ack.last_event_sha256}))
    }
}
