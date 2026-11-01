//! Closed ordinary ownership/capture prerequisite. No dispatch or central replay.
use super::*;
#[path = "binary_m3_replay.rs"]
pub(in crate::ml_dataset::solvers) mod replay;
#[path = "binary_m3_storage.rs"]
mod retained_storage;
#[path = "binary_m3_signed.rs"]
mod signed_stage;
#[path = "binary_m3_worker.rs"]
pub(in crate::ml_dataset::solvers) mod worker;
use crate::cryptanalysis::koblitz_groebner::bounded_matrix as native;
use sha2::Digest;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    Construction,
    Input,
    Events,
    Result,
    GroupReplay,
    ParentDelivery,
    Witness,
}
impl Slot {
    const ALL: [Self; 7] = [
        Self::Construction,
        Self::Input,
        Self::Events,
        Self::Result,
        Self::GroupReplay,
        Self::ParentDelivery,
        Self::Witness,
    ];
    fn identity(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Construction => (super::super::PATH, super::super::ROLE, "json"),
            Self::Input => (INPUT_PATH, INPUT_ROLE, "json"),
            Self::Events => (
                "binary-m3-event-index.json",
                "restricted-binary-m3-native-event-index/v1",
                "json",
            ),
            Self::Result => (
                "binary-m3-result.json",
                "restricted-binary-m3-result/v1",
                "json",
            ),
            Self::GroupReplay => (
                "binary-m3-group-replay.json",
                "restricted-binary-m3-group-replay/v1",
                "json",
            ),
            Self::ParentDelivery => (
                "binary-m3-parent-delivery.json",
                "restricted-binary-m3-parent-delivery/v1",
                "json",
            ),
            Self::Witness => ("pdp-witness.json", "independently-verified-pdp", "json"),
        }
    }
    fn matches(self, artifact: &types::Artifact) -> bool {
        let (path, role, encoding) = self.identity();
        (artifact.path == path && artifact.role == role && artifact.encoding == encoding)
            || (self == Self::Events
                && artifact.path == "binary-m3-events.jsonl"
                && artifact.role == "restricted-binary-m3-native-boundaries/v1"
                && artifact.encoding == "jsonl")
    }
}

/// Reserve BEFORE construction. The registry does not borrow/reset the budget;
/// its pointer is used only for identity comparison, never dereferenced.
pub(super) struct OrdinaryRootRegistry<'o> {
    pub(super) original: &'o VerifiedWorkerOriginal<'o>,
    ledger: *const Budget,
    receipts: solvers::WorkerStoredReceipts<'o>,
    input_registered: bool,
    input_identity_used: bool,
    slots: Option<retained_storage::RetainedSlots>,
}
impl<'o> OrdinaryRootRegistry<'o> {
    pub(super) fn admit(
        original: &'o VerifiedWorkerOriginal<'o>,
        ledger: &mut Budget,
        limits: &capture::CaptureLimits,
    ) -> Result<Self, String> {
        if ledger.phase != Phase::Setup
            || original.binding.is_none()
            || original.config.family != "binary"
        {
            return Err("ordinary root registry requires original setup context".into());
        }
        // Input encoding owns an immutable, charged identity. Adoption checks
        // that token and the actual root receipt without another parse/hash.
        ledger
            .reserve_checking(1024, 1024, Instant::now())
            .map_err(|s| format!("ordinary sealed identity comparison admission: {s:?}"))?;
        let receipts =
            solvers::WorkerStoredReceipts::admit(original, Slot::ALL.len(), |work, bytes| {
                ledger
                    .reserve_checking(work, bytes, Instant::now())
                    .map_err(|s| format!("ordinary receipt admission: {s:?}"))
            })?;
        // Fixed storage quotes must use the actual installed original sink's
        // limits; a caller cannot buy cheaper terminal metadata allowances.
        solvers::LIVE_SINK.with(|slot| match slot.borrow().as_ref() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&original.root.join("live"))?;
                sink.verify_attempt_limits(original.binding.expect("checked binding"), limits)
            }
            _ => Err("ordinary registry requires original bound sink".into()),
        })?;
        let slots = retained_storage::RetainedSlots::admit(ledger, limits)
            .map_err(|s| format!("ordinary terminal storage admission: {s:?}"))?;
        Ok(Self {
            original,
            ledger,
            receipts,
            slots: Some(slots),
            input_registered: false,
            input_identity_used: false,
        })
    }
    pub(super) fn adopt_input<'a>(
        &mut self,
        step: OrdinaryMatrixStep<'a>,
    ) -> Result<RegisteredMatrixInput<'a>, (String, OrdinaryMatrixStep<'a>)> {
        let identity = (|| -> Result<[u8; 32], String> {
            let input = &step.input;
            if self.input_registered
                || self.input_identity_used
                || !std::ptr::eq(self.ledger, input.ledger as *const Budget)
                || !input.bytes.origin_matches(input.ledger)
                || !Slot::Input.matches(&input.root_artifact)
                || input.bytes.len() > INPUT_CAP
            {
                return Err("ordinary input owner/slot mismatch or duplicate transfer".into());
            }
            self.input_identity_used = true;
            let input_sha256 = input.bytes.matches(self.original, input.ledger)?;
            if input.root_artifact.bytes != input.bytes.len() as u64
                || input.root_artifact.sha256 != hex::encode(input_sha256)
            {
                return Err("ordinary root/input differs from sealed bytes".into());
            }
            Ok(input_sha256)
        })();
        let input_sha256 = match identity {
            Ok(digest) => digest,
            Err(error) => return Err((error, step)),
        };
        let OrdinaryMatrixStep {
            input,
            report,
            adapter_stop,
        } = step;
        let PersistedOrdinaryInput {
            retained,
            bytes,
            root_artifact,
            capture_sequence,
            math,
            ledger,
        } = input;
        if let Err((error, root_artifact)) = self.receipts.push(root_artifact) {
            return Err((
                error,
                OrdinaryMatrixStep {
                    input: PersistedOrdinaryInput {
                        retained,
                        bytes,
                        root_artifact,
                        capture_sequence,
                        math,
                        ledger,
                    },
                    report,
                    adapter_stop,
                },
            ));
        }
        self.input_registered = true;
        Ok(RegisteredMatrixInput {
            input_sha256,
            retained,
            bytes: bytes.into_bytes(),
            capture_sequence,
            math,
            ledger,
            report,
            adapter_stop,
        })
    }
    /// A persistence failure can already own a durable root when its final
    /// deadline check refused. Transfer that actual receipt without requiring
    /// fresh budget or discarding the remaining partial failure evidence.
    pub(super) fn adopt_persistence_failure(
        &mut self,
        failed: &mut PersistenceFailure<'_>,
    ) -> Result<(), String> {
        if !std::ptr::eq(self.ledger, std::ptr::from_ref(&*failed.ledger)) {
            return Err("ordinary persistence failure belongs to another ledger".into());
        }
        if !failed.input_bytes.origin_matches(failed.ledger) {
            return Err("ordinary persistence bytes belong to another ledger".into());
        }
        if self.input_registered || self.input_identity_used {
            return Err("ordinary input already registered or identity allowance used".into());
        }
        let artifact = failed
            .root_artifact
            .as_ref()
            .ok_or("no acknowledged ordinary input root")?;
        if !failed.encoding_complete
            || failed.capture_sequence.is_none()
            || failed.input_bytes.len() > INPUT_CAP
            || !Slot::Input.matches(artifact)
        {
            return Err("ordinary failure root lacks complete persisted input".into());
        }
        self.input_identity_used = true;
        let digest = failed.input_bytes.matches(self.original, failed.ledger)?;
        if artifact.bytes != failed.input_bytes.len() as u64
            || artifact.sha256 != hex::encode(digest)
        {
            return Err("ordinary failure root differs from sealed input".into());
        }
        let artifact = failed.root_artifact.take().expect("checked root owner");
        if let Err((error, artifact)) = self.receipts.push(artifact) {
            failed.root_artifact = Some(artifact);
            return Err(error);
        }
        self.input_registered = true;
        Ok(())
    }
    // Future orchestrator consumes this registry with its completed/failed data;
    // actual root receipts move directly into the existing worker transport.
    pub(super) fn finish(self, data: solvers::SolverData) -> OrdinaryWorkerOutput {
        OrdinaryWorkerOutput {
            worker: self.receipts.finish(data),
            retained_slots: self.slots,
        }
    }
}
/// Handoff keeps incomplete encodings, rejected roots and authoritative bytes
/// alive alongside accepted worker roots. The orchestrator must retain both.
pub(super) struct OrdinaryWorkerOutput {
    worker: solvers::WorkerSolveOutput,
    retained_slots: Option<retained_storage::RetainedSlots>,
}
/// Root custody is now exclusively in the registry. All other actual ownership
/// stays here; no constructor creates this state without successful transfer.
pub(super) struct RegisteredMatrixInput<'a> {
    input_sha256: [u8; 32],
    retained: Retained,
    bytes: Vec<u8>,
    capture_sequence: u64,
    math: MathInput,
    ledger: &'a mut Budget,
    report: Option<native::Report>,
    adapter_stop: Option<Stop>,
}

const EVENT_CAP: usize = 2048;
const SIGNED_EVENT_CAP: usize = 1024;
#[derive(Default, Debug, Serialize)]
struct EventAccounting {
    attempted: u64,
    acknowledged: u64,
    reserved_checking_work: u64,
    reserved_trace_bytes: u64,
    acknowledged_wire_bytes: u64,
    reserved_retention_work: u64,
    reserved_retention_bytes: u64,
    reserved_retention_trace: u64,
}
#[derive(Serialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
enum OrdinaryBoundary<'a> {
    Matrix { snapshot: &'a native::Event },
    Signed { prefix: &'a SignedPrefix },
}
#[derive(Serialize)]
struct OrdinaryEventView<'a> {
    schema: &'static str,
    binding_sha256: &'a str,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    sequence: u64,
    capture_sequence: u64,
    event: OrdinaryBoundary<'a>,
}

pub(super) struct OrdinaryEvents<'o> {
    original: &'o VerifiedWorkerOriginal<'o>,
    limits: &'o capture::CaptureLimits,
    directory: std::path::PathBuf,
    ledger: *const Budget,
    binding_sha256: String,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    buffer: capture::TypedEventBuffer,
    accounting: EventAccounting,
    // Only a complete unacknowledged event; acknowledged bytes remain in live capture.
    retained_wire: Vec<u8>,
    start_position: capture::CommittedEventPosition,
    end_position: capture::CommittedEventPosition,
    acknowledged_prefix_len: usize,
    encoding_attempted: bool,
    current_encoding_retained: bool,
    #[cfg(test)]
    refuse_precommit: bool,
}
impl<'o> OrdinaryEvents<'o> {
    pub(super) fn admit(
        original: &'o VerifiedWorkerOriginal<'o>,
        limits: &'o capture::CaptureLimits,
        input: &mut PersistedOrdinaryInput<'_>,
    ) -> Result<Self, String> {
        if input.ledger.phase != Phase::Constructed || original.root.as_os_str().len() > 4096 {
            return Err(
                "ordinary events require persisted construction and bounded directory".into(),
            );
        }
        if input.bytes.len() > INPUT_CAP {
            let _ = input.ledger.fail::<()>(Stop::InvalidLimits);
            return Err("ordinary event input exceeds original cap".into());
        }
        if !input.bytes.origin_matches(input.ledger) {
            return Err("ordinary event bytes belong to another ledger".into());
        }
        let requested = capture::TypedEventBuffer::requested_bytes(EVENT_CAP as u64)
            .map_err(|e| format!("{e:?}"))?;
        input
            .ledger
            .reserve_checking(1024, requested + 8192, Instant::now())
            .map_err(|s| format!("ordinary event admission: {s:?}"))?;
        let input_sha256 = input.bytes.matches(original, input.ledger)?;
        let directory = original.root.join("live");
        let binding = original.binding.ok_or("ordinary events require binding")?;
        let binding_sha256 = binding.digest()?;
        let start_position = solvers::LIVE_SINK.with(|s| match s.borrow().as_ref() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&directory)?;
                sink.verify_attempt_limits(binding, limits)?;
                Ok::<_, String>(sink.committed_event_position())
            }
            _ => Err("ordinary events require bound sink".into()),
        })?;
        let buffer = capture::TypedEventBuffer::try_new(EVENT_CAP as u64, requested)
            .map_err(|e| format!("{e:?}"))?;
        Ok(Self {
            original,
            limits,
            directory,
            ledger: input.ledger,
            binding_sha256,
            input_sha256,
            input_capture_sequence: input.capture_sequence,
            buffer,
            accounting: EventAccounting::default(),
            retained_wire: Vec::new(),
            end_position: start_position.clone(),
            start_position,
            acknowledged_prefix_len: 0,
            encoding_attempted: false,
            current_encoding_retained: false,
            #[cfg(test)]
            refuse_precommit: false,
        })
    }
    #[cfg(test)]
    pub(super) fn refuse_precommit_for_test(&mut self) {
        self.refuse_precommit = true;
    }

    pub(super) fn take_unretained_encoding(&mut self) -> Vec<u8> {
        if self.encoding_attempted && !self.current_encoding_retained {
            self.encoding_attempted = false;
            self.buffer.take_encoded()
        } else {
            Vec::new()
        }
    }

    pub(super) fn forward(&mut self, event: &native::Event, ledger: &mut Budget) -> Result<(), ()> {
        self.forward_boundary(OrdinaryBoundary::Matrix { snapshot: event }, ledger)
    }

    fn forward_signed(&mut self, prefix: &SignedPrefix, ledger: &mut Budget) -> Result<(), Stop> {
        let acknowledged_before = self.accounting.acknowledged;
        let result = self.forward_boundary(OrdinaryBoundary::Signed { prefix }, ledger);
        // A durable acknowledgement remains true across a post-commit deadline.
        // The signed enumerator retains the event before its next clock check.
        if self.accounting.acknowledged == acknowledged_before + 1 {
            Ok(())
        } else {
            result.map_err(|()| ledger.stopped.unwrap_or(Stop::Observer))
        }
    }

    fn forward_boundary(
        &mut self,
        event: OrdinaryBoundary<'_>,
        ledger: &mut Budget,
    ) -> Result<(), ()> {
        self.encoding_attempted = false;
        self.current_encoding_retained = false;
        let expected_phase = match &event {
            OrdinaryBoundary::Matrix { .. } => Phase::Matrix,
            OrdinaryBoundary::Signed { .. } => Phase::Extraction,
        };
        if !std::ptr::eq(self.ledger, ledger as *const Budget) || ledger.phase != expected_phase {
            let _ = ledger.fail::<()>(Stop::Phase);
            return Err(());
        }
        // Matrix callback count remains native-owned; the combined stream can
        // additionally contain at most the actual 192 signed additions.
        let phase_limit = match &event {
            OrdinaryBoundary::Matrix { .. } => self.accounting.attempted >= native::MAX_EVENTS,
            OrdinaryBoundary::Signed { prefix } => prefix.sequence >= 192,
        };
        if phase_limit || self.accounting.attempted >= native::MAX_EVENTS + 192 {
            let _ = ledger.fail::<()>(Stop::Events);
            return Err(());
        }
        let cap = match &event {
            OrdinaryBoundary::Matrix { .. } => EVENT_CAP,
            OrdinaryBoundary::Signed { .. } => SIGNED_EVENT_CAP,
        };
        self.accounting.attempted += 1;
        // Prepay counting; admit the second pass from actual size before encoding.
        // Keep these reservations distinct from actual acknowledged wire bytes
        // and unchanged native operation counters. No refunds after refusal.
        ledger
            .reserve_checking(8 * cap as u64, 0, Instant::now())
            .map_err(|_| ())?;
        self.accounting.reserved_checking_work += 8 * cap as u64;
        ledger
            .reserve_trace(cap as u64, Instant::now())
            .map_err(|_| ())?;
        self.accounting.reserved_trace_bytes += cap as u64;
        ledger
            .reserve_checking(1024, 128, Instant::now())
            .map_err(|_| ())?;
        // Fixed file overhead remains prepaid. Length-dependent ownership and
        // storage credits are admitted from actual encoded bytes in precommit,
        // before allocation/copy and before any durable acknowledgement.
        let result = solvers::LIVE_SINK.with(|slot| -> Result<(), String> {
            let mut slot = slot.borrow_mut();
            let Some(solvers::EventSink::Bound(sink)) = slot.as_mut() else {
                return Err("ordinary event sink disappeared".into());
            };
            let binding = self
                .original
                .binding
                .ok_or("ordinary event binding absent")?;
            sink.verify_directory(&self.directory)?;
            sink.verify_attempt_limits(binding, self.limits)?;
            let capture_sequence = sink
                .committed_event_count()
                .checked_add(1)
                .ok_or("ordinary capture overflow")?;
            let view = OrdinaryEventView {
                schema: "crypto.ml-dataset.binary-m3-events/v1",
                binding_sha256: &self.binding_sha256,
                input_sha256: self.input_sha256,
                input_capture_sequence: self.input_capture_sequence,
                sequence: self.accounting.acknowledged + 1,
                capture_sequence,
                event,
            };
            // ready() can refuse before the emitter clears its buffer. Never
            // relabel a previous acknowledged encoding as this call's tail.
            self.buffer.clear_encoded();
            self.encoding_attempted = true;
            let shared_ledger = std::cell::RefCell::new(&mut *ledger);
            sink.event_typed_admit_encoding_with_limit(
                capture::TypedEventKind::OrdinaryBinaryM3Native,
                &view,
                &mut self.buffer,
                cap,
                &mut |expected| {
                    let work = 8 * expected as u64;
                    shared_ledger
                        .borrow_mut()
                        .reserve_checking(work, 0, Instant::now())
                        .map_err(|s| {
                            capture::CaptureError::Integrity(format!(
                                "ordinary encoding admission: {s:?}"
                            ))
                        })?;
                    self.accounting.reserved_checking_work += work;
                    Ok(())
                },
                &mut |_wire| {
                    // Buffer ownership already paid during event admission. No
                    // duplicate acknowledged-wire allocation, hash or root write.
                    self.current_encoding_retained = true;
                    #[cfg(test)]
                    if self.refuse_precommit {
                        return Err(capture::CaptureError::Integrity(
                            "injected refusal after actual encoding, before durable commit".into(),
                        ));
                    }
                    shared_ledger
                        .borrow_mut()
                        .check(Instant::now())
                        .map_err(|s| {
                            capture::CaptureError::Integrity(format!(
                                "ordinary event deadline: {s:?}"
                            ))
                        })
                },
            )
            .map_err(|e| format!("{e:?}"))?;
            self.accounting.acknowledged += 1;
            self.acknowledged_prefix_len += self.buffer.encoded_len();
            self.end_position = sink.committed_event_position();
            self.accounting.acknowledged_wire_bytes += self.buffer.encoded_len() as u64;
            Ok(())
        });
        if result.is_err() {
            if self.current_encoding_retained {
                self.retained_wire = self.buffer.take_encoded();
            }
            let _ = ledger.observer_failed();
            return Err(());
        }
        ledger.check(Instant::now()).map_err(|_| ())
    }
}

#[cfg(test)]
pub(super) fn test_transfer(
    registry: &mut OrdinaryRootRegistry<'_>,
    step: OrdinaryMatrixStep<'_>,
    root_ptr: *const u8,
    input_ptr: *const u8,
    mut events: OrdinaryEvents<'_>,
) {
    let stop = step.adapter_stop;
    if stop == Some(Stop::Deadline) {
        // Exercise the failure-owner transfer with an ACTUAL previously stored
        // root. This constructs a failure envelope, not a new storage receipt
        // or a claim that a timed storage failure was observed in this test.
        let PersistedOrdinaryInput {
            retained,
            bytes,
            root_artifact,
            capture_sequence,
            ledger,
            ..
        } = step.input;
        let mut other_ledger = Budget::new(
            registry.original.config.work_limit,
            ledger.memory_limit,
            ledger.trace_limit,
            ledger.deadline,
            Instant::now(),
        )
        .unwrap();
        let mut failed = PersistenceFailure {
            retained,
            input_bytes: bytes,
            encoding_complete: true,
            capture_sequence: Some(capture_sequence),
            root_artifact: Some(root_artifact),
            stop,
            error: "failure-owner transfer control".into(),
            ledger: &mut other_ledger,
        };
        // Same authenticated attempt and actual root are insufficient: origin
        // must retain the same exclusive cumulative budget borrower.
        assert!(registry.adopt_persistence_failure(&mut failed).is_err());
        assert!(!registry.input_identity_used);
        assert!(!registry.input_registered);
        assert!(registry.receipts.records.is_empty());
        assert_eq!(
            failed.root_artifact.as_ref().unwrap().path.as_ptr(),
            root_ptr
        );
        assert_eq!(failed.input_bytes.as_ptr(), input_ptr);
        assert_eq!(failed.stop, stop);
        failed.ledger = ledger;
        registry.adopt_persistence_failure(&mut failed).unwrap();
        assert!(failed.root_artifact.is_none());
        assert_eq!(failed.input_bytes.as_ptr(), input_ptr);
        assert_eq!(failed.stop, stop);
        assert_eq!(registry.receipts.records[0].path.as_ptr(), root_ptr);
        assert!(registry.adopt_persistence_failure(&mut failed).is_err());
        assert_eq!(registry.receipts.records.len(), 1);
        return;
    }
    // Rejection returns the entire original owner, even after a sticky stop.
    registry.input_registered = true;
    let (_, mut step) = match registry.adopt_input(step) {
        Err(e) => e,
        Ok(_) => panic!("duplicate accepted"),
    };
    assert_eq!(step.input.bytes.as_ptr(), input_ptr);
    assert_eq!(step.input.root_artifact.path.as_ptr(), root_ptr);
    registry.input_registered = false;
    let actual_path = std::mem::replace(
        &mut step.input.root_artifact.path,
        "koblitz-m3-input.json".into(),
    );
    let (_, mut step) = match registry.adopt_input(step) {
        Err(e) => e,
        Ok(_) => panic!("cross-family slot accepted"),
    };
    step.input.root_artifact.path = actual_path;
    let mut registered = match registry.adopt_input(step) {
        Ok(x) => x,
        Err((e, _)) => panic!("{e}"),
    };
    assert_eq!(registered.bytes.as_ptr(), input_ptr);
    assert_eq!(registered.adapter_stop, stop);
    assert_eq!(registry.receipts.records.len(), 1);
    assert_eq!(registry.receipts.records[0].path.as_ptr(), root_ptr);
    assert_eq!(
        registry.receipts.records[0].bytes,
        registered.bytes.len() as u64
    );
    assert!(registry.input_registered);
    retained_storage::test_store(registry, &mut registered, &mut events);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closed_ordinary_catalog_is_unique_and_never_koblitz() {
        let mut paths = std::collections::BTreeSet::new();
        for slot in Slot::ALL {
            let (path, role, encoding) = slot.identity();
            assert!(paths.insert(path));
            assert!(!path.contains("koblitz"));
            assert!(!role.contains("koblitz"));
            assert!(matches!(encoding, "json" | "jsonl"));
        }
        assert_eq!(paths.len(), 7);
    }
    #[test]
    fn closed_signed_envelope_uses_real_capture_encoder_at_maximum_width() {
        let prefix = SignedPrefix {
            sequence: u16::MAX,
            tuple: u8::MAX,
            indices: [u8::MAX; 3],
            summand: u8::MAX,
            prefix: Some((u64::MAX, u64::MAX)),
            matches_target: false,
        };
        let binding = "f".repeat(64);
        let view = OrdinaryEventView {
            schema: "crypto.ml-dataset.binary-m3-events/v1",
            binding_sha256: &binding,
            input_sha256: [u8::MAX; 32],
            input_capture_sequence: u64::MAX,
            sequence: u64::MAX,
            capture_sequence: u64::MAX,
            event: OrdinaryBoundary::Signed { prefix: &prefix },
        };
        let wire = capture::maximum_sequence_typed_wire(
            capture::TypedEventKind::OrdinaryBinaryM3Native,
            &view,
        );
        assert!(wire.len() <= SIGNED_EVENT_CAP);
        let value: serde_json::Value = serde_json::from_slice(&wire).unwrap();
        assert_eq!(value["event"]["kind"], "binary_m3_native");
        assert_eq!(value["event"]["payload"]["event"]["phase"], "signed");
    }

    #[test]
    fn closed_matrix_envelope_worst_width_fits_cap() {
        let c = native::Counters {
            main_attempts: u64::MAX,
            nonempty_rows: u64::MAX,
            selected_rows: u64::MAX,
            pruned_nonempty_rows: u64::MAX,
            criterion_attempts: u64::MAX,
            criterion_rows: u64::MAX,
            criterion_zero_reductions: u64::MAX,
            koszul_pruned: u64::MAX,
            frobenius_pruned: u64::MAX,
            criterion_word_xors: u64::MAX,
            reduction_word_xors: u64::MAX,
            attempted_events: u64::MAX,
            acknowledged_events: u64::MAX,
            dynamic_layout_bytes: u64::MAX,
            allocation_requests: u64::MAX,
        };
        for stage in [
            native::Stage::Begin,
            native::Stage::Pack,
            native::Stage::Reduce,
            native::Stage::Readback,
            native::Stage::Finish,
            native::Stage::CriterionLevel { level: u32::MAX },
            native::Stage::CriterionCandidate {
                generator: usize::MAX,
                multiplier: u8::MAX,
                frobenius: false,
            },
            native::Stage::MainCandidate {
                generator: usize::MAX,
                multiplier: u8::MAX,
            },
        ] {
            let event = native::Event {
                stage,
                reserved: native::Reservation {
                    work: u64::MAX,
                    bytes: u64::MAX,
                },
                counters: c,
            };
            let digest = "f".repeat(64);
            let view = OrdinaryEventView {
                schema: "crypto.ml-dataset.binary-m3-events/v1",
                binding_sha256: &digest,
                input_sha256: [255; 32],
                input_capture_sequence: u64::MAX,
                sequence: u64::MAX,
                capture_sequence: u64::MAX,
                event: OrdinaryBoundary::Matrix { snapshot: &event },
            };
            let bytes = capture::maximum_sequence_typed_wire(
                capture::TypedEventKind::OrdinaryBinaryM3Native,
                &view,
            );
            assert!(bytes.len() <= EVENT_CAP);
            assert!(!String::from_utf8(bytes).unwrap().contains("koblitz"));
        }
    }
}
