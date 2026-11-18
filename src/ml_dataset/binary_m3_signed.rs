//! Ordinary signed stage in the complete worker/replay draft.
//! No independent budget, synthetic event, or Koblitz artifact identity.
use super::*;

pub(super) struct OrdinarySignedOwned<'a> {
    signed: Option<SignedEnumeration>,
    publisher: BoundSignedPublisher<'a>,
    stop: Option<Stop>,
    ledger: SavedBudgetLedger,
    group: retained_storage::OwnedSlot,
    witness: retained_storage::OwnedSlot,
    origin_ledger: *const Budget,
}

const GROUP_CAP: usize = 49152;
impl<'a> OrdinarySignedOwned<'a> {
    pub(super) fn admit(
        binding: &'a capture::AttemptBinding,
        limits: &'a capture::CaptureLimits,
        ledger: &mut Budget,
    ) -> Result<Self, Stop> {
        if ledger.phase != Phase::Setup {
            return ledger.fail(Stop::Phase);
        }
        let make = |slot| retained_storage::OwnedSlot {
            slot,
            bytes: Vec::new(),
            encoding_complete: false,
            write_attempted: false,
            root: None,
            transferred: false,
            error: None,
        };
        let mut group = make(Slot::GroupReplay);
        let witness = make(Slot::Witness);
        for cap in [GROUP_CAP, SIGNED_WITNESS_BYTE_CAP] {
            let quote = quote_parent_storage_length(cap as u64, false, limits)
                .or_else(|stop| ledger.fail(stop))?;
            ledger.reserve_checking(quote.work, quote.bytes, Instant::now())?;
            ledger.reserve_trace(quote.trace, Instant::now())?;
        }
        ledger.reserve_checking(
            8 * GROUP_CAP as u64,
            GROUP_CAP as u64 + 4096,
            Instant::now(),
        )?;
        if group.bytes.try_reserve_exact(GROUP_CAP).is_err() {
            return ledger.fail(Stop::Allocation);
        }
        Ok(Self {
            signed: None,
            publisher: BoundSignedPublisher::new(binding, limits, ledger),
            stop: None,
            ledger: ledger.saved_ledger(),
            group,
            witness,
            origin_ledger: ledger,
        })
    }

    pub(super) fn store(
        &mut self,
        input: &mut RegisteredMatrixInput<'_>,
        registry: &mut OrdinaryRootRegistry<'_>,
    ) -> Result<(), String> {
        if !std::ptr::eq(self.origin_ledger, registry.ledger)
            || !std::ptr::eq(self.origin_ledger, input.ledger as *const Budget)
            || registry.original.binding != Some(self.publisher.binding)
            || self.group.write_attempted
            || self.witness.write_attempted
        {
            return Err("ordinary signed owner/ledger/binding or repeated storage".into());
        }
        if let Some(signed) = &self.signed {
            #[derive(Serialize)]
            struct View<'s> {
                schema: &'static str,
                binding: &'s capture::AttemptBinding,
                public_sha256: String,
                factor_base_sha256: String,
                input_sha256: String,
                addition_limit: u64,
                performed_additions: u64,
                acknowledged_additions: usize,
                acknowledged: &'s [SignedPrefix],
                unacknowledged: &'s Option<SignedPrefix>,
            }
            let group = View {
                schema: "crypto.ml-dataset.binary-m3-signed-prefix/v1",
                binding: self.publisher.binding,
                public_sha256: hex::encode(registry.original.public_sha256),
                factor_base_sha256: hex::encode(registry.original.factor_base_sha256),
                input_sha256: hex::encode(input.input_sha256),
                addition_limit: input.ledger.original_limits().additions,
                performed_additions: signed.enumeration_additions,
                acknowledged_additions: signed.acknowledged.len(),
                acknowledged: &signed.acknowledged,
                unacknowledged: &signed.unacknowledged,
            };
            retained_storage::encode(&mut self.group, GROUP_CAP, &group);
            self.group.store(registry.original, &mut registry.receipts);
        }
        // No normal witness root from a failed/unacknowledged publication.
        // The bytes remain owned regardless of receipt or root-write outcome.
        if let Some(encoding) = self.publisher.encoding.as_mut() {
            self.witness.bytes = std::mem::take(&mut encoding.bytes);
            self.witness.encoding_complete = encoding.complete && self.publisher.receipt.is_some();
            if self.witness.encoding_complete {
                self.witness
                    .store(registry.original, &mut registry.receipts);
            }
        }
        if self.group.error.is_some() || self.witness.error.is_some() {
            let _ = input.ledger.observer_failed();
            return Err("ordinary signed roots incomplete; owners retained".into());
        }
        Ok(())
    }
}

impl RegisteredMatrixInput<'_> {
    pub(super) fn run_signed<'a>(
        &mut self,
        events: &mut OrdinaryEvents<'_>,
        mut owned: OrdinarySignedOwned<'a>,
    ) -> OrdinarySignedOwned<'a> {
        let binding = owned.publisher.binding;
        let limits = owned.publisher.limits;
        let result = (|| -> Result<(), Stop> {
            // Reject a prepaid owner from another cumulative ledger BEFORE
            // setup, native additions, capture or publication can occur.
            if !std::ptr::eq(owned.origin_ledger, self.ledger as *const Budget) {
                return self.ledger.fail(Stop::Phase);
            }
            self.ledger.check(Instant::now())?;
            if self.ledger.phase != Phase::MatrixComplete
                || !self.report.as_ref().is_some_and(|r| r.native_step_complete)
                || !std::ptr::eq(events.ledger, self.ledger as *const Budget)
                || events.input_sha256 != self.input_sha256
                || events.input_capture_sequence != self.capture_sequence
                || events.original.binding != Some(binding)
            {
                return self.ledger.fail(Stop::Phase);
            }
            self.ledger.reserve_bytes(
                std::mem::size_of::<OrdinarySignedOwned<'_>>() as u64,
                Instant::now(),
            )?;
            // This is the actual shared emitter position, never a new allowance.
            let global = solvers::LIVE_SINK
                .with(|slot| match slot.borrow().as_ref() {
                    Some(solvers::EventSink::Bound(sink)) => {
                        sink.verify_attempt_limits(binding, limits)
                            .map_err(|_| Stop::InvalidLimits)?;
                        sink.verify_directory(&events.directory)
                            .map_err(|_| Stop::InvalidLimits)?;
                        usize::try_from(sink.committed_event_count()).map_err(|_| Stop::Overflow)
                    }
                    _ => Err(Stop::InvalidLimits),
                })
                .or_else(|stop| self.ledger.fail(stop))?;
            let counter = std::cell::Cell::new(global);
            let prepared = prepare_signed(&self.math, self.ledger, true)?;
            owned.signed = Some(enumerate_prepared_signed(
                prepared,
                &mut |event, ledger| {
                    let before = events.accounting.acknowledged;
                    let result = events.forward_signed(event, ledger);
                    if events.accounting.acknowledged == before + 1 {
                        // Only the successful real emitter acknowledgement moves
                        // this counter; witness publication uses the same count.
                        counter.set(counter.get().checked_add(1).ok_or(Stop::Overflow)?);
                    }
                    result
                },
                &mut |witness, ledger| {
                    let mut global = counter.get();
                    let result = owned.publisher.publish(witness, ledger, &mut global);
                    // Publication may return a genuine receipt with a late stop.
                    // Preserve the actual resulting counter on every return.
                    counter.set(global);
                    result
                },
            ));
            if let Some(stop) = owned.signed.as_ref().and_then(|s| s.stop) {
                return Err(stop);
            }
            Ok(())
        })();
        if let Err(stop) = result {
            owned.stop = Some(*self.ledger.stopped.get_or_insert(stop));
        }
        owned.ledger = self.ledger.saved_ledger();
        owned
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn different_prepaid_ledger_is_rejected_before_signed_work_and_keeps_owners() {
        let f = solvers::verified_worker_original_tests::Fixture::with_ordinary_runtime_shape(
            23, false, "f4",
        );
        execution::create(&f.root).unwrap();
        let context = execution::admit(&f.root).unwrap().unwrap();
        let (binding, limits) = crate::ml_dataset::admit_capture_context(&f.root)
            .unwrap()
            .unwrap();
        let mut original_roots = Vec::new();
        let case = crate::ml_dataset::telemetry::measured(
            &f.root,
            &mut original_roots,
            "curve-construction",
            || solvers::curves::generate(&f.config),
        )
        .unwrap();
        let original = solvers::verify_and_store_worker_originals(
            &f.config,
            &case,
            &f.root,
            Some(&binding),
            &mut original_roots,
        )
        .unwrap();
        let emitter =
            capture::CaptureEmitter::create(&f.root.join("live"), binding.clone(), limits.clone())
                .unwrap();
        solvers::LIVE_SINK
            .with(|slot| *slot.borrow_mut() = Some(solvers::EventSink::Bound(emitter)));
        let _guard = solvers::LiveSinkGuard;
        let now = Instant::now();
        let mut ledger = Budget::new(
            f.config.work_limit,
            context.launch.memory_bytes,
            context.launch.trace_bytes,
            context.deadline,
            now,
        )
        .unwrap();
        let mut other = Budget::new(
            f.config.work_limit,
            context.launch.memory_bytes,
            context.launch.trace_bytes,
            context.deadline,
            now,
        )
        .unwrap();
        let wrong = OrdinarySignedOwned::admit(&binding, &limits, &mut other).unwrap();
        let other_before = other.saved_ledger();
        let mut registry = OrdinaryRootRegistry::admit(&original, &mut ledger, &limits).unwrap();
        let retained = construct(&original, &context, &mut ledger).unwrap();
        let mut input = match persist(&original, &case, &context, &limits, retained, &mut ledger) {
            Ok(input) => input,
            Err(_) => panic!("fixed input persistence refused"),
        };
        let mut events = OrdinaryEvents::admit(&original, &limits, &mut input).unwrap();
        let step = matrix_step(
            input,
            native::Engine::F4Macaulay,
            f.config.degree_bound,
            &mut |e, b| events.forward(e, b),
        );
        let mut registered = match registry.adopt_input(step) {
            Ok(input) => input,
            Err(_) => panic!("input adoption refused"),
        };
        let before = registered.ledger.saved_ledger();
        let count = events.accounting.acknowledged;
        let roots = registry.receipts.records.clone();
        let input_digest = registered.input_sha256;
        let rejected = registered.run_signed(&mut events, wrong);
        assert!(rejected.signed.is_none());
        assert!(!rejected.publisher.attempted);
        assert!(!rejected.group.write_attempted && !rejected.witness.write_attempted);
        assert_eq!(registered.ledger.additions, before.group_additions);
        assert_eq!(
            registered.ledger.checking_work,
            before.checking_reserved_work
        );
        assert_eq!(registered.ledger.native, before.native_reserved);
        assert_eq!(events.accounting.acknowledged, count);
        assert_eq!(registry.receipts.records, roots);
        assert_eq!(registered.input_sha256, input_digest);
        assert_eq!(other.saved_ledger(), other_before);
        assert!(std::ptr::eq(rejected.origin_ledger, &other));
        assert_eq!(
            rejected.stop,
            Some(before.first_stop.unwrap_or(Stop::Phase))
        );
    }
}

impl OrdinarySignedOwned<'_> {
    pub(super) fn facts(&self) -> (bool, bool, bool) {
        let domain = self.signed.as_ref().is_some_and(|s| {
            s.complete && s.acknowledged.len() == 192 && s.unacknowledged.is_none()
        });
        let witness = self.publisher.receipt.is_some();
        (domain, witness, self.witness.transferred)
    }
    pub(super) fn move_residuals(mut self, custody: &mut solvers::WorkerResidualCustody) {
        if self.witness.bytes.is_empty() {
            if let Some(encoding) = self.publisher.encoding.as_mut() {
                self.witness.bytes = std::mem::take(&mut encoding.bytes);
            }
        }
        retained_storage::move_slot_residual(self.group, custody);
        retained_storage::move_slot_residual(self.witness, custody);
    }
}
impl OrdinarySignedOwned<'_> {
    pub(super) fn roots_complete(&self) -> bool {
        self.group.transferred && (self.publisher.receipt.is_none() || self.witness.transferred)
    }
}
