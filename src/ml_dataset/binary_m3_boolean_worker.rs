//! Optional supplied-proof worker increment. No CLI dispatch or proof search.
use super::*;
const ANCHOR_CAP: usize = 8192;
const CHECK_CAP: usize = 49152;
const PROOF_CAP: usize = 4096;
// This request is untrusted proof material, never an equation/source token.
// Future finite-span producer must be separately admitted on the same ledger.
#[derive(Clone, Copy)]
pub(in crate::ml_dataset::solvers) enum ProofMode<'a> {
    Supplied(&'a [u8]),
    Produce,
}
pub(in crate::ml_dataset::solvers) struct Request<'a> {
    pub(in crate::ml_dataset::solvers) row_index: usize,
    pub(in crate::ml_dataset::solvers) proof: ProofMode<'a>,
}
impl Request<'_> {
    fn receipt_schema(&self) -> &'static str {
        match self.proof {
            ProofMode::Supplied(_) => "crypto.ml-dataset.binary-m3-boolean-check/v1",
            ProofMode::Produce => "crypto.ml-dataset.binary-m3-boolean-check/v2",
        }
    }
    fn supplied(&self) -> &[u8] {
        match &self.proof {
            ProofMode::Supplied(bytes) => bytes,
            ProofMode::Produce => &[],
        }
    }
}
pub(in crate::ml_dataset::solvers) struct Owned<'a> {
    request: Request<'a>,
    origin: *const Budget,
    anchor: OwnedSlot,
    check: OwnedSlot,
    attempted: bool,
}
impl<'a> Owned<'a> {
    pub(in crate::ml_dataset::solvers) fn admit(
        request: Request<'a>,
        ledger: &mut Budget,
        limits: &capture::CaptureLimits,
    ) -> Result<Self, Stop> {
        if ledger.phase != Phase::Setup {
            return ledger.fail(Stop::Phase);
        }
        if request.supplied().len() > PROOF_CAP {
            return ledger.fail(Stop::InvalidLimits);
        }
        // Reserve success AND failure retention before construction/capture.
        for cap in [ANCHOR_CAP, CHECK_CAP] {
            let q = quote_parent_storage_length(cap as u64, false, limits)
                .or_else(|s| ledger.fail(s))?;
            ledger.reserve_checking(q.work, q.bytes, Instant::now())?;
            ledger.reserve_trace(q.trace, Instant::now())?;
        }
        // Covers bounded JSON values/hex metadata and encoding, not checker
        // arithmetic. The latter is charged once at its actual invocation.
        ledger.reserve_checking(
            16 * (ANCHOR_CAP + CHECK_CAP) as u64,
            2 * 1024 * 1024,
            Instant::now(),
        )?;
        let make = |slot, cap| -> Result<OwnedSlot, Stop> {
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(cap).map_err(|_| Stop::Allocation)?;
            Ok(OwnedSlot {
                slot,
                bytes,
                encoding_complete: false,
                write_attempted: false,
                root: None,
                transferred: false,
                error: None,
            })
        };
        let anchor = make(Slot::BooleanAnchor, ANCHOR_CAP).or_else(|s| ledger.fail(s))?;
        let check = make(Slot::BooleanCheck, CHECK_CAP).or_else(|s| ledger.fail(s))?;
        Ok(Self {
            request,
            origin: ledger,
            anchor,
            check,
            attempted: false,
        })
    }
    pub(in crate::ml_dataset::solvers) fn run_before_signed(
        &mut self,
        registry: &mut OrdinaryRootRegistry<'_>,
        input: &mut RegisteredMatrixInput<'_>,
        events: &OrdinaryEvents<'_>,
    ) {
        if self.attempted {
            return;
        }
        self.attempted = true;
        let before = input.ledger.saved_ledger();
        let valid = std::ptr::eq(self.origin, registry.ledger)
            && std::ptr::eq(self.origin, std::ptr::from_ref(&*input.ledger))
            && registry.input_registered
            && std::ptr::eq(registry.original, events.original)
            && std::ptr::eq(self.origin, events.ledger)
            && input.input_sha256 == events.input_sha256
            && input.capture_sequence == events.input_capture_sequence
            && input.ledger.phase == Phase::MatrixComplete;
        let checked = if valid {
            input.ledger.check(Instant::now())
        } else {
            Err(Stop::Phase)
        };
        let mut attempt = None;
        let mut produced = None;
        let mut reason = "pre-signed anchor admission refused";
        if checked.is_ok() {
            // Separate immutable anchor: terminal Result remains post-signed,
            // preserving existing EventIndex/MatrixResult replay equalities.
            let tail = events.retained_wire.len();
            let retained = events.acknowledged_prefix_len.checked_add(tail);
            if bounded_report_shape(input.report.as_ref()) && retained.is_some() {
                let view = MatrixResult {
                    schema: "crypto.ml-dataset.binary-m3-matrix-result/v4",
                    input_sha256: input.input_sha256,
                    input_capture_sequence: input.capture_sequence,
                    report: input.report.as_ref(),
                    adapter_stop: input.adapter_stop,
                    ledger: input.ledger.saved_ledger(),
                    event_accounting: &events.accounting,
                    retained_event_bytes: retained.unwrap(),
                    acknowledged_event_prefix_bytes: events.acknowledged_prefix_len,
                    unacknowledged_event_tail_bytes: tail,
                    source_completion_claimed: false,
                };
                encode(&mut self.anchor, ANCHOR_CAP, &view);
                self.anchor.store(registry.original, &mut registry.receipts);
                // The check joins the actual acknowledged transport entry;
                // missing/rejected anchor transfer becomes a retained refusal.
                match &self.request.proof {
                    ProofMode::Supplied(suffix) => {
                        attempt = Some(boolean_consequence::check_anchor(
                            registry,
                            input,
                            events,
                            self.request.row_index,
                            suffix,
                            &self.anchor,
                        ))
                    }
                    ProofMode::Produce => {
                        produced = Some(boolean_consequence::produce_and_check(
                            registry,
                            input,
                            events,
                            self.request.row_index,
                            &self.anchor,
                        ))
                    }
                }
                reason = "checked_or_refused";
            } else {
                reason = "pre-signed bounded report unavailable";
            }
        }
        let after = input.ledger.saved_ledger();
        let record = serde_json::json!({
            "schema":self.request.receipt_schema(),
            "binding":registry.original.binding,
            "original_public_sha256":registry.original.public_sha256,
            "original_factor_base_sha256":registry.original.factor_base_sha256,
            "input_sha256":input.input_sha256,
            "input_capture_sequence":input.capture_sequence,
            "requested_row_index":self.request.row_index,
            "supplied_proof_suffix_hex":hex::encode(self.request.supplied()),
            "derivation_origin":match self.request.proof {ProofMode::Supplied(_)=>"separately_supplied_boolean_generator_identity",ProofMode::Produce=>"separately_searched_boolean_generator_identity"},
            "native_operation_witness":false,"training_export_admitted":false,
            "reason":reason,"before":before,"after":after,
            "attempt":attempt.as_ref().map(boolean_consequence::restricted_attempt),
            "producer":produced,
            "anchor_status":self.anchor.status(),
        });
        encode(&mut self.check, CHECK_CAP, &record);
        self.check.store(registry.original, &mut registry.receipts);
        if !self.anchor.transferred || !self.check.transferred {
            let _ = input.ledger.observer_failed();
        }
    }
    // Prepaid failure output is retained even if original construction/input
    // never reached an admitted target; never invent a source or target row.
    pub(in crate::ml_dataset::solvers) fn finish(
        mut self,
        registry: &mut OrdinaryRootRegistry<'_>,
        ledger: &Budget,
        custody: &mut solvers::WorkerResidualCustody,
    ) {
        if !self.attempted {
            let record = serde_json::json!({
                "schema":self.request.receipt_schema(),
                "binding":registry.original.binding,
                "requested_row_index":self.request.row_index,
                "supplied_proof_suffix_hex":hex::encode(self.request.supplied()),
                "reason":"original_target_not_reached", "ledger":ledger.saved_ledger(),
                "attempt":null,"training_export_admitted":false,
                "native_operation_witness":false,
            });
            encode(&mut self.check, CHECK_CAP, &record);
            self.check.store(registry.original, &mut registry.receipts);
        }
        move_slot_residual(self.anchor, custody);
        move_slot_residual(self.check, custody);
    }
}
