//! One-shot continuation of the actual acknowledged original native owner.
//! Not wired into production dispatch; native admission gates remain closed.
use super::*;
use crate::cryptanalysis::sat::{NativeAdmissionFailure, SolveResult};
use provenance::checked_model;

#[derive(Clone, Copy)]
pub(in crate::ml_dataset::solvers) struct SearchCaps {
    pub events: usize,
    pub conflicts: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum Verdict {
    Partial,
    NativeUnsatUncertified,
    CheckedSignedRelation,
}
pub(in crate::ml_dataset::solvers) struct OwnedSearch<'a> {
    initial: WithLedger<'a>,
    model: Option<Vec<bool>>,
    verification: checked_model::Progress,
    verdict: Verdict,
    first_stop: Option<Stop>,
    invoked: bool,
    result_ack: Option<capture::CommittedEventPosition>,
    native_result: Option<NativeOutcome>,
    terminal_ack: Option<capture::CommittedEventPosition>,
    report: Option<SearchReport>,
}
/// Kept in the same state captured by the original installed callback.
/// Initial fields/seal are never reopened or appended by this phase.
pub(in crate::ml_dataset::solvers) struct SearchCapture {
    active: bool,
    limit: usize,
    events: Vec<SolverEvent>,
    pending: Option<SolverEvent>,
    initial_ack: capture::CommittedEventPosition,
    last_ack: capture::CommittedEventPosition,
}
#[derive(Serialize)]
struct SearchEvent<'a> {
    schema: &'static str,
    original_binding: [u8; 32],
    pattern: u32,
    initial_checkpoint_sequence: u64,
    initial_checkpoint_sha256: &'a str,
    native_index: usize,
    event: &'a SolverEvent,
}
fn reserve<T>(v: &mut Vec<T>, n: usize, b: &mut Budget) -> Result<(), Stop> {
    if n > v.capacity() {
        let cap = n
            .max(v.capacity().checked_mul(2).ok_or(Stop::Overflow)?)
            .max(4);
        b.reserve_checking(
            v.len() as u64,
            (cap.checked_mul(std::mem::size_of::<T>())
                .ok_or(Stop::Overflow)?) as u64,
            Instant::now(),
        )?;
        v.try_reserve_exact(cap - v.len())
            .map_err(|_| Stop::Allocation)?;
        b.check(Instant::now())?;
    }
    Ok(())
}
pub(in crate::ml_dataset::solvers) fn on_event(
    state: &mut CaptureState,
    event: &SolverEvent,
) -> Result<(), Stop> {
    let mut search = state.search.take().ok_or(Stop::Phase)?;
    let result = (|| {
        if !search.active
            || !state.closed
            || !state.sealed
            || search.pending.is_some()
            || search.events.len() >= search.limit
        {
            return Err(Stop::Phase);
        }
        state.budget.check(Instant::now())?;
        let mut count = Count {
            budget: &mut state.budget,
            len: 0,
            cap: state.limits.max_event_bytes as usize,
        };
        serde_json::to_writer(&mut count, event).map_err(|_| Stop::Serialization)?;
        let len = count.len as u64;
        state.budget.reserve_checking(
            len.checked_mul(8).ok_or(Stop::Overflow)?,
            len.checked_mul(8)
                .and_then(|n| n.checked_add(1024))
                .ok_or(Stop::Overflow)?,
            Instant::now(),
        )?;
        let next = search.events.len().checked_add(1).ok_or(Stop::Overflow)?;
        reserve(&mut search.events, next, &mut state.budget)?;
        search.pending = Some(event.clone());
        state.budget.check(Instant::now())?;
        let record = SearchEvent {
            schema: "binary-sat-search-event/v1",
            original_binding: state.binding,
            pattern: state.pattern,
            initial_checkpoint_sequence: search.initial_ack.event_count,
            initial_checkpoint_sha256: &search.initial_ack.last_event_sha256,
            native_index: search.events.len(),
            event: search.pending.as_ref().unwrap(),
        };
        let before = state.sink.committed_event_position();
        if before != search.last_ack {
            return Err(Stop::Observer);
        }
        let result = publish_kind(
            capture::TypedEventKind::BinarySatSearch,
            &mut state.budget,
            &mut state.sink,
            state.buffer.as_mut().ok_or(Stop::Phase)?,
            state.limits.max_event_bytes as usize,
            &record,
        );
        // Retain actual custody position even when publication or postchecks fail.
        search.last_ack = state.sink.committed_event_position();
        let publication = result?;
        if !publication.valid || publication.position != search.last_ack {
            return Err(Stop::Observer);
        }
        search.events.push(search.pending.take().unwrap());
        state.budget.check(Instant::now())
    })();
    if result.is_err() {
        search.active = false;
    }
    state.search = Some(search);
    result
}
fn native_stop(s: Stop) -> NativeAdmissionFailure {
    match s {
        Stop::Deadline => NativeAdmissionFailure::Deadline,
        Stop::Bytes | Stop::Allocation => NativeAdmissionFailure::Bytes,
        Stop::Overflow => NativeAdmissionFailure::Overflow,
        _ => NativeAdmissionFailure::Work,
    }
}
struct ModelAdmission<'a>(&'a mut Budget);
impl provenance::Admission for ModelAdmission<'_> {
    fn reserve(&mut self, q: provenance::Quote) -> Result<(), provenance::Error> {
        self.0
            .reserve_checking(q.work, q.bytes, Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
    fn check(&mut self) -> Result<(), provenance::Error> {
        self.0
            .check(Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
}
impl checked_model::VerificationAdmission for ModelAdmission<'_> {
    fn addition(&mut self) -> Result<(), provenance::Error> {
        self.0
            .reserve_addition(Instant::now())
            .map_err(|_| provenance::Error::Admission)
    }
}
pub(in crate::ml_dataset::solvers) fn continue_search(
    initial: WithLedger<'_>,
    caps: SearchCaps,
) -> OwnedSearch<'_> {
    let mut owned = OwnedSearch {
        initial,
        model: None,
        verification: checked_model::Progress::default(),
        verdict: Verdict::Partial,
        first_stop: None,
        invoked: false,
        result_ack: None,
        native_result: None,
        terminal_ack: None,
        report: None,
    };
    if let Err(stop) = owned.run(caps) {
        let mut state = owned
            .initial
            .stage
            .shared
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let stop = state.budget.fail::<()>(stop).unwrap_err();
        if let Some(s) = state.search.as_mut() {
            s.active = false;
        }
        owned.first_stop = Some(stop);
    }
    owned.finish_search_report();
    owned
}
impl OwnedSearch<'_> {
    fn run(&mut self, caps: SearchCaps) -> Result<(), Stop> {
        let stage = &mut self.initial.stage;
        let original_deadline;
        {
            let mut state = stage.shared.lock().map_err(|_| Stop::Observer)?;
            state.budget.check(Instant::now())?;
            if self.invoked
                || caps.events == 0
                || caps.conflicts == 0
                || !state.closed
                || !state.sealed
                || state.search.is_some()
            {
                return Err(Stop::Phase);
            }
            let ack = self.initial.ack.as_ref().ok_or(Stop::Phase)?;
            if &state.sink.committed_event_position() != ack {
                return Err(Stop::Observer);
            }
            let map = stage.evidence.checked.as_ref().ok_or(Stop::Phase)?;
            if map.original_binding() != state.binding
                || map.pattern() != state.pattern
                || !stage.evidence.map_published
            {
                return Err(Stop::Phase);
            }
            if state.budget.phase != super::super::super::super::Phase::Setup {
                return Err(Stop::Phase);
            }
            let solver = stage.solver.as_ref().ok_or(Stop::Phase)?;
            original_deadline = state.budget.deadline;
            if solver.deadline != Some(original_deadline) {
                return Err(Stop::Deadline);
            }
            state.budget.reserve_checking(512, 2048, Instant::now())?;
            // Acknowledged checked installation finishes the SAT construction
            // stage. No limits, counters or original deadline are reset.
            state.budget.phase = super::super::super::super::Phase::Constructed;
            state.search = Some(SearchCapture {
                active: true,
                limit: caps.events,
                events: Vec::new(),
                pending: None,
                initial_ack: ack.clone(),
                last_ack: ack.clone(),
            });
        }
        let shared = Arc::clone(&stage.shared);
        let solver = stage.solver.as_mut().ok_or(Stop::Phase)?;
        solver.conflict_budget = solver.conflict_budget.min(caps.conflicts);
        solver
            .set_search_admission(move |q| {
                let mut state = shared
                    .lock()
                    .map_err(|_| NativeAdmissionFailure::UncoveredPath)?;
                state
                    .budget
                    .reserve_checking(q.work, q.requested_replacement_bytes, Instant::now())
                    .map_err(native_stop)
            })
            .map_err(|_| Stop::NativeFailure)?;
        self.invoked = true;
        let result = solver.solve();
        self.native_result = Some(match result {
            SolveResult::Sat => NativeOutcome::Sat,
            SolveResult::Unsat => NativeOutcome::UnsatUncertified,
            SolveResult::Unknown => NativeOutcome::Unknown,
        });
        {
            let mut state = stage.shared.lock().map_err(|_| Stop::Observer)?;
            if let Some(s) = state.search.as_mut() {
                s.active = false;
            }
            state.budget.check(Instant::now())?;
        }
        if solver.deadline != Some(original_deadline) {
            return Err(Stop::Deadline);
        }
        if solver.search_admission_failure().is_some() || solver.event_observer_error().is_some() {
            return Err(Stop::NativeFailure);
        }
        match result {
            SolveResult::Unknown => return Err(Stop::NativeFailure),
            SolveResult::Unsat => {
                self.verdict = Verdict::NativeUnsatUncertified;
                return Ok(());
            }
            SolveResult::Sat => {}
        }
        self.model = solver.admitted_model();
        let model = self.model.as_ref().ok_or(Stop::NativeFailure)?;
        let mut state = stage.shared.lock().map_err(|_| Stop::Observer)?;
        let map = stage.evidence.checked.as_ref().ok_or(Stop::Phase)?;
        state.budget.begin_extraction(false, Instant::now())?;
        checked_model::verify(
            map,
            model,
            &stage.evidence.math.signed,
            stage.evidence.math.signed_target,
            &mut self.verification,
            &mut ModelAdmission(&mut state.budget),
        )
        .map_err(|_| Stop::NativeFailure)?;
        state.budget.check(Instant::now())?;
        #[derive(Serialize)]
        struct ResultRecord<'a> {
            schema: &'static str,
            original_binding: [u8; 32],
            pattern: u32,
            initial_checkpoint_sequence: u64,
            initial_checkpoint_sha256: &'a str,
            search_end_sequence: u64,
            search_end_sha256: &'a str,
            model: &'a [bool],
            verification: &'a checked_model::Progress,
            scalar_recovered: bool,
            all_patterns_negative: bool,
        }
        let search = state.search.as_ref().ok_or(Stop::Phase)?;
        let last = state.sink.committed_event_position();
        if last != search.last_ack {
            return Err(Stop::Observer);
        }
        // Pay fixed receipt-string copies before creating stable borrowed record.
        state.budget.reserve_checking(512, 1024, Instant::now())?;
        let initial = self.initial.ack.as_ref().ok_or(Stop::Phase)?;
        let record = ResultRecord {
            schema: "binary-sat-search-result/v1",
            original_binding: state.binding,
            pattern: state.pattern,
            initial_checkpoint_sequence: initial.event_count,
            initial_checkpoint_sha256: &initial.last_event_sha256,
            search_end_sequence: last.event_count,
            search_end_sha256: &last.last_event_sha256,
            model,
            verification: &self.verification,
            scalar_recovered: false,
            all_patterns_negative: false,
        };
        let cap = state.limits.max_event_bytes as usize;
        let CaptureState {
            budget,
            sink,
            buffer,
            ..
        } = &mut *state;
        let result = publish_kind(
            capture::TypedEventKind::BinarySatSearch,
            budget,
            sink,
            buffer.as_mut().ok_or(Stop::Phase)?,
            cap,
            &record,
        );
        let actual = sink.committed_event_position();
        if actual != last {
            self.result_ack = Some(actual.clone());
        }
        let publication = result?;
        if !publication.valid || publication.position != actual {
            return Err(Stop::Observer);
        }
        budget.check(Instant::now())?;
        self.verdict = Verdict::CheckedSignedRelation;
        Ok(())
    }
    pub(in crate::ml_dataset::solvers) fn verdict(&self) -> Verdict {
        self.verdict
    }
    pub(in crate::ml_dataset::solvers) fn stop(&self) -> Option<Stop> {
        self.first_stop
    }
}

/// This is a producer report, not a central replay or export capability.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(in crate::ml_dataset::solvers) enum NativeOutcome {
    Sat,
    UnsatUncertified,
    Unknown,
}
#[derive(Clone, Copy, Debug, Serialize)]
pub(in crate::ml_dataset::solvers) struct SearchReport {
    pub invoked: bool,
    pub native_result: Option<NativeOutcome>,
    pub checked_signed_relation: bool,
    pub first_stop: Option<Stop>,
    pub terminal_stop: Option<Stop>,
    pub terminal_acknowledged: bool,
    pub terminal_accepted: bool,
    pub totals: [u64; 7],
}
fn search_totals(b: &Budget) -> [u64; 7] {
    [
        b.native.work,
        b.native.bytes,
        b.checking_work,
        b.checking_performed_work,
        b.adapter_bytes,
        b.trace_bytes,
        b.additions,
    ]
}
impl OwnedSearch<'_> {
    /// Called once on every return path. A stopped owner cannot pay for a new
    /// publication. We retain an in-memory report of unavailable custody instead
    /// of clearing its stop or extending its deadline to write a failure record.
    fn finish_search_report(&mut self) {
        let terminal_stop = self.publish_search_terminal().err();
        let mut state = self
            .initial
            .stage
            .shared
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(stop) = terminal_stop {
            let first = state.budget.fail::<()>(stop).unwrap_err();
            self.first_stop.get_or_insert(first);
        }
        self.report = Some(SearchReport {
            invoked: self.invoked,
            native_result: self.native_result,
            checked_signed_relation: self.verdict == Verdict::CheckedSignedRelation,
            first_stop: self.first_stop,
            terminal_stop,
            terminal_acknowledged: self.terminal_ack.is_some(),
            terminal_accepted: terminal_stop.is_none(),
            totals: search_totals(&state.budget),
        });
    }
    fn publish_search_terminal(&mut self) -> Result<(), Stop> {
        const CAP: u64 = 4096;
        let mut state = self
            .initial
            .stage
            .shared
            .lock()
            .map_err(|_| Stop::Observer)?;
        state.budget.check(Instant::now())?;
        if self.report.is_some() || self.terminal_ack.is_some() {
            return Err(Stop::Phase);
        }
        if CAP > state.limits.max_event_bytes as u64 {
            return Err(Stop::Bytes);
        }
        // Same bounded prepayment as the initial ledger, including receipt copies
        // and self-publication. No fresh Budget or WitnessBudget is introduced.
        state
            .budget
            .reserve_checking(CAP * 56 + 65536, 2048, Instant::now())?;
        state.budget.reserve_trace(CAP * 4 + 4096, Instant::now())?;
        let before = state.sink.committed_event_position();
        let expected = self
            .result_ack
            .as_ref()
            .or_else(|| state.search.as_ref().map(|s| &s.last_ack))
            .or(self.initial.ack.as_ref())
            .ok_or(Stop::Phase)?;
        if expected != &before {
            return Err(Stop::Observer);
        }
        let initial = self.initial.ack.as_ref().ok_or(Stop::Phase)?;
        #[derive(Serialize)]
        struct Terminal<'a> {
            schema: &'static str,
            original_binding: [u8; 32],
            pattern: u32,
            initial_checkpoint_sequence: u64,
            initial_checkpoint_sha256: &'a str,
            covered_sequence: u64,
            covered_sha256: &'a str,
            invoked: bool,
            native_result: Option<NativeOutcome>,
            checked_signed_relation: bool,
            first_stop: Option<Stop>,
            limits: [u64; 4],
            totals: [u64; 7],
            scalar_recovered: bool,
            all_patterns_negative: bool,
        }
        let b = &state.budget;
        let record = Terminal {
            schema: "binary-sat-search-terminal/v1",
            original_binding: state.binding,
            pattern: state.pattern,
            initial_checkpoint_sequence: initial.event_count,
            initial_checkpoint_sha256: &initial.last_event_sha256,
            covered_sequence: before.event_count,
            covered_sha256: &before.last_event_sha256,
            invoked: self.invoked,
            native_result: self.native_result,
            checked_signed_relation: self.verdict == Verdict::CheckedSignedRelation,
            first_stop: b.stopped,
            limits: [
                b.native_work_limit,
                b.memory_limit,
                b.trace_limit,
                b.addition_limit,
            ],
            totals: search_totals(b),
            scalar_recovered: false,
            all_patterns_negative: false,
        };
        let CaptureState { sink, buffer, .. } = &mut *state;
        let result = sink.event_typed_admit_encoding_with_limit(
            capture::TypedEventKind::BinarySatSearch,
            &record,
            buffer.as_mut().ok_or(Stop::Phase)?,
            CAP as usize,
            &mut |n| {
                if n as u64 <= CAP {
                    Ok(())
                } else {
                    Err(capture::CaptureError::Integrity(
                        "search terminal wire cap".into(),
                    ))
                }
            },
            &mut |bytes| {
                if bytes.len() as u64 <= CAP {
                    Ok(())
                } else {
                    Err(capture::CaptureError::Integrity(
                        "search terminal wire cap".into(),
                    ))
                }
            },
        );
        let actual = sink.committed_event_position();
        let valid = actual.event_count
            == before.event_count.checked_add(1).ok_or(Stop::Overflow)?
            && actual.binding_sha256 == before.binding_sha256
            && actual.byte_end > before.byte_end;
        // Capture the actual durable position even if the sink reports failure
        // after commit, or the original deadline expires immediately afterwards.
        if actual != before {
            self.terminal_ack = Some(actual);
        }
        result.map_err(|_| Stop::Observer)?;
        if !valid {
            return Err(Stop::Observer);
        }
        state.budget.check(Instant::now())?;
        Ok(())
    }
    pub(in crate::ml_dataset::solvers) fn search_report(&self) -> Option<&SearchReport> {
        self.report.as_ref()
    }
    pub(in crate::ml_dataset::solvers) fn search_terminal_ack(
        &self,
    ) -> Option<&capture::CommittedEventPosition> {
        self.terminal_ack.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn genuine_both_twist_owner_retained_at_closed_native_admission_prerequisite() {
        use crate::ml_dataset::{solvers, telemetry};
        for twist in [false, true] {
            let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
                23, twist, "sat",
            );
            execution::create(&f.root).unwrap();
            let mut registry = Vec::new();
            let case = telemetry::measured(&f.root, &mut registry, "curve-construction", || {
                crate::ml_dataset::curves::generate(&f.config)
            })
            .unwrap();
            let token = solvers::verify_and_store_worker_originals(
                &f.config,
                &case,
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
            let mut owner = match OriginalOwner::admit(&token, &case, &context, &sink, &limits) {
                Ok(owner) => owner,
                Err(AdmissionFailure::BeforeBudget(stage)) => {
                    panic!("requested profile original admission failed at {stage:?}")
                }
                Err(AdmissionFailure::Retained(frame)) => panic!(
                    "requested profile original admission failed at {:?}: {:?}",
                    frame.stage,
                    frame.first_stop()
                ),
            };
            let ownership = owner.budget.ownership_id;
            let deadline = owner.budget.deadline;
            // Keep original solver limits and original admitted pools. This
            // positive gate must fail, rather than accept refusal or raise caps,
            // if the composition cannot finish under that actual envelope.
            let gate_limits = provenance::Limits {
                variables: 100_000,
                codes: owner.math.codes.len(),
                domain_clauses: 1_000_000,
                work: owner.budget.native_work_limit,
                bytes: owner.budget.memory_limit,
            };
            let work_before_map = owner.budget.checking_work;
            owner
                .build_map(0, gate_limits)
                .expect("required requested-profile map construction");
            let work_after_map = owner.budget.checking_work;
            owner
                .persist_map(&mut sink)
                .unwrap_or_else(|stop| {
                    panic!(
                        "required requested-profile original map capture: {stop:?}; work before map={work_before_map}, after map={work_after_map}, after publication attempt={}, native work={}, limit={}, retained map bytes={}",
                        owner.budget.checking_work,
                        owner.budget.native.work,
                        owner.budget.native_work_limit,
                        owner.retained_bytes.len()
                    )
                });

            let map_position = owner.map_ack.as_ref().unwrap().clone();
            assert!(owner.map_published);
            assert_eq!(map_position, sink.committed_event_position());
            let checker_limits = provenance::initial::Limits {
                work: owner.budget.native_work_limit,
                bytes: owner.budget.memory_limit,
                events: 100_000,
                constraints: 1_000_000,
            };
            let stage = match NativeInitial::prepare(owner, sink, checker_limits) {
                Ok(stage) => stage,
                Err(Failure::BeforeNative { stop, .. }) => {
                    panic!("required requested-profile native admission: {stop:?}")
                }
                Err(Failure::Retained { stop, .. }) => {
                    panic!("required requested-profile native initial capture/check: {stop:?}")
                }
            };

            let stage = stage.detach();
            let initial = match super::super::finish(stage) {
                Ok(v) => v,
                Err(f) => panic!("initial ledger: {:?}", f.stop),
            };
            let owned = continue_search(
                initial,
                SearchCaps {
                    events: 100_000,
                    conflicts: 100_000,
                },
            );
            assert_eq!(owned.verdict(), Verdict::Partial);
            assert_eq!(owned.stop(), Some(Stop::NativeFailure));
            assert!(owned.invoked);
            let report = owned.search_report().expect("every outcome retains report");
            assert!(report.invoked);
            assert_eq!(report.native_result, Some(NativeOutcome::Unknown));
            assert!(!report.terminal_accepted);
            assert!(!report.terminal_acknowledged);
            assert_eq!(report.first_stop, Some(Stop::NativeFailure));
            assert_eq!(report.terminal_stop, Some(Stop::NativeFailure));
            assert!(owned.search_terminal_ack().is_none());
            let state = owned.initial.stage.shared.lock().unwrap();
            assert_eq!(state.budget.ownership_id, ownership);
            assert_eq!(state.budget.deadline, deadline);
            assert!(state.closed && state.sealed);
            assert!(owned
                .initial
                .stage
                .solver
                .as_ref()
                .unwrap()
                .search_admission_failure()
                .is_some());
            assert!(state.search.as_ref().unwrap().events.is_empty());
            assert!(!state.search.as_ref().unwrap().active);
            assert_eq!(
                state.sink.committed_event_position(),
                *owned.initial.ack.as_ref().unwrap()
            );
        }
    }
}

pub(in crate::ml_dataset::solvers) struct ReportedSearch {
    pub owner:OwnedSearch<'static>, report_admitted:bool, report_taken:bool,
}
pub(in crate::ml_dataset::solvers) fn continue_reported(initial:WithLedger<'static>,caps:SearchCaps)->ReportedSearch {
    let admitted={let mut state=initial.stage.shared.lock().unwrap_or_else(|e|e.into_inner());
        state.budget.reserve_checking(16*4096+4096,65536,Instant::now()).is_ok()};
    // A failed reservation is sticky: continue_search retains it and cannot solve.
    ReportedSearch{owner:continue_search(initial,caps),report_admitted:admitted,report_taken:false}
}
impl ReportedSearch {
    pub(in crate::ml_dataset::solvers) fn outcome(&mut self)->Option<serde_json::Value>{
        if !self.report_admitted||self.report_taken{return None} self.report_taken=true;
        let report=self.owner.search_report()?;
        let ack=self.owner.search_terminal_ack();
        let value=serde_json::json!({"adapter_id":"sat-search-v1","strategy":"sat-search-v1","status":"partial",
            "solver_completed":false,"scalar_recovered_without_witness":false,"all_patterns_negative":false,
            "search_report":report,"terminal_sequence":ack.map(|a|a.event_count),
            "terminal_sha256":ack.map(|a|a.last_event_sha256.as_str())});
        assert!(super::super::super::super::worker_terminal_encoded_size(&value,4096).is_ok(),"paid search parent schema cap");
        Some(value)
    }
}

#[cfg(all(test, sat_qualification_child))]
#[path = "sat_search_qualification.rs"]
mod qualification;
