//! Proposed component, intentionally not registered in central replay yet.
//! Verifies one contiguous original-map -> initial-batches -> marker segment.
//! Full custody, original-file authentication and exporter release are separate
//! mandatory caller gates. No search, UNSAT, producer-cost or fall-degree claim.
#[path = "sat_initial_packed.rs"]
mod packed;
use crate::cryptanalysis::binary_semaev_chain_sat::provenance::{
    self, Admission, CheckedMap, Quote,
};
use crate::cryptanalysis::sat::{Lit, SolverEvent, SolverEventReason};
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    Cap,
    Admission,
    Decode,
    Identity,
    Range,
    Phase,
    Marker,
    Mathematics,
    Allocation,
}
#[derive(Clone, Copy)]
pub(super) struct Limits {
    pub line_bytes: usize,
    pub stream_bytes: u64,
    pub events: usize,
    pub batches: usize,
    pub decoded_bytes: usize,
    pub decoded_slots: usize,
    pub decoded_words: usize,
}
/// Caller must derive these from original authenticated files and the actual
/// map payload commit. They are not certified merely by constructing this value.
pub(super) struct Expected<'a> {
    pub binding_sha256: &'a str,
    pub original_binding: [u8; 32],
    pub pattern: u32,
    pub map_sequence: u64,
    pub map_event_sha256: &'a str,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
    schema: String,
    binding_sha256: String,
    sequence: u64,
    previous_event_sha256: String,
    event: Envelope,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    kind: String,
    payload: Payload,
}
#[derive(Deserialize)]
#[serde(tag = "schema", deny_unknown_fields)]
enum Payload {
    #[serde(rename = "ml-dataset.sat-initial-batch/v1")]
    Batch {
        original_binding: [u8; 32],
        pattern: u32,
        map_capture_sequence: u64,
        map_capture_sha256: String,
        first_native_index: usize,
        events: Vec<InitialEvent>,
    },
    #[serde(rename = "ml-dataset.sat-initial-batch/v3")]
    PackedBatch {
        original_binding: [u8; 32],
        pattern: u32,
        map_capture_sequence: u64,
        map_capture_sha256: String,
        first_native_index: usize,
        native_event_count: usize,
        events_packed: String,
    },
    #[serde(rename = "ml-dataset.sat-initial-checked/v1")]
    Checked {
        original_binding: [u8; 32],
        pattern: u32,
        map_capture_sequence: u64,
        map_capture_sha256: String,
        native_events: usize,
        native_capture_end_sequence: u64,
        native_capture_end_sha256: String,
        clauses: usize,
        xors: usize,
        roots: usize,
        insertion_contradiction: bool,
        search_checked: bool,
        unsat_proof_checked: bool,
    },
}
/// Closed initial-only schema instead of permissive SolverEvent deserialization.
/// It rejects extra fields, duplicate typed fields and all search variants.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum InitialEvent {
    Snapshot {
        version: u32,
        variables: u32,
        trail_limits: Vec<usize>,
        already_unsat: bool,
    },
    Clause {
        clause_id: u32,
        literals: Vec<Lit>,
        existing: bool,
        detached: bool,
    },
    XorConstraint {
        row_id: usize,
        mask: Vec<u64>,
        rhs: bool,
    },
    Assignment {
        literal: Lit,
        level: i32,
        trail_index: usize,
        initial: bool,
        reason: InitialReason,
    },
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum InitialReason {
    Clause { clause_id: u32 },
}
impl InitialEvent {
    fn into_native(self) -> SolverEvent {
        match self {
            Self::Snapshot {
                version,
                variables,
                trail_limits,
                already_unsat,
            } => SolverEvent::Snapshot {
                version,
                variables,
                trail_limits,
                already_unsat,
            },
            Self::Clause {
                clause_id,
                literals,
                existing,
                detached,
            } => SolverEvent::Clause {
                clause_id,
                literals,
                existing,
                detached,
            },
            Self::XorConstraint { row_id, mask, rhs } => {
                SolverEvent::XorConstraint { row_id, mask, rhs }
            }
            Self::Assignment {
                literal,
                level,
                trail_index,
                initial,
                reason: InitialReason::Clause { clause_id },
            } => SolverEvent::Assignment {
                literal,
                level,
                trail_index,
                initial,
                reason: SolverEventReason::Clause { clause_id },
            },
        }
    }
}
struct Marker {
    clauses: usize,
    xors: usize,
    roots: usize,
    contradiction: bool,
}
pub(super) struct Transcript {
    binding: String,
    original_binding: [u8; 32],
    pattern: u32,
    map_sequence: u64,
    map_hash: String,
    previous_sequence: u64,
    previous_hash: String,
    events: Vec<SolverEvent>,
    batches: usize,
    observed_bytes: u64,
    pending: Option<Wire>,
    marker: Option<Marker>,
    limits: Limits,
    first_error: Option<Error>,
    batch_version: Option<u8>,
    codec: packed::State,
    pending_binary: Vec<u8>,
    decoded_bytes: usize,
    decoded_slots: usize,
    decoded_words: usize,
}
/// Mathematical consistency of this transcript against a checked original map.
/// Not a release token for exporter callbacks or proof of producer costs.
pub(super) struct VerifiedInitialTranscript<'a> {
    initial: provenance::initial::VerifiedInitial<'a>,
    transcript: &'a Transcript,
}
fn admit(a: &mut impl Admission, work: u64, bytes: u64) -> Result<(), Error> {
    a.reserve(Quote { work, bytes })
        .map_err(|_| Error::Admission)?;
    a.check().map_err(|_| Error::Admission)
}
impl Transcript {
    /// Parsed marker position only; math and whole-capture custody still required.
    pub(super) fn marker_position(&self) -> Option<(u64, &str)> {
        if self.first_error.is_none() && self.marker.is_some() {
            Some((self.previous_sequence, &self.previous_hash))
        } else {
            None
        }
    }

    pub(super) fn new(
        expected: Expected<'_>,
        limits: Limits,
        a: &mut impl Admission,
    ) -> Result<Self, Error> {
        admit(a, 4096, 2048)?;
        if expected.binding_sha256.len() != 64
            || expected.map_event_sha256.len() != 64
            || !expected
                .binding_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
            || !expected
                .map_event_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
            || expected.map_sequence == 0
            || limits.line_bytes == 0
            || limits.events == 0
            || limits.batches == 0
        {
            return Err(Error::Identity);
        }
        Ok(Self {
            binding: expected.binding_sha256.into(),
            original_binding: expected.original_binding,
            pattern: expected.pattern,
            map_sequence: expected.map_sequence,
            map_hash: expected.map_event_sha256.into(),
            previous_sequence: expected.map_sequence,
            previous_hash: expected.map_event_sha256.into(),
            events: Vec::new(),
            batches: 0,
            observed_bytes: 0,
            pending: None,
            marker: None,
            limits,
            first_error: None,
            batch_version: None,
            codec: packed::State::default(),
            pending_binary: Vec::new(),
            decoded_bytes: 0,
            decoded_slots: 0,
            decoded_words: 0,
        })
    }
    pub(super) fn feed(&mut self, line: &[u8], a: &mut impl Admission) -> Result<(), Error> {
        if let Some(e) = self.first_error {
            return Err(e);
        }
        let result = self.feed_inner(line, a);
        if let Err(e) = result {
            self.first_error = Some(e);
        }
        result
    }
    fn feed_inner(&mut self, line: &[u8], a: &mut impl Admission) -> Result<(), Error> {
        if self.marker.is_some() || self.pending.is_some() {
            return Err(Error::Phase);
        }
        if line.len() > self.limits.line_bytes || !line.ends_with(b"\n") {
            return Err(Error::Cap);
        }
        let before_events = self.events.len();
        let next_bytes = self
            .observed_bytes
            .checked_add(line.len() as u64)
            .filter(|n| *n <= self.limits.stream_bytes)
            .ok_or(Error::Cap)?;
        // Preserve the inherited typed parse allocation quote: tagged Payload /
        // InitialEvent / InitialReason can buffer serde Content intermediates.
        // Generic Value shape alone is not yet an established bound for those.
        admit(
            a,
            (line.len() as u64)
                .checked_mul(256)
                .and_then(|n| n.checked_add(1024))
                .ok_or(Error::Cap)?,
            (line.len() as u64).checked_mul(128).ok_or(Error::Cap)?,
        )?;
        self.pending = Some(serde_json::from_slice(line).map_err(|_| Error::Decode)?);
        let row = self.pending.as_ref().expect("decoded row retained");
        if row.schema != "crypto.ml-dataset.live-event/v2"
            || row.binding_sha256 != self.binding
            || row.sequence != self.previous_sequence.checked_add(1).ok_or(Error::Range)?
            || row.previous_event_sha256 != self.previous_hash
            || row.event.kind != "binary_sat_initial"
        {
            return Err(Error::Identity);
        }
        let mut extra = 0;
        let mut version = None;
        match &row.event.payload {
            Payload::Batch {
                original_binding,
                pattern,
                map_capture_sequence,
                map_capture_sha256,
                first_native_index,
                events,
            } => {
                if *original_binding != self.original_binding
                    || *pattern != self.pattern
                    || *map_capture_sequence != self.map_sequence
                    || map_capture_sha256 != &self.map_hash
                {
                    return Err(Error::Identity);
                }
                if *first_native_index != self.events.len()
                    || events.is_empty()
                    || self.batches >= self.limits.batches
                {
                    return Err(Error::Range);
                }
                version = Some(1);
                extra = events.len();
                if self
                    .events
                    .len()
                    .checked_add(extra)
                    .filter(|n| *n <= self.limits.events)
                    .is_none()
                {
                    return Err(Error::Cap);
                }
            }
            Payload::PackedBatch {
                original_binding,
                pattern,
                map_capture_sequence,
                map_capture_sha256,
                first_native_index,
                native_event_count,
                ..
            } => {
                if *original_binding != self.original_binding
                    || *pattern != self.pattern
                    || *map_capture_sequence != self.map_sequence
                    || map_capture_sha256 != &self.map_hash
                {
                    return Err(Error::Identity);
                }
                if *first_native_index != self.events.len()
                    || *native_event_count == 0
                    || self.batches >= self.limits.batches
                {
                    return Err(Error::Range);
                }
                extra = *native_event_count;
                if self
                    .events
                    .len()
                    .checked_add(extra)
                    .filter(|n| *n <= self.limits.events)
                    .is_none()
                {
                    return Err(Error::Cap);
                }
                version = Some(3);
            }
            Payload::Checked {
                original_binding,
                pattern,
                map_capture_sequence,
                map_capture_sha256,
                native_events,
                native_capture_end_sequence,
                native_capture_end_sha256,
                search_checked,
                unsat_proof_checked,
                ..
            } => {
                if *original_binding != self.original_binding
                    || *pattern != self.pattern
                    || *map_capture_sequence != self.map_sequence
                    || map_capture_sha256 != &self.map_hash
                {
                    return Err(Error::Identity);
                }
                if self.batches == 0
                    || *native_events != self.events.len()
                    || *native_capture_end_sequence != self.previous_sequence
                    || native_capture_end_sha256 != &self.previous_hash
                    || *search_checked
                    || *unsat_proof_checked
                {
                    return Err(Error::Marker);
                }
            }
        }
        if let Some(version) = version {
            if self.batch_version.is_some_and(|v| v != version) {
                return Err(Error::Phase);
            }
        }
        if let Payload::PackedBatch { events_packed, .. } = &row.event.payload {
            // Encoded string is retained inside pending before every reservation.
            // Require canonical padded STANDARD input; malformed decode retains
            // both the string and the partially filled binary allocation.
            let n = events_packed.len();
            if n % 4 != 0 {
                return Err(Error::Decode);
            }
            let capacity = n
                .checked_div(4)
                .and_then(|n| n.checked_mul(3))
                .ok_or(Error::Cap)?;
            let total = self
                .decoded_bytes
                .checked_add(capacity)
                .filter(|n| *n <= self.limits.decoded_bytes)
                .ok_or(Error::Cap)?;
            admit(
                a,
                (n as u64)
                    .checked_mul(8)
                    .and_then(|n| n.checked_add(1024))
                    .ok_or(Error::Cap)?,
                capacity as u64,
            )?;
            // Preserve any prior successful bytes until the new reservation is accepted.
            self.pending_binary.clear();
            self.pending_binary
                .try_reserve_exact(capacity)
                .map_err(|_| Error::Allocation)?;
            self.pending_binary.resize(capacity, 0);
            let len = base64::engine::general_purpose::STANDARD
                .decode_slice(events_packed.as_bytes(), &mut self.pending_binary)
                .map_err(|_| Error::Decode)?;
            self.pending_binary.truncate(len);
            let before = self.events.len();
            packed::decode_into(
                &self.pending_binary,
                &mut self.codec,
                &mut self.events,
                packed::DecodeLimits {
                    events: before.checked_add(extra).ok_or(Error::Cap)?,
                    slots: self
                        .limits
                        .decoded_slots
                        .checked_sub(self.decoded_slots)
                        .ok_or(Error::Cap)?,
                    words: self
                        .limits
                        .decoded_words
                        .checked_sub(self.decoded_words)
                        .ok_or(Error::Cap)?,
                },
                &mut |work, bytes| {
                    a.reserve(Quote { work, bytes })
                        .map_err(|_| packed::DecodeError::Admission)?;
                    a.check().map_err(|_| packed::DecodeError::Admission)
                },
            )
            .map_err(|e| match e {
                packed::DecodeError::Admission => Error::Admission,
                packed::DecodeError::Cap => Error::Cap,
                _ => Error::Decode,
            })?;
            if self.events.len() != before + extra {
                return Err(Error::Range);
            }
            self.decoded_bytes = total;
        } else if extra != 0 {
            // Admission includes any relocation; geometric growth avoids quadratic copies.
            let needed = self.events.len().checked_add(extra).ok_or(Error::Cap)?;
            let capacity = if needed > self.events.capacity() {
                needed
                    .max(
                        self.events
                            .capacity()
                            .checked_mul(2)
                            .unwrap_or(self.limits.events),
                    )
                    .min(self.limits.events)
            } else {
                0
            };
            let work = (extra as u64)
                .checked_mul(16)
                .and_then(|n| n.checked_add((self.events.len() as u64).checked_mul(4)?))
                .ok_or(Error::Cap)?;
            let bytes = (capacity as u64)
                .checked_mul(std::mem::size_of::<SolverEvent>() as u64)
                .ok_or(Error::Cap)?;
            admit(a, work, bytes)?;
            if capacity > 0 {
                self.events
                    .try_reserve_exact(capacity - self.events.len())
                    .map_err(|_| Error::Allocation)?;
            }
        }
        let mut next_slots = self.decoded_slots;
        let mut next_words = self.decoded_words;
        if extra > 0 {
            admit(a, (extra as u64).checked_mul(16).ok_or(Error::Cap)?, 0)?;
            let mut add_lengths = |slots: usize, words: usize| -> Result<(), Error> {
                next_slots = next_slots
                    .checked_add(slots)
                    .filter(|n| *n <= self.limits.decoded_slots)
                    .ok_or(Error::Cap)?;
                next_words = next_words
                    .checked_add(words)
                    .filter(|n| *n <= self.limits.decoded_words)
                    .ok_or(Error::Cap)?;
                Ok(())
            };
            match &self.pending.as_ref().expect("retained row").event.payload {
                Payload::Batch { events, .. } => {
                    for event in events {
                        match event {
                            InitialEvent::Snapshot { trail_limits, .. } => {
                                add_lengths(trail_limits.len(), 0)?
                            }
                            InitialEvent::Clause { literals, .. } => {
                                add_lengths(literals.len(), 0)?
                            }
                            InitialEvent::XorConstraint { mask, .. } => add_lengths(0, mask.len())?,
                            InitialEvent::Assignment { .. } => {}
                        }
                    }
                }
                Payload::PackedBatch { .. } => {
                    for event in &self.events[before_events..] {
                        match event {
                            SolverEvent::Snapshot { trail_limits, .. } => {
                                add_lengths(trail_limits.len(), 0)?
                            }
                            SolverEvent::Clause { literals, .. } => add_lengths(literals.len(), 0)?,
                            SolverEvent::XorConstraint { mask, .. } => add_lengths(0, mask.len())?,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        // No fallible operations after moving typed event owners into the prefix.
        let row = self.pending.take().expect("checked row retained");
        match row.event.payload {
            Payload::Batch { events, .. } => {
                self.events
                    .extend(events.into_iter().map(InitialEvent::into_native));
                self.batches += 1;
            }
            Payload::PackedBatch { .. } => {
                self.batches += 1;
            }
            Payload::Checked {
                clauses,
                xors,
                roots,
                insertion_contradiction,
                ..
            } => {
                self.marker = Some(Marker {
                    clauses,
                    xors,
                    roots,
                    contradiction: insertion_contradiction,
                });
            }
        }
        self.decoded_slots = next_slots;
        self.decoded_words = next_words;
        if version.is_some() {
            self.batch_version = version;
        }
        self.previous_sequence = row.sequence;
        self.previous_hash = hex::encode(Sha256::digest(line));
        self.observed_bytes = next_bytes;
        Ok(())
    }
    pub(super) fn verify<'a>(
        &'a self,
        map: &'a CheckedMap,
        limits: provenance::initial::Limits,
        a: &mut impl Admission,
    ) -> Result<VerifiedInitialTranscript<'a>, Error> {
        if let Some(e) = self.first_error {
            return Err(e);
        }
        if map.original_binding() != self.original_binding || map.pattern() != self.pattern {
            return Err(Error::Identity);
        }
        let marker = self.marker.as_ref().ok_or(Error::Phase)?;
        let initial = provenance::initial::verify_initial(map, &self.events, limits, a)
            .map_err(|_| Error::Mathematics)?;
        if initial.clause_count() != marker.clauses
            || initial.xor_count() != marker.xors
            || initial.root_count() != marker.roots
            || initial.insertion_contradiction() != marker.contradiction
        {
            return Err(Error::Marker);
        }
        Ok(VerifiedInitialTranscript {
            initial,
            transcript: self,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use crate::cryptanalysis::sat::Solver;
    use num_bigint::BigUint;
    use serde_json::json;
    use std::sync::{Arc, Mutex};
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
        refuse: bool,
        calls: usize,
        refuse_at: Option<usize>,
    }
    impl Admission for Ledger {
        fn reserve(&mut self, q: Quote) -> Result<(), provenance::Error> {
            self.calls += 1;
            if self.refuse || self.refuse_at == Some(self.calls) {
                return Err(provenance::Error::Admission);
            }
            self.work = self.work.checked_add(q.work).unwrap();
            self.bytes = self.bytes.checked_add(q.bytes).unwrap();
            Ok(())
        }
        fn check(&mut self) -> Result<(), provenance::Error> {
            Ok(())
        }
    }
    fn wire(sequence: u64, previous: &str, payload: serde_json::Value) -> Vec<u8> {
        let mut bytes = serde_json::to_vec(&json!({
            "schema":"crypto.ml-dataset.live-event/v2", "binding_sha256":"a".repeat(64),
            "sequence":sequence, "previous_event_sha256":previous,
            "event":{"kind":"binary_sat_initial", "payload":payload}
        }))
        .unwrap();
        bytes.push(b'\n');
        bytes
    }
    fn decoder(a: &mut Ledger) -> Transcript {
        Transcript::new(
            Expected {
                binding_sha256: &"a".repeat(64),
                original_binding: [3; 32],
                pattern: 0,
                map_sequence: 1,
                map_event_sha256: &"b".repeat(64),
            },
            Limits {
                line_bytes: 4_000_000,
                stream_bytes: 8_000_000,
                events: 100_000,
                batches: 8,
                decoded_bytes: 8_000_000,
                decoded_slots: 1_000_000,
                decoded_words: 1_000_000,
            },
            a,
        )
        .unwrap()
    }
    fn initial_limits() -> provenance::initial::Limits {
        provenance::initial::Limits {
            work: u64::MAX,
            bytes: u64::MAX,
            events: 100_000,
            constraints: 100_000,
        }
    }
    #[test]
    fn imported_real_native_snapshot_round_trip_and_mutation_controls() {
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let b = F2mElement::from_bit_positions(&[1], 3);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        let original = provenance::Original {
            binding: [3; 32],
            a: &a,
            b: &b,
            irr: &irr,
            twist: true,
            m: 4,
            target: Some(&a),
            pattern: 0,
            codes: &codes,
        };
        let caps = provenance::Limits {
            variables: 10000,
            codes: 8,
            domain_clauses: 10000,
            work: u64::MAX,
            bytes: u64::MAX,
        };
        let mut ledger = Ledger::default();
        let built = provenance::build_original(&original, caps, &mut ledger).unwrap();
        let bytes = serde_json::to_vec(built.map()).unwrap();
        let imported: provenance::Map = serde_json::from_slice(&bytes).unwrap();
        let checked = provenance::check_imported(&original, imported, caps, &mut ledger).unwrap();
        let mut solver = Solver::new(checked.variables());
        for c in checked.formula() {
            match c {
                provenance::Constraint::Clause { literals } => {
                    solver.add_clause(literals.clone());
                }
                provenance::Constraint::Xor { wires, rhs } => {
                    solver.add_xor(wires, *rhs);
                }
            }
        }
        let observed = Arc::new(Mutex::new(Vec::new()));
        let sink = observed.clone();
        solver
            .set_event_observer(move |e| {
                sink.lock().unwrap().push(e.clone());
                Ok(())
            })
            .unwrap();
        drop(solver);
        let events = Arc::try_unwrap(observed).unwrap().into_inner().unwrap();
        let seal =
            provenance::initial::verify_initial(&checked, &events, initial_limits(), &mut ledger)
                .unwrap();
        let batch = json!({ "schema":"ml-dataset.sat-initial-batch/v1",
            "original_binding":([3u8;32]),"pattern":0,"map_capture_sequence":1,
            "map_capture_sha256":"b".repeat(64),"first_native_index":0,"events":events });
        let line = wire(2, &"b".repeat(64), batch.clone());
        let digest = hex::encode(Sha256::digest(&line));
        let marker = json!({"schema":"ml-dataset.sat-initial-checked/v1",
            "original_binding":([3u8;32]),"pattern":0,"map_capture_sequence":1,
            "map_capture_sha256":"b".repeat(64),"native_events":events.len(),
            "native_capture_end_sequence":2,"native_capture_end_sha256":digest,
            "clauses":seal.clause_count(),"xors":seal.xor_count(),"roots":seal.root_count(),
            "insertion_contradiction":seal.insertion_contradiction(),
            "search_checked":false,"unsat_proof_checked":false});
        let marker_line = wire(3, &digest, marker.clone());
        let mut transcript = decoder(&mut ledger);
        transcript.feed(&line, &mut ledger).unwrap();
        transcript.feed(&marker_line, &mut ledger).unwrap();
        let verified = transcript
            .verify(&checked, initial_limits(), &mut ledger)
            .unwrap();
        assert_eq!(verified.initial.events(), events.as_slice());
        assert_eq!(verified.transcript.previous_sequence, 3);
        for mutation in 0..6 {
            let mut changed = batch.clone();
            match mutation {
                0 => changed["pattern"] = json!(1),
                1 => changed["first_native_index"] = json!(1),
                2 => changed["map_capture_sha256"] = json!("c".repeat(64)),
                3 => changed["unexpected"] = json!(true),
                4 => changed["events"][0]["unexpected"] = json!(true),
                _ => changed["events"][0]["kind"] = json!("decision"),
            }
            let mut bad = decoder(&mut ledger);
            let before = ledger.work;
            assert!(bad
                .feed(&wire(2, &"b".repeat(64), changed), &mut ledger)
                .is_err());
            assert!(ledger.work > before);
            assert!(bad.feed(&line, &mut ledger).is_err()); // sticky; no repair/reset
        }
        // V3 preserves the actual observed native stream across batch boundaries.
        let split = events.len() / 2;
        assert!(split > 0 && split < events.len());
        let mut second_state = packed::State::default();
        for event in &events[..split] {
            packed::encode(event, &mut second_state, &mut |_| Ok::<_, ()>(())).unwrap();
        }
        let packed_batch = |slice: &[SolverEvent], state: packed::State, first: usize| {
            let mut value = batch.clone();
            value["schema"] = json!("ml-dataset.sat-initial-batch/v3");
            value.as_object_mut().unwrap().remove("events");
            value["first_native_index"] = json!(first);
            value["native_event_count"] = json!(slice.len());
            value["events_packed"] = serde_json::to_value(packed::Packed {
                events: slice,
                state,
            })
            .unwrap();
            value
        };
        let first = packed_batch(&events[..split], packed::State::default(), 0);
        let second = packed_batch(&events[split..], second_state, split);
        let first_wire = wire(2, &"b".repeat(64), first.clone());
        let first_hash = hex::encode(Sha256::digest(&first_wire));
        let second_wire = wire(3, &first_hash, second);
        let second_hash = hex::encode(Sha256::digest(&second_wire));
        let mut packed_marker = marker.clone();
        packed_marker["native_capture_end_sequence"] = json!(3);
        packed_marker["native_capture_end_sha256"] = json!(second_hash);
        let mut replay = decoder(&mut ledger);
        replay.feed(&first_wire, &mut ledger).unwrap();
        replay.feed(&second_wire, &mut ledger).unwrap();
        replay
            .feed(&wire(4, &second_hash, packed_marker), &mut ledger)
            .unwrap();
        let verified = replay
            .verify(&checked, initial_limits(), &mut ledger)
            .unwrap();
        assert_eq!(verified.initial.events(), events.as_slice());
        assert_eq!(replay.batch_version, Some(3));
        for bad_text in ["!!!!", "AB==", "AA", "gA=="] {
            let mut bad = decoder(&mut ledger);
            let mut changed = first.clone();
            changed["events_packed"] = json!(bad_text);
            let before = ledger.work;
            assert!(bad
                .feed(&wire(2, &"b".repeat(64), changed), &mut ledger)
                .is_err());
            assert!(bad.pending.is_some());
            assert!(ledger.work > before);
            assert!(bad.feed(&first_wire, &mut ledger).is_err());
        }
        let mut bad = decoder(&mut ledger);
        let mut changed = first.clone();
        changed["native_event_count"] = json!(split + 1);
        assert!(matches!(
            bad.feed(&wire(2, &"b".repeat(64), changed), &mut ledger),
            Err(Error::Range)
        ));
        assert_eq!(bad.events.len(), split);
        assert!(!bad.pending_binary.is_empty());
        assert!(bad.pending.is_some());
        let mut bad = decoder(&mut ledger);
        ledger.refuse_at = Some(ledger.calls + 6);
        let paid = ledger.work;
        assert!(matches!(
            bad.feed(&first_wire, &mut ledger),
            Err(Error::Admission)
        ));
        assert!(ledger.work > paid);
        assert!(bad.pending.is_some());
        assert!(!bad.pending_binary.is_empty());
        assert!(!bad.events.is_empty());
        ledger.refuse_at = None;
        let mut bad = decoder(&mut ledger);
        bad.feed(&first_wire, &mut ledger).unwrap();
        bad.limits.decoded_slots = bad.decoded_slots;
        assert!(matches!(
            bad.feed(&second_wire, &mut ledger),
            Err(Error::Cap)
        ));
        assert!(bad.pending.is_some());
        assert!(!bad.pending_binary.is_empty());
        let mut bad = decoder(&mut ledger);
        bad.feed(&first_wire, &mut ledger).unwrap();
        let mut legacy = batch.clone();
        legacy["first_native_index"] = json!(split);
        legacy["events"] = json!(&events[split..]);
        assert!(matches!(
            bad.feed(&wire(3, &first_hash, legacy), &mut ledger),
            Err(Error::Phase)
        ));
        let mut wrong = marker.clone();
        wrong["roots"] = json!(seal.root_count() + 1);
        let mut bad = decoder(&mut ledger);
        bad.feed(&line, &mut ledger).unwrap();
        bad.feed(&wire(3, &digest, wrong), &mut ledger).unwrap();
        assert!(matches!(
            bad.verify(&checked, initial_limits(), &mut ledger),
            Err(Error::Marker)
        ));
        let mut bad = decoder(&mut ledger);
        bad.feed(&line, &mut ledger).unwrap();
        ledger.refuse = true;
        let before = (ledger.work, ledger.bytes);
        assert!(matches!(
            bad.feed(&marker_line, &mut ledger),
            Err(Error::Admission)
        ));
        assert_eq!((ledger.work, ledger.bytes), before);
        assert_eq!(bad.events.len(), events.len());
        ledger.refuse = false;
        let imported = serde_json::from_slice(&bytes).unwrap();
        let foreign = provenance::Original {
            binding: [4; 32],
            ..original
        };
        let failed = provenance::check_imported(&foreign, imported, caps, &mut ledger).unwrap_err();
        assert!(failed.retained.is_some());
    }
}
