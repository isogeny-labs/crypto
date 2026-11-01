//! Prepaid terminal custody for authoritative ordinary bytes. No signed proof.
use super::*;
use std::io::Write;
pub(super) const CAPS: [usize; 4] = [8192, 0, 32768, 16384];
// The read/format limits above are unchanged. Native bounded_matrix produces
// at most eight rows of eight monomials in exactly three Boolean variables.
// Fixed numeric/enum fields plus that closed shape serialize below 8KiB (the
// widest real-serde control below checks schema drift). Reserve this proven
// result envelope instead of the unrelated external read ceiling.
pub(super) const ADMITTED_CAPS: [usize; 4] = [8192, 16384, 8192, 16384];
fn bounded_report_shape(report: Option<&native::Report>) -> bool {
    report.is_none_or(|r| {
        r.columns <= 8
            && r.rank.is_none_or(|rank| rank <= 8)
            && r.rows.len() <= 8
            && r.rows.iter().all(|row| {
                row.n_vars == 3
                    && row.terms.len() <= 8
                    && row.terms.iter().all(|term| term.mask <= 7)
            })
    })
}

const SLOTS: [Slot; 4] = [
    Slot::Construction,
    Slot::Events,
    Slot::Result,
    Slot::ParentDelivery,
];

pub(super) struct OwnedSlot {
    pub(super) slot: Slot,
    pub(super) bytes: Vec<u8>,
    pub(super) encoding_complete: bool,
    pub(super) write_attempted: bool,
    pub(super) root: Option<types::Artifact>,
    pub(super) transferred: bool,
    pub(super) error: Option<String>,
}
impl OwnedSlot {
    pub(super) fn store(
        &mut self,
        original: &VerifiedWorkerOriginal<'_>,
        receipts: &mut solvers::WorkerStoredReceipts<'_>,
    ) {
        if self.write_attempted || !self.encoding_complete {
            return;
        }
        self.write_attempted = true;
        let (path, role, encoding) = self.slot.identity();
        match storage::write_bytes(original.root, path, role, encoding, &self.bytes) {
            Err(error) => self.error = Some(error),
            Ok(root) => {
                // Retain the acknowledgement BEFORE attempting transport.
                self.root = Some(root);
                match receipts.push(self.root.take().expect("stored root")) {
                    Ok(()) => self.transferred = true,
                    Err((error, root)) => {
                        self.root = Some(root);
                        self.error = Some(error);
                    }
                }
            }
        }
    }
}
#[derive(Serialize)]
struct Status<'a> {
    path: &'static str,
    retained_bytes: usize,
    encoding_complete: bool,
    write_attempted: bool,
    root_transferred: bool,
    untransferred_root: Option<&'a types::Artifact>,
    error: Option<&'a str>,
}
impl OwnedSlot {
    fn status(&self) -> Status<'_> {
        Status {
            path: self.slot.identity().0,
            retained_bytes: self.bytes.len(),
            encoding_complete: self.encoding_complete,
            write_attempted: self.write_attempted,
            root_transferred: self.transferred,
            untransferred_root: self.root.as_ref(),
            error: self.error.as_deref(),
        }
    }
}
struct Capped<'a> {
    bytes: &'a mut Vec<u8>,
    cap: usize,
}
impl Write for Capped<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.cap.saturating_sub(self.bytes.len()) {
            return Err(std::io::Error::other("ordinary terminal encoding cap"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub(super) fn encode(slot: &mut OwnedSlot, cap: usize, value: &impl Serialize) {
    match serde_json::to_writer(
        Capped {
            bytes: &mut slot.bytes,
            cap,
        },
        value,
    ) {
        Ok(()) => slot.encoding_complete = true,
        Err(error) => slot.error = Some(error.to_string()),
    }
}

pub(super) struct RetainedSlots {
    ledger: *const Budget,
    attempted: bool,
    slots: [OwnedSlot; 4],
    // Exact admitted ceilings, not observed storage/CPU costs.
    prepaid: [RetentionQuote; 4],
}
impl RetainedSlots {
    pub(super) fn admit(
        ledger: &mut Budget,
        limits: &capture::CaptureLimits,
    ) -> Result<Self, Stop> {
        let mut prepaid = [RetentionQuote {
            work: 0,
            bytes: 0,
            trace: 0,
        }; 4];
        for i in 0..4 {
            prepaid[i] = quote_parent_storage_length(ADMITTED_CAPS[i] as u64, false, limits)
                .or_else(|stop| ledger.fail(stop))?;
            ledger.reserve_checking(prepaid[i].work, prepaid[i].bytes, Instant::now())?;
            ledger.reserve_trace(prepaid[i].trace, Instant::now())?;
            // Result/delivery serialization and their bounded owner storage.
            ledger.reserve_checking(
                8 * ADMITTED_CAPS[i] as u64,
                ADMITTED_CAPS[i] as u64,
                Instant::now(),
            )?;
        }
        let mut slots = std::array::from_fn(|i| OwnedSlot {
            slot: SLOTS[i],
            bytes: Vec::new(),
            encoding_complete: false,
            write_attempted: false,
            root: None,
            transferred: false,
            error: None,
        });
        for i in 0..4 {
            if slots[i].bytes.try_reserve_exact(ADMITTED_CAPS[i]).is_err() {
                return ledger.fail(Stop::Allocation);
            }
        }
        Ok(Self {
            ledger,
            attempted: false,
            slots,
            prepaid,
        })
    }
}
#[derive(Serialize)]
struct EventIndex<'a> {
    schema: &'static str,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    start: &'a capture::CommittedEventPosition,
    end: &'a capture::CommittedEventPosition,
    acknowledged_events: u64,
    acknowledged_wire_bytes: u64,
    pending_tail: &'a [u8],
}
#[derive(Serialize)]
struct MatrixResult<'a> {
    schema: &'static str,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    report: Option<&'a native::Report>,
    adapter_stop: Option<Stop>,
    ledger: SavedBudgetLedger,
    event_accounting: &'a EventAccounting,
    retained_event_bytes: usize,
    acknowledged_event_prefix_bytes: usize,
    unacknowledged_event_tail_bytes: usize,
    source_completion_claimed: bool,
}
#[derive(Serialize)]
struct Delivery<'a> {
    schema: &'static str,
    input_sha256: [u8; 32],
    source_completion_claimed: bool,
    // This lists actual receipts BEFORE this delivery's own root is written.
    // Its receipt is subsequently carried by WorkerStoredReceipts.
    accepted_roots_before_delivery: &'a [types::Artifact],
    prior_slots: [Status<'a>; 3],
    prepaid_storage: &'a [RetentionQuote; 4],
    ledger: SavedBudgetLedger,
}

impl OrdinaryRootRegistry<'_> {
    /// One attempt only. All owners survive errors; no new budget/deadline is
    /// minted to flush already-authorized evidence after a producer stop.
    pub(super) fn retain_authoritative_slots(
        &mut self,
        input: &mut RegisteredMatrixInput<'_>,
        events: &mut OrdinaryEvents<'_>,
    ) -> Result<(), String> {
        if !self.input_registered
            || input.input_sha256 != events.input_sha256
            || input.capture_sequence != events.input_capture_sequence
            || !std::ptr::eq(self.ledger, std::ptr::from_ref(&*input.ledger))
            || !std::ptr::eq(self.ledger, events.ledger)
            || !std::ptr::eq(self.original, events.original)
        {
            return Err("ordinary retained owner/ledger/original mismatch".into());
        }
        let owned = self
            .slots
            .as_mut()
            .ok_or("ordinary retained slots absent")?;
        if owned.attempted || !std::ptr::eq(owned.ledger, self.ledger) {
            return Err("ordinary retained storage already attempted or wrong ledger".into());
        }
        // Check credits before moving bytes; refused checks leave all owners.
        let tail_len = events.retained_wire.len();
        let len = events
            .acknowledged_prefix_len
            .checked_add(tail_len)
            .ok_or("ordinary logical event length overflow")?;
        if tail_len > EVENT_CAP
            || events.accounting.acknowledged_wire_bytes != events.acknowledged_prefix_len as u64
            || events
                .end_position
                .event_count
                .checked_sub(events.start_position.event_count)
                .is_none_or(|n| n < events.accounting.acknowledged)
            || events
                .end_position
                .byte_end
                .checked_sub(events.start_position.byte_end)
                .is_none_or(|n| n < events.accounting.acknowledged_wire_bytes)
            || input.retained.bytes.len() > CAPS[0]
        {
            return Err("ordinary durable event prefix identity mismatch".into());
        }
        owned.attempted = true;
        owned.slots[0].bytes = std::mem::take(&mut input.retained.bytes);
        // Encoding failures retain raw bytes, never a normal JSON root.
        owned.slots[0].encoding_complete = !input.retained.encoding_failed;
        let index = EventIndex {
            schema: "crypto.ml-dataset.binary-m3-event-index/v1",
            input_sha256: events.input_sha256,
            input_capture_sequence: events.input_capture_sequence,
            start: &events.start_position,
            end: &events.end_position,
            acknowledged_events: events.accounting.acknowledged,
            acknowledged_wire_bytes: events.accounting.acknowledged_wire_bytes,
            pending_tail: &events.retained_wire,
        };
        encode(&mut owned.slots[1], ADMITTED_CAPS[1], &index);
        // The encoded index now owns any pending tail. If encoding refused,
        // leave the original buffer with the events owner for residual custody.
        if owned.slots[1].encoding_complete {
            events.retained_wire.clear();
        }
        for i in 0..2 {
            owned.slots[i].store(self.original, &mut self.receipts);
            if owned.slots[i].error.is_some() {
                let _ = input.ledger.observer_failed();
            }
        }
        let result = MatrixResult {
            schema: "crypto.ml-dataset.binary-m3-matrix-result/v4",
            input_sha256: events.input_sha256,
            input_capture_sequence: input.capture_sequence,
            report: input.report.as_ref(),
            adapter_stop: input.adapter_stop,
            ledger: input.ledger.saved_ledger(),
            event_accounting: &events.accounting,
            retained_event_bytes: len,
            acknowledged_event_prefix_bytes: events.acknowledged_prefix_len,
            unacknowledged_event_tail_bytes: tail_len,
            source_completion_claimed: false,
        };
        if bounded_report_shape(input.report.as_ref()) {
            encode(&mut owned.slots[2], ADMITTED_CAPS[2], &result);
        } else {
            owned.slots[2].error =
                Some("native report violates existing three-variable shape invariant".into());
        }
        owned.slots[2].store(self.original, &mut self.receipts);
        if owned.slots[2].error.is_some() {
            let _ = input.ledger.observer_failed();
        }
        let (prior, delivery_slot) = owned.slots.split_at_mut(3);
        let delivery = Delivery {
            schema: "crypto.ml-dataset.binary-m3-parent-delivery/v1",
            input_sha256: events.input_sha256,
            source_completion_claimed: false,
            accepted_roots_before_delivery: &self.receipts.records,
            prior_slots: [prior[0].status(), prior[1].status(), prior[2].status()],
            prepaid_storage: &owned.prepaid,
            ledger: input.ledger.saved_ledger(),
        };
        encode(&mut delivery_slot[0], CAPS[3], &delivery);
        delivery_slot[0].store(self.original, &mut self.receipts);
        if delivery_slot[0].error.is_some() {
            let _ = input.ledger.observer_failed();
        }
        // Returning Err signals incomplete retention, but the registry retains
        // every byte and untransferred root; accepted roots stay in transport.
        if owned.slots.iter().any(|s| !s.transferred) {
            Err("ordinary terminal retention incomplete; owners retained".into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub(super) fn test_store(
    registry: &mut OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &mut OrdinaryEvents<'_>,
) {
    let original_stop = input.ledger.stopped;
    let collision = original_stop == Some(Stop::Observer);
    let construction_bytes = input.retained.bytes.clone();
    let event_bytes = events.retained_wire.clone();
    let acknowledged = events.acknowledged_prefix_len;
    events.input_sha256[0] ^= 1;
    assert!(registry.retain_authoritative_slots(input, events).is_err());
    events.input_sha256[0] ^= 1;
    events.input_capture_sequence += 1;
    assert!(registry.retain_authoritative_slots(input, events).is_err());
    events.input_capture_sequence -= 1;
    assert!(!registry.slots.as_ref().unwrap().attempted);
    assert_eq!(input.retained.bytes, construction_bytes);
    assert_eq!(events.retained_wire, event_bytes);
    assert_eq!(registry.receipts.records.len(), 1);
    if collision {
        assert_eq!(acknowledged, 0);
        assert!(!event_bytes.is_empty());
        assert_eq!(events.accounting.acknowledged, 0);
        assert_eq!(events.accounting.attempted, 1);
        assert_eq!(events.accounting.reserved_retention_work, 0);
    } else {
        assert!(event_bytes.is_empty());
        assert_eq!(
            acknowledged as u64,
            events.accounting.acknowledged_wire_bytes
        );
    }
    if collision {
        std::fs::write(
            registry.original.root.join(Slot::Construction.identity().0),
            b"existing different bytes",
        )
        .unwrap();
    }
    let result = registry.retain_authoritative_slots(input, events);
    assert_eq!(result.is_err(), collision);
    let slots = registry.slots.as_ref().unwrap();
    assert_eq!(slots.slots[0].bytes, construction_bytes);
    let index: serde_json::Value = serde_json::from_slice(&slots.slots[1].bytes).unwrap();
    assert_eq!(index["pending_tail"], serde_json::json!(event_bytes));
    assert_eq!(
        index["acknowledged_wire_bytes"],
        serde_json::json!(acknowledged)
    );
    assert_eq!(
        index["schema"],
        "crypto.ml-dataset.binary-m3-event-index/v1"
    );
    assert!(input.retained.bytes.is_empty());
    assert!(events.retained_wire.is_empty());
    for slot in &slots.slots {
        if slot.transferred {
            assert_eq!(
                std::fs::read(registry.original.root.join(slot.slot.identity().0)).unwrap(),
                slot.bytes
            );
            assert!(slot.root.is_none());
            let receipt = registry
                .receipts
                .records
                .iter()
                .find(|r| r.path == slot.slot.identity().0)
                .unwrap();
            assert_eq!(
                receipt.sha256,
                hex::encode(sha2::Sha256::digest(&slot.bytes))
            );
        }
    }
    let result: serde_json::Value = serde_json::from_slice(&slots.slots[2].bytes).unwrap();
    assert_eq!(result["acknowledged_event_prefix_bytes"], acknowledged);
    assert_eq!(result["source_completion_claimed"], false);
    assert_eq!(result["unacknowledged_event_tail_bytes"], event_bytes.len());
    if collision {
        assert_eq!(
            std::fs::read(registry.original.root.join(Slot::Construction.identity().0)).unwrap(),
            b"existing different bytes"
        );
        assert!(slots.slots[0].error.is_some());
        assert!(!slots.slots[0].transferred);
    }
    assert_eq!(input.ledger.stopped, original_stop);
    let before = registry.receipts.records.len();
    assert!(registry.retain_authoritative_slots(input, events).is_err());
    assert_eq!(registry.receipts.records.len(), before);
}

impl RetainedSlots {
    pub(super) fn move_residuals(self, custody: &mut solvers::WorkerResidualCustody) {
        for slot in self.slots {
            move_slot_residual(slot, custody);
        }
    }
}
pub(super) fn move_slot_residual(slot: OwnedSlot, custody: &mut solvers::WorkerResidualCustody) {
    if !slot.transferred && (!slot.bytes.is_empty() || slot.root.is_some()) {
        // Closed worker assembly has four ordinary terminal slots, two signed
        // slots and at most two mutually exclusive early-failure owners. Ten
        // prepaid transport entries cover every path without a new allocation.
        let item = solvers::WorkerResidualArtifact {
            path: slot.slot.identity().0,
            bytes: slot.bytes,
            untransferred_root: slot.root,
        };
        if let Err(_item) = custody.retain(item) {
            unreachable!("closed ordinary residual inventory exceeded ten admitted entries");
        }
    }
}

#[cfg(test)]
mod envelope_tests {
    use super::*;
    #[test]
    fn widest_durable_index_and_owned_pending_bytes_fit_prepaid_envelope() {
        let position = capture::CommittedEventPosition {
            binding_sha256: "f".repeat(64),
            event_count: u64::MAX,
            byte_end: u64::MAX,
            last_event_sha256: "f".repeat(64),
        };
        let tail = vec![255; EVENT_CAP];
        let view = EventIndex {
            schema: "crypto.ml-dataset.binary-m3-event-index/v1",
            input_sha256: [255; 32],
            input_capture_sequence: u64::MAX,
            start: &position,
            end: &position,
            acknowledged_events: u64::MAX,
            acknowledged_wire_bytes: u64::MAX,
            pending_tail: &tail,
        };
        let bytes = serde_json::to_vec(&view).unwrap();
        assert!(bytes.len() < ADMITTED_CAPS[1]);
        let parsed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(parsed["pending_tail"], serde_json::json!(tail));
    }
    #[test]
    fn real_serde_widest_native_shape_fits_admitted_result_envelope() {
        let value: serde_json::Value =
            serde_json::from_str(include_str!("binary_m3_result_max.json")).unwrap();
        let mut report: native::Report = serde_json::from_value(value["report"].clone()).unwrap();
        let ledger: SavedBudgetLedger = serde_json::from_value(value["ledger"].clone()).unwrap();
        let accounting = EventAccounting {
            attempted: u64::MAX,
            acknowledged: u64::MAX,
            reserved_checking_work: u64::MAX,
            reserved_trace_bytes: u64::MAX,
            acknowledged_wire_bytes: u64::MAX,
            reserved_retention_work: u64::MAX,
            reserved_retention_bytes: u64::MAX,
            reserved_retention_trace: u64::MAX,
        };
        assert!(bounded_report_shape(Some(&report)));
        let view = MatrixResult {
            schema: "crypto.ml-dataset.binary-m3-matrix-result/v4",
            input_sha256: [255; 32],
            input_capture_sequence: u64::MAX,
            report: Some(&report),
            adapter_stop: Some(Stop::Serialization),
            ledger,
            event_accounting: &accounting,
            retained_event_bytes: usize::MAX,
            acknowledged_event_prefix_bytes: usize::MAX,
            unacknowledged_event_tail_bytes: usize::MAX,
            source_completion_claimed: false,
        };
        let bytes = serde_json::to_vec(&view).unwrap();
        assert_eq!(bytes.len(), serde_json::to_vec(&value).unwrap().len());
        assert!(bytes.len() < 4096 && bytes.len() < ADMITTED_CAPS[2]);
        assert_eq!(CAPS[2], 32768);
        report.rows.push(report.rows[0].clone());
        assert!(!bounded_report_shape(Some(&report)));
    }
}
