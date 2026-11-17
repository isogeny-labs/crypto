//! Unwired live-owner adapter. Child of binary_m3_storage, not a raw-file API.
//! Publication requires a separate charged receipt/export integration.
use super::*;
use crate::cryptanalysis::pq_groebner_f2::F2BoolPoly;
#[path = "boolean_identity.rs"]
mod identity;
#[path = "boolean_identity_producer.rs"]
mod producer;
const WIRE_CAP: usize = 16384;
const PROOF_CAP: usize = 4096;
// Includes bounded JSON decoding, hashing, equality, fixed-mask encoding,
// sink snapshots, and fixed result metadata. This is an admission envelope,
// not an instruction measurement. It is never refunded.
const ADAPTER_WORK: u64 = 2 * 1024 * 1024;
const ADAPTER_BYTES: u64 = 4 * 1024 * 1024;
const PRODUCED_WIRE_CAP: usize = 4096;
// 62 dense N3 polynomials use 62*42 bytes; the header and eight
// generator/multiplier pairs add fewer than 128. This covers every producer
// output, not caller-supplied proofs. The independent arithmetic is unchanged.
fn produced_limits() -> identity::Limits {
    identity::Limits {
        variables: 3,
        generators: 61,
        polynomial_terms: 8,
        proof_terms: 8,
        accumulator_entries: 8,
        caps: [4096, 32768, 4096, 4096, 4096, 4096, 16, 262144, 65536],
    }
}
fn limits() -> identity::Limits {
    identity::Limits {
        variables: 3,
        generators: 61,
        polynomial_terms: 8,
        proof_terms: 128,
        accumulator_entries: 8,
        caps: [
            16384, 262144, 32768, 32768, 65536, 32768, 256, 2097152, 524288,
        ],
    }
}
#[derive(Debug)]
pub(super) enum Refusal {
    SharedBudget(Stop),
    Owner,
    Shape,
    Receipt,
    Watermark,
    Allocation,
    Encoding,
    PostCheck {
        reason: PostCheckFailure,
        wire: Vec<u8>,
        audit: identity::Audit,
    },
}
#[derive(Debug)]
pub(super) enum PostCheckFailure {
    SharedBudget(Stop),
    Watermark,
}
// Fields stay restricted. No serde/public training record is implemented here.
pub(super) struct RestrictedCheck {
    pub(super) input_sha256: [u8; 32],
    pub(super) input_capture_sequence: u64,
    pub(super) result_sha256: String,
    pub(super) watermark: capture::CommittedEventPosition,
    pub(super) row_index: usize,
    // Exact ordered generator/target/proof encoding; no digest substitutes it.
    pub(super) wire: Vec<u8>,
    pub(super) audit: identity::Audit,
}
pub(super) struct Attempt {
    pub(super) before: SavedBudgetLedger,
    pub(super) after: SavedBudgetLedger,
    pub(super) outcome: Result<RestrictedCheck, Refusal>,
}
// An explicitly source-only view: no target, proof, row index, outcome,
// watermark, observed counters or runtime labels. Borrowed originals preserve
// family/twist/field/policy identities without a caller-supplied replacement.
pub(super) struct SourceView<'a> {
    pub(super) config: &'a types::CaseConfig,
    pub(super) public: &'a serde_json::Value,
    pub(super) factor_base: &'a serde_json::Value,
    pub(super) generators: &'a [F2BoolPoly],
    // vi is bit zero selecting codes[0]/codes[1] for summand i, i=0,1,2.
    pub(super) selector_codes: [u64; 2],
}
fn same_owners(
    registry: &OrdinaryRootRegistry<'_>,
    input: &RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
) -> bool {
    registry.input_registered
        && std::ptr::eq(registry.original, events.original)
        && std::ptr::eq(registry.ledger, std::ptr::from_ref(&*input.ledger))
        && std::ptr::eq(registry.ledger, events.ledger)
        && input.input_sha256 == events.input_sha256
        && input.capture_sequence == events.input_capture_sequence
}
// This function does not mint a published feature record; it exposes only
// already-admitted source data to a future exporter inside this owner module.
pub(super) fn source_view<'a>(
    registry: &'a OrdinaryRootRegistry<'_>,
    input: &'a RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
) -> Result<SourceView<'a>, Refusal> {
    if !same_owners(registry, input, events) {
        return Err(Refusal::Owner);
    }
    let built = input.retained.constructed.as_ref().ok_or(Refusal::Owner)?;
    Ok(SourceView {
        config: registry.original.config,
        public: registry.original.public,
        factor_base: registry.original.factor_base,
        generators: &built.equations,
        selector_codes: input.math.codes,
    })
}
fn current_head(events: &OrdinaryEvents<'_>) -> Result<capture::CommittedEventPosition, Refusal> {
    solvers::LIVE_SINK.with(|cell| {
        let mut cell = cell.borrow_mut();
        match cell.as_mut() {
            Some(solvers::EventSink::Bound(sink)) => {
                sink.verify_directory(&events.directory)
                    .map_err(|_| Refusal::Watermark)?;
                let binding = events.original.binding.ok_or(Refusal::Owner)?;
                sink.verify_attempt_limits(binding, events.limits)
                    .map_err(|_| Refusal::Watermark)?;
                Ok(sink.committed_event_position())
            }
            _ => Err(Refusal::Watermark),
        }
    })
}
// Fixed three-selector adapter only. General checker remains arbitrary-width.
// Native degree order is not wire lexicographic order; preserve generator
// order and zeros while converting each polynomial independently.
fn write_poly(out: &mut impl Write, p: &F2BoolPoly) -> Result<(), Refusal> {
    if p.n_vars != 3 || p.terms.len() > 8 {
        return Err(Refusal::Shape);
    }
    let mut seen = 0u8;
    for t in &p.terms {
        if t.mask > 7 || seen & (1 << t.mask) != 0 {
            return Err(Refusal::Shape);
        }
        seen |= 1 << t.mask;
    }
    write!(out, " {}", p.terms.len()).map_err(|_| Refusal::Encoding)?;
    for mask in [0u8, 1, 3, 7, 5, 2, 6, 4] {
        if seen & (1 << mask) == 0 {
            continue;
        }
        write!(out, " {}", mask.count_ones()).map_err(|_| Refusal::Encoding)?;
        for v in 0..3 {
            if mask & (1 << v) != 0 {
                write!(out, " {v}").map_err(|_| Refusal::Encoding)?;
            }
        }
    }
    Ok(())
}
// Successful OwnedSlot::store moves its acknowledgement into this actual
// transport owner. An untransferred local root is not an accepted receipt.
// Caller prepays the bounded scan/hash in ADAPTER_WORK; no new allocation.
pub(super) fn transferred_result<'a>(
    slot: &OwnedSlot,
    receipts: &'a solvers::WorkerStoredReceipts<'_>,
    original: &VerifiedWorkerOriginal<'_>,
    expected: Slot,
) -> Result<&'a types::Artifact, Refusal> {
    if !std::ptr::eq(receipts.original, original)
        || receipts.capacity > solvers::WORKER_STORED_RECEIPT_MAX
        || receipts.records.len() > receipts.capacity
        || slot.slot != expected
        || !slot.write_attempted
        || !slot.transferred
        || !slot.encoding_complete
        || slot.root.is_some()
        || slot.error.is_some()
        || slot.bytes.len() > ADMITTED_CAPS[2]
    {
        return Err(Refusal::Receipt);
    }
    let mut matching = receipts
        .records
        .iter()
        .filter(|r| r.path == expected.identity().0);
    let root = matching.next().ok_or(Refusal::Receipt)?;
    if matching.next().is_some()
        || !expected.matches(root)
        || root.bytes != slot.bytes.len() as u64
        || root.sha256 != crate::ml_dataset::digest(&slot.bytes)
    {
        return Err(Refusal::Receipt);
    }
    Ok(root)
}

pub(super) fn check(
    registry: &OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    row_index: usize,
    // Only the proof suffix is supplied: space, proof_count, proof terms,
    // newline. Source generators and target never come from caller bytes.
    proof_suffix: &[u8],
) -> Attempt {
    let before = input.ledger.saved_ledger();
    let outcome = match registry.slots.as_ref() {
        Some(slots) if slots.attempted && std::ptr::eq(slots.ledger, registry.ledger) => {
            check_inner(
                registry,
                input,
                events,
                row_index,
                proof_suffix,
                &slots.slots[2],
                Slot::Result,
            )
        }
        _ => Err(Refusal::Receipt),
    };
    let after = input.ledger.saved_ledger();
    Attempt {
        before,
        after,
        outcome,
    }
}
pub(super) fn check_anchor(
    registry: &OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    row_index: usize,
    proof_suffix: &[u8],
    anchor: &OwnedSlot,
) -> Attempt {
    let before = input.ledger.saved_ledger();
    let outcome = check_inner(
        registry,
        input,
        events,
        row_index,
        proof_suffix,
        anchor,
        Slot::BooleanAnchor,
    );
    let after = input.ledger.saved_ledger();
    Attempt {
        before,
        after,
        outcome,
    }
}
fn check_inner(
    registry: &OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    row_index: usize,
    proof_suffix: &[u8],
    slot: &OwnedSlot,
    expected: Slot,
) -> Result<RestrictedCheck, Refusal> {
    let limits = limits();
    let work = ADAPTER_WORK
        + limits
            .caps
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != identity::Resource::AllocationBytes as usize)
            .map(|(_, cap)| cap)
            .sum::<u64>();
    input
        .ledger
        .reserve_checking(
            work,
            ADAPTER_BYTES + limits.caps[identity::Resource::AllocationBytes as usize],
            Instant::now(),
        )
        .map_err(Refusal::SharedBudget)?;
    // All subsequent refusal paths retain this admission on the same ledger.
    if !same_owners(registry, input, events) {
        return Err(Refusal::Owner);
    }
    if proof_suffix.len() > PROOF_CAP {
        return Err(Refusal::Shape);
    }
    let root = transferred_result(slot, &registry.receipts, registry.original, expected)?;
    let watermark = current_head(events)?;
    if watermark != events.end_position {
        return Err(Refusal::Watermark);
    }
    let saved: serde_json::Value =
        serde_json::from_slice(&slot.bytes).map_err(|_| Refusal::Receipt)?;
    let report = input.report.as_ref().ok_or(Refusal::Shape)?;
    if !bounded_report_shape(Some(report)) || report.rows.get(row_index).is_none() {
        return Err(Refusal::Shape);
    }
    // Exact report equality binds a row ordinal to the immutable transferred
    // result, including partial reports; an absent row never becomes zero.
    if saved["schema"] != "crypto.ml-dataset.binary-m3-matrix-result/v4"
        || saved["input_sha256"]
            != serde_json::to_value(input.input_sha256).map_err(|_| Refusal::Encoding)?
        || saved["input_capture_sequence"] != input.capture_sequence
        || saved["report"] != serde_json::to_value(report).map_err(|_| Refusal::Encoding)?
    {
        return Err(Refusal::Receipt);
    }
    let built = input.retained.constructed.as_ref().ok_or(Refusal::Owner)?;
    if built.equations.len() != registry.original.config.n as usize
        || built.equations.len() > 61
        || built.equations.is_empty()
    {
        return Err(Refusal::Shape);
    }
    let mut wire = Vec::new();
    wire.try_reserve_exact(WIRE_CAP)
        .map_err(|_| Refusal::Allocation)?;
    {
        let mut writer = Capped {
            bytes: &mut wire,
            cap: WIRE_CAP,
        };
        write!(writer, "BQI1 3 {}", built.equations.len()).map_err(|_| Refusal::Encoding)?;
        for p in &built.equations {
            write_poly(&mut writer, p)?;
        }
        write_poly(&mut writer, &report.rows[row_index])?;
        writer
            .write_all(proof_suffix)
            .map_err(|_| Refusal::Encoding)?;
    }
    let audit = identity::verify_untrusted(&wire, limits, input.ledger.deadline);
    if let Err(stop) = input.ledger.check(Instant::now()) {
        return Err(Refusal::PostCheck {
            reason: PostCheckFailure::SharedBudget(stop),
            wire,
            audit,
        });
    }
    match current_head(events) {
        Ok(after) if after == watermark => {}
        _ => {
            return Err(Refusal::PostCheck {
                reason: PostCheckFailure::Watermark,
                wire,
                audit,
            })
        }
    }
    Ok(RestrictedCheck {
        input_sha256: input.input_sha256,
        input_capture_sequence: input.capture_sequence,
        result_sha256: root.sha256.clone(),
        watermark,
        row_index,
        wire,
        audit,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;
    // Genuine retained ordinary construction fixture. Reading it here supplies
    // arithmetic test data ONLY, never an admitted original-system token.
    fn fixture() -> Vec<F2BoolPoly> {
        let saved: serde_json::Value =
            serde_json::from_str(include_str!("binary_m3_retained_input.json")).unwrap();
        let construction: serde_json::Value =
            serde_json::from_str(saved["construction_json"].as_str().unwrap()).unwrap();
        construction["evidence"]["coordinate_anfs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| poly(v.as_u64().unwrap() as u8))
            .collect()
    }
    fn poly(bits: u8) -> F2BoolPoly {
        F2BoolPoly {
            n_vars: 3,
            terms: (0..8)
                .filter(|i| bits & (1 << i) != 0)
                .map(|i| F2BoolMono::from_mask(i))
                .collect(),
        }
    }
    fn wire(gens: &[F2BoolPoly], target: &F2BoolPoly, proof: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(WIRE_CAP);
        let mut writer = Capped {
            bytes: &mut bytes,
            cap: WIRE_CAP,
        };
        write!(writer, "BQI1 3 {}", gens.len()).unwrap();
        for g in gens {
            write_poly(&mut writer, g).unwrap();
        }
        write_poly(&mut writer, target).unwrap();
        writer.write_all(proof).unwrap();
        bytes
    }
    fn verify(bytes: &[u8]) -> identity::Audit {
        identity::verify_untrusted(
            bytes,
            limits(),
            Instant::now() + std::time::Duration::from_secs(5),
        )
    }
    #[test]
    fn retained_ordinary_generators_preserve_order_and_zero_coordinates() {
        let gens = fixture();
        assert_eq!(gens.len(), 23);
        assert!(gens[15].terms.is_empty());
        assert!(gens[18].terms.is_empty());
        assert!(verify(&wire(&gens, &gens[0], b" 1 0 0\n")).result.is_ok());
        let mut reordered = fixture();
        reordered.swap(0, 1);
        assert!(matches!(
            verify(&wire(&reordered, &gens[0], b" 1 0 0\n")).result,
            Err(identity::Failure::EqualityMismatch)
        ));
    }
    #[test]
    fn retained_nontrivial_xor_and_false_target() {
        let gens = fixture();
        // Exact retained ANFs 105 XOR 232 = 129. Includes cross-source cancellation.
        assert!(verify(&wire(&gens, &poly(129), b" 2 0 0 1 0\n"))
            .result
            .is_ok());
        assert!(matches!(
            verify(&wire(&gens, &poly(128), b" 2 0 0 1 0\n")).result,
            Err(identity::Failure::EqualityMismatch)
        ));
        // Constant coordinate 5; zero coordinate 15 remains index 15.
        assert!(verify(&wire(&gens, &poly(1), b" 1 5 0\n")).result.is_ok());
        assert!(verify(&wire(&gens, &poly(0), b" 1 15 0\n")).result.is_ok());
    }
    #[test]
    fn codec_rejects_duplicate_high_mask_and_wrong_dimension() {
        for p in [
            F2BoolPoly {
                n_vars: 3,
                terms: vec![F2BoolMono::one(), F2BoolMono::one()],
            },
            F2BoolPoly {
                n_vars: 3,
                terms: vec![F2BoolMono::from_mask(8)],
            },
            F2BoolPoly {
                n_vars: 4,
                terms: vec![],
            },
        ] {
            assert!(matches!(
                write_poly(&mut Vec::new(), &p),
                Err(Refusal::Shape)
            ));
        }
        let gens = fixture();
        assert!(matches!(
            verify(&wire(&gens, &gens[0], b" 1 0 1 3\n")).result,
            Err(identity::Failure::Malformed(_))
        ));
    }
    #[test]
    fn exact_lex_order_is_not_mask_order_and_codec_cap_refuses() {
        let mut bytes = Vec::new();
        write_poly(&mut bytes, &poly(255)).unwrap();
        assert_eq!(bytes, b" 8 0 1 0 2 0 1 3 0 1 2 2 0 2 1 1 2 1 2 1 2");
        let mut small = Vec::new();
        let mut capped = Capped {
            bytes: &mut small,
            cap: 2,
        };
        assert!(write_poly(&mut capped, &poly(255)).is_err());
        assert!(small.len() <= 2);
    }
}

// Called only from the existing genuine construction -> persisted input ->
// adopt_input -> retain_authoritative_slots test path, after actual storage
// and WorkerStoredReceipts::push. It exercises custody without pretending
// that a stopped original ledger can admit a later full checker invocation.
#[cfg(test)]
pub(super) fn test_transferred_result_custody(
    registry: &mut OrdinaryRootRegistry<'_>,
    ledger: &mut Budget,
) {
    // The real persisted-input test reaches retention directly from matrix
    // completion, without signed replay. The normal positive control retains
    // its actual live phase/deadline; stopped controls remain stopped.
    let before = ledger.saved_ledger();
    let deadline = ledger.deadline;
    if ledger.stopped.is_none() {
        assert_eq!(ledger.phase, Phase::MatrixComplete);
        ledger.check(Instant::now()).unwrap();
    }
    assert_eq!(ledger.saved_ledger(), before);
    assert_eq!(ledger.deadline, deadline);
    let original = registry.original;
    let slot = &mut registry.slots.as_mut().unwrap().slots[2];
    let receipts = &mut registry.receipts;
    assert!(slot.transferred);
    assert!(slot.root.is_none());
    let accepted = transferred_result(slot, receipts, original, Slot::Result)
        .unwrap()
        .clone();
    assert_eq!(accepted.sha256, crate::ml_dataset::digest(&slot.bytes));
    let index = receipts
        .records
        .iter()
        .position(|r| r.path == accepted.path)
        .unwrap();

    let missing = receipts.records.remove(index);
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records.insert(index, missing);
    receipts.records[index].sha256.replace_range(
        0..1,
        if accepted.sha256.starts_with('0') {
            "1"
        } else {
            "0"
        },
    );
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records[index] = accepted.clone();
    receipts.records[index].bytes += 1;
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records[index] = accepted.clone();
    receipts.records[index].role.push_str("-foreign");
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records[index] = accepted.clone();
    receipts.records[index].encoding = "jsonl".into();
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records[index] = accepted.clone();
    // Corrupt a different existing transport entry, avoiding any new capacity.
    let other = if index == 0 { 1 } else { 0 };
    let held = std::mem::replace(&mut receipts.records[other], accepted.clone());
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    receipts.records[other] = held;
    slot.transferred = false;
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    slot.transferred = true;
    slot.root = Some(accepted.clone());
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_err());
    slot.root = None;
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_ok());

    // Genuine failed duplicate transfer: same stored bytes are acknowledged
    // by storage::write_bytes, then actual push rejects the duplicate path.
    // The rejected acknowledgement stays in the local slot, not transport.
    let count = receipts.records.len();
    let mut rejected = OwnedSlot {
        slot: Slot::Result,
        bytes: slot.bytes.clone(),
        encoding_complete: true,
        write_attempted: false,
        root: None,
        transferred: false,
        error: None,
    };
    rejected.store(original, receipts);
    assert!(rejected.write_attempted);
    assert!(!rejected.transferred);
    assert!(rejected.root.is_some());
    assert!(rejected.error.is_some());
    assert_eq!(receipts.records.len(), count);
    assert!(transferred_result(&rejected, receipts, original, Slot::Result).is_err());
    assert!(transferred_result(slot, receipts, original, Slot::Result).is_ok());
}

// Serialization uses the output owner's separately prepaid bounded envelope.
// Proof/target/cost values remain in this restricted receipt only.
pub(super) fn restricted_attempt(attempt: &Attempt) -> serde_json::Value {
    let audit_value = |audit: &identity::Audit| {
        let result = match audit.result {
            Ok(d) => serde_json::json!({"kind":"equal", "target_degree":d.target,
                "max_quotient_summand_degree":d.max_quotient_summand,
                "max_before_quotient_summand_degree":d.max_before_quotient_summand}),
            Err(identity::Failure::Malformed(s)) => {
                serde_json::json!({"kind":"malformed", "syntax":format!("{s:?}")})
            }
            Err(identity::Failure::EqualityMismatch) => {
                serde_json::json!({"kind":"equality_mismatch"})
            }
            Err(identity::Failure::Unknown(e)) => {
                serde_json::json!({"kind":"unknown", "exhaustion":format!("{e:?}")})
            }
        };
        serde_json::json!({
            "result":result,
            "charges":audit.charges.used,
            "source_authenticated_by_arithmetic_checker":false,
            "native_operation_witness":false,
        })
    };
    let outcome = match &attempt.outcome {
        Ok(checked) => serde_json::json!({
            "status":"arithmetic_audit", "input_sha256":checked.input_sha256,
            "input_capture_sequence":checked.input_capture_sequence,
            "anchor_sha256":checked.result_sha256,"watermark":checked.watermark,
            "row_index":checked.row_index,"wire_hex":hex::encode(&checked.wire),
            "audit":audit_value(&checked.audit),
        }),
        Err(Refusal::PostCheck {
            reason,
            wire,
            audit,
        }) => serde_json::json!({
            "status":"post_check_refused","reason":format!("{reason:?}"),
            "wire_hex":hex::encode(wire),"audit":audit_value(audit),
        }),
        Err(reason) => serde_json::json!({"status":"refused","reason":format!("{reason:?}")}),
    };
    serde_json::json!({"before":attempt.before,"after":attempt.after,"outcome":outcome})
}

// Private fixed-size original-bound token. No constructor accepts supplied
// coefficients or an authentication flag; only admitted owners mint this.
struct BoundTarget {
    generators: [u8; 61],
    count: usize,
    target: u8,
    input_sha256: [u8; 32],
    input_capture_sequence: u64,
    anchor_sha256: String,
    watermark: capture::CommittedEventPosition,
    row_index: usize,
    selector_codes: [u64; 2],
}
fn coefficient_bits(p: &F2BoolPoly) -> Result<u8, Refusal> {
    if p.n_vars != 3 || p.terms.len() > 8 {
        return Err(Refusal::Shape);
    }
    let mut bits = 0u8;
    for term in &p.terms {
        if term.mask > 7 || bits & (1 << term.mask) != 0 {
            return Err(Refusal::Shape);
        }
        bits |= 1 << term.mask;
    }
    Ok(bits)
}
fn write_bits(out: &mut impl Write, bits: u8) -> Result<(), Refusal> {
    write!(out, " {}", bits.count_ones()).map_err(|_| Refusal::Encoding)?;
    for mask in [0u8, 1, 3, 7, 5, 2, 6, 4] {
        if bits & (1 << mask) == 0 {
            continue;
        }
        write!(out, " {}", mask.count_ones()).map_err(|_| Refusal::Encoding)?;
        for v in 0..3 {
            if mask & (1 << v) != 0 {
                write!(out, " {v}").map_err(|_| Refusal::Encoding)?;
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod produced_bound_tests {
    use super::*;
    #[test]
    fn largest_source_and_eight_summands_fit_and_reject_changed_targets() {
        // Exercise every N3 coefficient vector in a full 61-generator wire.
        // Independently compute each target by monomial union/XOR, without
        // calling the certificate producer or the checker to choose it.
        for seed in 0..=255u8 {
            let generators: Vec<u8> = (0..61).map(|i| seed.rotate_left(i % 8) ^ i as u8).collect();
            let mut target = 0u8;
            for (i, g) in generators.iter().take(8).enumerate() {
                for mono in 0..8u8 {
                    if g & (1 << mono) != 0 {
                        target ^= 1 << (mono | i as u8);
                    }
                }
            }
            for changed in [false, true] {
                let mut wire = Vec::new();
                write!(wire, "BQI1 3 61").unwrap();
                for bits in &generators {
                    write_bits(&mut wire, *bits).unwrap();
                }
                write_bits(&mut wire, target ^ u8::from(changed)).unwrap();
                write!(wire, " 8").unwrap();
                for i in 0..8u8 {
                    write!(wire, " {} {}", i, i.count_ones()).unwrap();
                    for v in 0..3 {
                        if i & (1 << v) != 0 {
                            write!(wire, " {v}").unwrap();
                        }
                    }
                }
                wire.push(b'\n');
                assert!(wire.len() <= PRODUCED_WIRE_CAP);
                let audit = identity::verify_untrusted(
                    &wire,
                    produced_limits(),
                    Instant::now() + std::time::Duration::from_secs(10),
                );
                if changed {
                    assert!(matches!(
                        audit.result,
                        Err(identity::Failure::EqualityMismatch)
                    ));
                } else {
                    assert!(audit.result.is_ok(), "{audit:?}");
                }
            }
        }
        let mut dense = Vec::new();
        write_bits(&mut dense, 255).unwrap();
        assert_eq!(dense.len(), 42);
        assert!(62 * dense.len() + 128 < PRODUCED_WIRE_CAP);
        let mut wire = b"BQI1 3 61".to_vec();
        for _ in 0..62 {
            wire.extend_from_slice(&dense);
        }
        wire.extend_from_slice(b" 8");
        for i in 0..8u8 {
            write!(wire, " {} {}", i, i.count_ones()).unwrap();
            for v in 0..3 {
                if i & (1 << v) != 0 {
                    write!(wire, " {v}").unwrap();
                }
            }
        }
        wire.push(b'\n');
        // The full N3 polynomial vanishes when multiplied by any nonconstant
        // Boolean monomial, so only the first, constant multiplier survives.
        let audit = identity::verify_untrusted(
            &wire,
            produced_limits(),
            Instant::now() + std::time::Duration::from_secs(10),
        );
        assert!(audit.result.is_ok(), "dense extremum: {audit:?}");
    }
}
// Only produce_and_check can call this with its private freshly minted token.
// Full anchor/source admission was paid and performed by mint_bound_target.
// Recheck the retained scalar source/target and watermark before independent
// arithmetic; do not decode/re-serialize the same acknowledged report twice.
fn check_bound_target(
    registry: &OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    target: &BoundTarget,
    suffix: &[u8],
) -> Attempt {
    let before = input.ledger.saved_ledger();
    let outcome = (|| -> Result<RestrictedCheck, Refusal> {
        let caps = produced_limits();
        let arithmetic = caps
            .caps
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != identity::Resource::AllocationBytes as usize)
            .map(|(_, n)| n)
            .sum::<u64>();
        // 8/byte wire encoding plus fixed scalar/source scans, head joins,
        // hash-string copies and metadata. All reserves precede the work.
        input
            .ledger
            .reserve_checking(
                arithmetic + 8 * PRODUCED_WIRE_CAP as u64 + 16384,
                caps.caps[identity::Resource::AllocationBytes as usize] + 65536,
                Instant::now(),
            )
            .map_err(Refusal::SharedBudget)?;
        if !same_owners(registry, input, events)
            || input.input_sha256 != target.input_sha256
            || input.capture_sequence != target.input_capture_sequence
            || input.math.codes != target.selector_codes
        {
            return Err(Refusal::Owner);
        }
        if current_head(events)? != target.watermark {
            return Err(Refusal::Watermark);
        }
        let built = input.retained.constructed.as_ref().ok_or(Refusal::Owner)?;
        let row = input
            .report
            .as_ref()
            .and_then(|r| r.rows.get(target.row_index))
            .ok_or(Refusal::Shape)?;
        if built.equations.len() != target.count || coefficient_bits(row)? != target.target {
            return Err(Refusal::Shape);
        }
        for (actual, expected) in built
            .equations
            .iter()
            .zip(&target.generators[..target.count])
        {
            if coefficient_bits(actual)? != *expected {
                return Err(Refusal::Shape);
            }
        }
        let mut wire = Vec::new();
        wire.try_reserve_exact(PRODUCED_WIRE_CAP)
            .map_err(|_| Refusal::Allocation)?;
        {
            let mut out = Capped {
                bytes: &mut wire,
                cap: PRODUCED_WIRE_CAP,
            };
            write!(out, "BQI1 3 {}", target.count).map_err(|_| Refusal::Encoding)?;
            for bits in target.generators[..target.count]
                .iter()
                .chain(std::iter::once(&target.target))
            {
                write_bits(&mut out, *bits)?;
            }
            out.write_all(suffix).map_err(|_| Refusal::Encoding)?;
        }
        let audit = identity::verify_untrusted(&wire, caps, input.ledger.deadline);
        if let Err(stop) = input.ledger.check(Instant::now()) {
            return Err(Refusal::PostCheck {
                reason: PostCheckFailure::SharedBudget(stop),
                wire,
                audit,
            });
        }
        if !current_head(events).is_ok_and(|head| head == target.watermark) {
            return Err(Refusal::PostCheck {
                reason: PostCheckFailure::Watermark,
                wire,
                audit,
            });
        }
        Ok(RestrictedCheck {
            input_sha256: target.input_sha256,
            input_capture_sequence: target.input_capture_sequence,
            result_sha256: target.anchor_sha256.clone(),
            watermark: target.watermark.clone(),
            row_index: target.row_index,
            wire,
            audit,
        })
    })();
    Attempt {
        before,
        after: input.ledger.saved_ledger(),
        outcome,
    }
}
// Call only after charging ADAPTER_WORK/ADAPTER_BYTES to the same ledger.
fn mint_bound_target(
    registry: &OrdinaryRootRegistry<'_>,
    input: &RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    row_index: usize,
    anchor: &OwnedSlot,
) -> Result<BoundTarget, Refusal> {
    if !same_owners(registry, input, events) {
        return Err(Refusal::Owner);
    }
    let root = transferred_result(
        anchor,
        &registry.receipts,
        registry.original,
        Slot::BooleanAnchor,
    )?;
    let watermark = current_head(events)?;
    if watermark != events.end_position {
        return Err(Refusal::Watermark);
    }
    let saved: serde_json::Value =
        serde_json::from_slice(&anchor.bytes).map_err(|_| Refusal::Receipt)?;
    let report = input.report.as_ref().ok_or(Refusal::Shape)?;
    if !bounded_report_shape(Some(report)) {
        return Err(Refusal::Shape);
    }
    let target = report.rows.get(row_index).ok_or(Refusal::Shape)?;
    if saved["schema"] != "crypto.ml-dataset.binary-m3-matrix-result/v4"
        || saved["input_sha256"]
            != serde_json::to_value(input.input_sha256).map_err(|_| Refusal::Encoding)?
        || saved["input_capture_sequence"] != input.capture_sequence
        || saved["report"] != serde_json::to_value(report).map_err(|_| Refusal::Encoding)?
    {
        return Err(Refusal::Receipt);
    }
    let built = input.retained.constructed.as_ref().ok_or(Refusal::Owner)?;
    let count = built.equations.len();
    if count == 0 || count > 61 || count != registry.original.config.n as usize {
        return Err(Refusal::Shape);
    }
    let mut generators = [0; 61];
    for (i, p) in built.equations.iter().enumerate() {
        generators[i] = coefficient_bits(p)?;
    }
    Ok(BoundTarget {
        generators,
        count,
        target: coefficient_bits(target)?,
        input_sha256: input.input_sha256,
        input_capture_sequence: input.capture_sequence,
        anchor_sha256: root.sha256.clone(),
        watermark,
        row_index,
        selector_codes: input.math.codes,
    })
}
pub(super) fn produce_and_check(
    registry: &OrdinaryRootRegistry<'_>,
    input: &mut RegisteredMatrixInput<'_>,
    events: &OrdinaryEvents<'_>,
    row_index: usize,
    anchor: &OwnedSlot,
) -> serde_json::Value {
    let before = input.ledger.saved_ledger();
    let admitted = (|| -> Result<BoundTarget, Refusal> {
        input
            .ledger
            .reserve_checking(ADAPTER_WORK, ADAPTER_BYTES, Instant::now())
            .map_err(Refusal::SharedBudget)?;
        let target = mint_bound_target(registry, input, events, row_index, anchor)?;
        let (work, bytes) = producer::quote(target.count).map_err(|_| Refusal::Shape)?;
        input
            .ledger
            .reserve_checking(work, bytes, Instant::now())
            .map_err(Refusal::SharedBudget)?;
        Ok(target)
    })();
    let target = match admitted {
        Ok(target) => target,
        Err(reason) => {
            return serde_json::json!({"status":"producer_admission_refused",
            "reason":format!("{reason:?}"),"before":before,"after":input.ledger.saved_ledger()})
        }
    };
    let (work, bytes) = producer::quote(target.count).expect("admitted bounded target");
    let produced = producer::produce(
        &target.generators[..target.count],
        target.target,
        work,
        bytes,
        input.ledger.deadline,
    );
    let produced_after = input.ledger.saved_ledger();
    // Retain full bounded producer partial state/cursor as diagnostic text;
    // it is not a trusted label or a native instruction trajectory.
    let retained_state = format!("{produced:?}");
    let unchanged = current_head(events).is_ok_and(|head| head == target.watermark)
        && input.input_sha256 == target.input_sha256
        && input.capture_sequence == target.input_capture_sequence
        && input.math.codes == target.selector_codes;
    let checked = match &produced.outcome {
        Ok(suffix) if unchanged => {
            Some(check_bound_target(registry, input, events, &target, suffix))
        }
        _ => None,
    };
    serde_json::json!({
        "status":if unchanged {"producer_attempt"}else{"producer_post_check_refused"},
        "derivation_origin":"separately_searched_boolean_generator_identity",
        "checker_profile":"bounded-n3-proof8/v2",
        "input_sha256":target.input_sha256,"input_capture_sequence":target.input_capture_sequence,
        "anchor_sha256":target.anchor_sha256,"watermark":target.watermark,"row_index":target.row_index,
        "selector_codes":target.selector_codes,"selector_convention":"vi selects code[bit] for summand i; three one-bit selectors",
        "before":before,"after_producer":produced_after,"after_check":input.ledger.saved_ledger(),
        "reserved_work":produced.reserved_work,"reserved_bytes":produced.reserved_bytes,
        "producer_work":produced.counts.work,"producer_candidates":produced.counts.candidates,
        "producer_terms":produced.counts.terms,"producer_pivots":produced.counts.pivots,
        "producer_row_xors":produced.counts.row_xors,"producer_provenance_steps":produced.counts.provenance_steps,
        "retained_producer_state_debug":retained_state,
        "check":checked.as_ref().map(restricted_attempt),
        "native_operation_witness":false,"training_export_admitted":false,
    })
}
