//! Actual native Solver installation and fresh-observer capture. This stage
//! deliberately exposes no solve method: search-prefix admission comes next.
#[path = "sat_initial_checkpoint.rs"]
pub(in crate::ml_dataset::solvers) mod checkpoint;
#[path = "sat_initial_packed.rs"]
mod packed;
use super::*;
use crate::cryptanalysis::sat::{Solver, SolverEvent, SolverEventReason};
use serde::Serialize;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
struct Quote {
    work: u64,
    bytes: u64,
    events: usize,
}
fn plus(a: u64, b: u64) -> Result<u64, Stop> {
    a.checked_add(b).ok_or(Stop::Overflow)
}
fn times(a: u64, b: u64) -> Result<u64, Stop> {
    a.checked_mul(b).ok_or(Stop::Overflow)
}
/// Version-bound structural upper bound for this Solver's initial installation,
/// including geometric vector capacity, snapshot transient clones, retained
/// observer copies and concurrently live capture hash/metadata scratch. It is
/// not an instruction count. No search allocation is covered by this quote.
fn quote(
    map: &provenance::CheckedMap,
    budget: &mut Budget,
    limits: &capture::CaptureLimits,
) -> Result<Quote, Stop> {
    budget.reserve_checking(1024, 0, Instant::now())?;
    let event_cap = limits.max_event_bytes;
    let n = map.variables() as u64;
    let words = plus(n, 63)? / 64;
    let (mut clauses, mut lits, mut square, mut xors, mut wires) = (0, 0, 0, 0, 0);
    for row in map.formula() {
        budget.reserve_checking(16, 0, Instant::now())?;
        match row {
            provenance::Constraint::Clause { literals } => {
                let k = literals.len() as u64;
                clauses = plus(clauses, 1)?;
                lits = plus(lits, k)?;
                square = plus(square, times(k, k)?)?;
            }
            provenance::Constraint::Xor { wires: row, .. } => {
                xors = plus(xors, 1)?;
                wires = plus(wires, row.len() as u64)?;
            }
        }
    }
    let events = plus(plus(plus(1, clauses)?, xors)?, n)?;
    let event_owner = plus(
        (2 * std::mem::size_of::<SolverEvent>() + 2 * std::mem::size_of::<BatchAck>()) as u64,
        1024,
    )?;
    let mut bytes = plus(
        std::mem::size_of::<Solver>() as u64,
        plus(
            32 * 1024 + 2 * 1024 * 1024,
            plus(
                times(limits.metadata_reserve_bytes, 2)?,
                times(event_cap, 2)?,
            )?,
        )?,
    )?;
    for part in [
        times(n, 512)?,
        times(lits, 64)?,
        times(clauses, 128)?,
        times(xors, plus(128, times(words, 32)?)?)?,
        times(events, event_owner)?,
        capture::TypedEventBuffer::requested_bytes(event_cap).map_err(|_| Stop::Bytes)?,
    ] {
        bytes = plus(bytes, part)?;
    }
    let mut work = 0;
    for part in [
        times(n, 64)?,
        times(square, 8)?,
        times(lits, 32)?,
        times(wires, 32)?,
        times(times(xors, words)?, 32)?,
        // Four packed passes each scan dense masks to count/emit set bits.
        // Literal/varint/base64 traversal remains charged per actual wire byte.
        times(times(xors, words)?, 32)?,
        // Three serializer closed-variant scans; no uncharged event walk.
        times(events, 6)?,
    ] {
        work = plus(work, part)?;
    }

    bytes = plus(bytes, 256)?; // two codec states and constant stack scratch
    Ok(Quote {
        work,
        bytes,
        events: usize::try_from(events).map_err(|_| Stop::Overflow)?,
    })
}

/// All original evidence remains owned across native or capture failure.
struct Evidence<'a> {
    original: Option<&'a VerifiedWorkerOriginal<'a>>,
    original_hashes: ([u8; 32], [u8; 32]),
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
}
struct BatchAck {
    valid: bool,
    first: usize,
    last: usize,
    position: capture::CommittedEventPosition,
}
struct CaptureState {
    budget: Budget,
    sink: capture::CaptureEmitter,
    limits: capture::CaptureLimits,
    binding: [u8; 32],
    pattern: u32,
    map_ack: capture::CommittedEventPosition,
    events: Vec<SolverEvent>,
    acks: Vec<BatchAck>,
    pending: Option<SolverEvent>,
    buffer: Option<capture::TypedEventBuffer>,
    event_limit: usize,
    chunk_start: usize,
    chunk_bytes: usize,
    chunk_binary: usize,
    codec_state: packed::State,
    chunk_codec_start: packed::State,
    closed: bool,
    sealed: bool,
    seal_ack: Option<capture::CommittedEventPosition>,
}
#[allow(clippy::large_enum_variant)]
pub(super) enum Failure<'a> {
    BeforeNative {
        owner: OriginalOwner<'a>,
        sink: capture::CaptureEmitter,
        stop: Stop,
    },
    Retained {
        stage: NativeInitial<'a>,
        stop: Stop,
    },
}
pub(super) struct NativeInitial<'a> {
    evidence: Evidence<'a>,
    shared: Arc<Mutex<CaptureState>>,
    solver: Option<Solver>,
}

#[derive(Serialize)]
struct Batch<'a> {
    schema: &'static str,
    original_binding: [u8; 32],
    pattern: u32,
    map_capture_sequence: u64,
    map_capture_sha256: &'a str,
    first_native_index: usize,
    native_event_count: usize,
    events_packed: packed::Packed<'a>,
}
#[derive(Serialize)]
struct InitialChecked<'a> {
    schema: &'static str,
    original_binding: [u8; 32],
    pattern: u32,
    map_capture_sequence: u64,
    map_capture_sha256: &'a str,
    native_events: usize,
    native_capture_end_sequence: u64,
    native_capture_end_sha256: &'a str,
    clauses: usize,
    xors: usize,
    roots: usize,
    insertion_contradiction: bool,
    search_checked: bool,
    unsat_proof_checked: bool,
}
/// Capture's own bounded internal counting pass is paid from this prior trusted
/// payload count plus a fixed envelope ceiling. The second pass is admitted at
/// its actual counted size. Fixed live scratch was prepaid by quote().
struct Publication {
    position: capture::CommittedEventPosition,
    valid: bool,
}
fn publish<T: Serialize + ?Sized>(
    budget: &mut Budget,
    sink: &mut capture::CaptureEmitter,
    buffer: &mut capture::TypedEventBuffer,
    cap: usize,
    payload: &T,
) -> Result<Publication, Stop> {
    let mut count = Count {
        budget,
        len: 0,
        cap,
    };
    serde_json::to_writer(&mut count, payload).map_err(|_| Stop::Serialization)?;
    let envelope = plus(count.len as u64, 512)?;
    budget.reserve_checking(times(envelope, 8)?, 0, Instant::now())?;
    let before = sink.committed_event_position();
    let next = before.event_count.checked_add(1).ok_or(Stop::Overflow)?;
    let ledger = RefCell::new(budget);
    let result = sink.event_typed_admit_encoding_with_limit(
        capture::TypedEventKind::BinarySatInitial,
        payload,
        buffer,
        cap,
        &mut |n| {
            ledger
                .borrow_mut()
                .reserve_checking(
                    (n as u64).checked_mul(8).ok_or_else(|| {
                        capture::CaptureError::Integrity("SAT encoding overflow".into())
                    })?,
                    0,
                    Instant::now(),
                )
                .map_err(|_| capture::CaptureError::Integrity("SAT encoding refused".into()))
        },
        &mut |bytes| {
            let n = bytes.len() as u64;
            let mut budget = ledger.borrow_mut();
            budget
                .reserve_checking(
                    n.checked_mul(32).and_then(|x| x.checked_add(65536)).ok_or(
                        capture::CaptureError::Integrity("SAT capture overflow".into()),
                    )?,
                    0,
                    Instant::now(),
                )
                .map_err(|_| capture::CaptureError::Integrity("SAT capture refused".into()))?;
            budget
                .reserve_trace(
                    n.checked_mul(4).and_then(|x| x.checked_add(4096)).ok_or(
                        capture::CaptureError::Integrity("SAT trace overflow".into()),
                    )?,
                    Instant::now(),
                )
                .map_err(|_| capture::CaptureError::Integrity("SAT trace refused".into()))
        },
    );
    result.map_err(|_| Stop::Observer)?;
    let after = sink.committed_event_position();
    // Success is an actual appended event. Caller stores this receipt without
    // another deadline gate. These checks must not throw away a durable receipt.
    let valid = after.event_count == next
        && after.binding_sha256 == before.binding_sha256
        && after.byte_end > before.byte_end;
    Ok(Publication {
        position: after,
        valid,
    })
}
impl CaptureState {
    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop> {
        self.budget.fail(stop)
    }
    fn flush(&mut self) -> Result<(), Stop> {
        if self.chunk_start == self.events.len() {
            return Ok(());
        }
        let record = Batch {
            schema: "ml-dataset.sat-initial-batch/v3",
            original_binding: self.binding,
            pattern: self.pattern,
            map_capture_sequence: self.map_ack.event_count,
            map_capture_sha256: &self.map_ack.last_event_sha256,
            first_native_index: self.chunk_start,
            native_event_count: self.events.len() - self.chunk_start,
            events_packed: packed::Packed {
                events: &self.events[self.chunk_start..],
                state: self.chunk_codec_start,
            },
        };
        self.budget.reserve_checking(16, 0, Instant::now())?; // packed string quotes
        let position = publish(
            &mut self.budget,
            &mut self.sink,
            self.buffer.as_mut().ok_or(Stop::Phase)?,
            self.limits.max_event_bytes as usize,
            &record,
        )?;
        let valid = position.valid;
        self.acks.push(BatchAck {
            first: self.chunk_start,
            last: self.events.len(),
            position: position.position,
            valid,
        });
        if !valid {
            return Err(Stop::Observer);
        }
        self.chunk_start = self.events.len();
        self.chunk_bytes = 0;
        self.chunk_binary = 0;
        self.chunk_codec_start = self.codec_state;
        Ok(())
    }
    fn on_event(&mut self, event: &SolverEvent) -> Result<(), Stop> {
        let result = self.on_event_inner(event);
        match result {
            Ok(()) => Ok(()),
            Err(s) => self.fail(s),
        }
    }
    fn on_event_inner(&mut self, event: &SolverEvent) -> Result<(), Stop> {
        if self.closed || self.pending.is_some() || self.events.len() >= self.event_limit {
            return Err(Stop::Events);
        }
        // Fresh installation emits only these records. A later observer or
        // search event cannot silently extend this initial-snapshot certificate.
        match event {
            SolverEvent::Snapshot { trail_limits, .. } if trail_limits.is_empty() => {}
            SolverEvent::Clause {
                existing: true,
                detached: false,
                ..
            } => {}
            SolverEvent::XorConstraint { .. } => {}
            SolverEvent::Assignment {
                initial: true,
                reason: SolverEventReason::Clause { .. },
                ..
            } => {}
            _ => return Err(Stop::Phase),
        }
        // All transient and retained copies were quoted against actual formula
        // dimensions before native attachment. Install pending ownership first.
        self.pending = Some(event.clone());
        self.budget.check(Instant::now())?;
        let cap = self.limits.max_event_bytes as usize;
        let usable = cap.checked_sub(1024).ok_or(Stop::Bytes)?;
        let mut next_codec = self.codec_state;
        let mut binary = 0usize;
        let previous_binary = self.chunk_binary;
        let event = self.pending.as_ref().expect("pending event installed");
        // Prepay each four base64 characters when its first source byte arrives.
        // This includes eventual final padding; there is no temporary byte Vec.
        packed::encode(event, &mut next_codec, &mut |_| -> Result<(), Stop> {
            if previous_binary.checked_add(binary).ok_or(Stop::Overflow)? % 3 == 0 {
                self.budget.reserve_checking(32, 0, Instant::now())?;
            }
            binary = binary.checked_add(1).ok_or(Stop::Overflow)?;
            let alone = binary
                .checked_add(2)
                .and_then(|n| n.checked_div(3))
                .and_then(|n| n.checked_mul(4))
                .and_then(|n| n.checked_add(2))
                .ok_or(Stop::Overflow)?;
            if alone > usable {
                return Err(Stop::Bytes);
            }
            Ok(())
        })?;
        let packed_len = |n: usize| {
            n.checked_add(2)
                .and_then(|n| n.checked_div(3))
                .and_then(|n| n.checked_mul(4))
                .and_then(|n| n.checked_add(2))
                .ok_or(Stop::Overflow)
        };
        let combined = self
            .chunk_binary
            .checked_add(binary)
            .ok_or(Stop::Overflow)?;
        if packed_len(combined)? > usable {
            self.flush()?;
            // Splitting base64 groups can require one additional padded group.
            self.budget.reserve_checking(32, 0, Instant::now())?;
        }
        let total = self
            .chunk_binary
            .checked_add(binary)
            .ok_or(Stop::Overflow)?;
        let wire_len = packed_len(total)?;
        self.events
            .push(self.pending.take().expect("pending event owned"));
        self.codec_state = next_codec;
        self.chunk_binary = total;
        self.chunk_bytes = wire_len;
        Ok(())
    }
}

impl<'a> NativeInitial<'a> {
    pub(super) fn prepare(
        mut owner: OriginalOwner<'a>,
        sink: capture::CaptureEmitter,
        checker_limits: provenance::initial::Limits,
    ) -> Result<Self, Failure<'a>> {
        let preflight = (|| -> Result<Quote, Stop> {
            owner.budget.reserve_checking(8192, 8192, Instant::now())?;
            if !owner.map_published {
                return Err(Stop::Phase);
            }
            sink.verify_directory(&owner.original.root.join("live"))
                .map_err(|_| Stop::Observer)?;
            sink.verify_attempt_limits(
                owner.original.binding.ok_or(Stop::InvalidLimits)?,
                &owner.capture_limits,
            )
            .map_err(|_| Stop::Observer)?;
            if Some(&sink.committed_event_position()) != owner.map_ack.as_ref() {
                return Err(Stop::Observer);
            }
            let q = quote(
                owner.checked.as_ref().ok_or(Stop::Phase)?,
                &mut owner.budget,
                &owner.capture_limits,
            )?;
            owner
                .budget
                .reserve_checking(q.work, q.bytes, Instant::now())?;
            Ok(q)
        })();
        let q = match preflight {
            Ok(q) => q,
            Err(stop) => {
                let stop = owner.budget.fail::<()>(stop).unwrap_err();
                return Err(Failure::BeforeNative { owner, sink, stop });
            }
        };
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
        } = owner;
        let shared = Arc::new(Mutex::new(CaptureState {
            budget,
            sink,
            limits: capture_limits,
            binding,
            pattern: pattern.expect("published map has pattern"),
            map_ack: map_ack.as_ref().expect("published map has receipt").clone(),
            events: Vec::new(),
            acks: Vec::new(),
            pending: None,
            buffer: None,
            event_limit: q.events,
            chunk_start: 0,
            chunk_bytes: 0,
            chunk_binary: 0,
            codec_state: packed::State::default(),
            chunk_codec_start: packed::State::default(),
            closed: false,
            sealed: false,
            seal_ack: None,
        }));
        let mut stage = Self {
            evidence: Evidence {
                original_hashes: (original.public_sha256, original.factor_base_sha256),
                original: Some(original),
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
            },
            shared,
            solver: None,
        };
        match stage.prepare_inner(checker_limits) {
            Ok(()) => Ok(stage),
            Err(stop) => {
                let mut state = stage.shared.lock().unwrap_or_else(|e| e.into_inner());
                let stop = state.budget.fail::<()>(stop).unwrap_err();
                drop(state);
                Err(Failure::Retained { stage, stop })
            }
        }
    }
    fn prepare_inner(&mut self, checker_limits: provenance::initial::Limits) -> Result<(), Stop> {
        {
            let mut state = self.shared.lock().map_err(|_| Stop::Observer)?;
            let count = state.event_limit;
            state
                .events
                .try_reserve_exact(count)
                .map_err(|_| Stop::Allocation)?;
            state
                .acks
                .try_reserve_exact(count)
                .map_err(|_| Stop::Allocation)?;
            let cap = state.limits.max_event_bytes;
            state.buffer = Some(
                capture::TypedEventBuffer::try_new(
                    cap,
                    capture::TypedEventBuffer::requested_bytes(cap).map_err(|_| Stop::Bytes)?,
                )
                .map_err(|_| Stop::Allocation)?,
            );
            state.budget.check(Instant::now())?;
        }
        let map = self.evidence.checked.as_ref().ok_or(Stop::Phase)?;
        self.solver = Some(Solver::new(map.variables()));
        let solver = self.solver.as_mut().expect("native solver installed");
        solver.deadline = Some(
            self.shared
                .lock()
                .map_err(|_| Stop::Observer)?
                .budget
                .deadline,
        );
        install_formula(solver, map, &self.shared)?;
        attach_initial(solver, &self.shared)?;
        seal_initial(map, &self.shared, checker_limits)
    }
}
fn install_formula(
    solver: &mut Solver,
    map: &provenance::CheckedMap,
    shared: &Arc<Mutex<CaptureState>>,
) -> Result<(), Stop> {
    for row in map.formula() {
        shared
            .lock()
            .map_err(|_| Stop::Observer)?
            .budget
            .check(Instant::now())?;
        match row {
            provenance::Constraint::Clause { literals } => {
                solver.add_clause(literals.clone());
            }
            provenance::Constraint::Xor { wires, rhs } => {
                solver.add_xor(wires, *rhs);
            }
        }
    }
    Ok(())
}
fn attach_initial(solver: &mut Solver, shared: &Arc<Mutex<CaptureState>>) -> Result<(), Stop> {
    // Actual native snapshot, including native clause offsets/root reasons.
    // Native strings on first observer failure are covered by fixed scratch.
    let output = Arc::clone(shared);
    solver
        .set_event_observer(move |event| {
            let mut state = match output.lock() {
                Ok(s) => s,
                Err(poison) => {
                    let mut s = poison.into_inner();
                    let _: Result<(), Stop> = s.fail(Stop::Observer);
                    return Err("SAT initial observer lock refused".into());
                }
            };
            state
                .on_event(event)
                .map_err(|_| "SAT initial observer refused".to_owned())
        })
        .map_err(|_| Stop::Observer)?;
    Ok(())
}
fn seal_initial(
    map: &provenance::CheckedMap,
    shared: &Arc<Mutex<CaptureState>>,
    checker_limits: provenance::initial::Limits,
) -> Result<(), Stop> {
    let mut state = shared.lock().map_err(|_| Stop::Observer)?;
    state.closed = true;
    state.flush()?;
    let (clauses, xors, roots, contradiction) = {
        let CaptureState { events, budget, .. } = &mut *state;
        let seal =
            provenance::initial::verify_initial(map, events, checker_limits, &mut Admit(budget))
                .map_err(|_| Stop::NativeFailure)?;
        (
            seal.clause_count(),
            seal.xor_count(),
            seal.root_count(),
            seal.insertion_contradiction(),
        )
    };
    let CaptureState {
        budget,
        sink,
        buffer,
        limits,
        binding,
        pattern,
        map_ack,
        events,
        acks,
        seal_ack,
        sealed,
        ..
    } = &mut *state;
    let last = acks.last().ok_or(Stop::Events)?;
    let record = InitialChecked {
        schema: "ml-dataset.sat-initial-checked/v1",
        original_binding: *binding,
        pattern: *pattern,
        map_capture_sequence: map_ack.event_count,
        map_capture_sha256: &map_ack.last_event_sha256,
        native_events: events.len(),
        native_capture_end_sequence: last.position.event_count,
        native_capture_end_sha256: &last.position.last_event_sha256,
        clauses,
        xors,
        roots,
        insertion_contradiction: contradiction,
        search_checked: false,
        unsat_proof_checked: false,
    };
    let position = publish(
        budget,
        sink,
        buffer.as_mut().ok_or(Stop::Phase)?,
        limits.max_event_bytes as usize,
        &record,
    )?;
    let valid = position.valid;
    *seal_ack = Some(position.position);
    *sealed = valid;
    if !valid {
        return Err(Stop::Observer);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    struct Temp(PathBuf);
    impl Drop for Temp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }
    // Real native mathematical fixture, intentionally small and not presented
    // as requested-profile worker activation. No hand-written snapshot is used.
    fn fixture(
        pattern: u32,
        infinity: bool,
    ) -> (
        Temp,
        provenance::CheckedMap,
        Arc<Mutex<CaptureState>>,
        Solver,
    ) {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp = Temp(std::env::temp_dir().join(format!(
            "sat-initial-custody-{}-{nonce}",
            std::process::id()
        )));
        std::fs::create_dir_all(&temp.0).unwrap();
        let binding = capture::AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: "4".repeat(64),
            source_commit: "a".repeat(40),
            binary_sha256: "5".repeat(64),
            cell_index: 7,
            attempt_sha256: "6".repeat(64),
        };
        let mut limits = capture::capture_limits(16).unwrap();
        limits.max_event_bytes = 4096;
        limits.validate().unwrap();
        let mut sink =
            capture::CaptureEmitter::create(&temp.0.join("live"), binding, limits.clone()).unwrap();
        let now = Instant::now();
        let mut budget = Budget::new(
            16384,
            512 << 20,
            16 << 20,
            now + std::time::Duration::from_secs(20),
            now,
        )
        .unwrap();
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let b = F2mElement::from_bit_positions(&[1], 3);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        let original = provenance::Original {
            binding: [7; 32],
            a: &a,
            b: &b,
            irr: &irr,
            twist: true,
            m: 4,
            target: if infinity { None } else { Some(&a) },
            pattern,
            codes: &codes,
        };
        let map = provenance::build_original(
            &original,
            provenance::Limits {
                variables: 100000,
                codes: 64,
                domain_clauses: 100000,
                work: 16_000_000,
                bytes: 64 << 20,
            },
            &mut Admit(&mut budget),
        )
        .unwrap();
        // The test's map publication is setup. The production constructor instead
        // requires the already-charged OriginalOwner::persist_map receipt.
        sink.payload_borrowed(
            "original.json",
            "sat-original-gate-map",
            "json",
            &serde_json::to_vec(map.map()).unwrap(),
        )
        .unwrap();
        let map_ack = sink.committed_event_position();
        let q = quote(&map, &mut budget, &limits).unwrap();
        budget
            .reserve_checking(q.work, q.bytes, Instant::now())
            .unwrap();
        let cap = limits.max_event_bytes;
        let shared = Arc::new(Mutex::new(CaptureState {
            budget,
            sink,
            limits,
            binding: [7; 32],
            pattern,
            map_ack,
            events: Vec::with_capacity(q.events),
            acks: Vec::with_capacity(q.events),
            pending: None,
            buffer: Some(
                capture::TypedEventBuffer::try_new(
                    cap,
                    capture::TypedEventBuffer::requested_bytes(cap).unwrap(),
                )
                .unwrap(),
            ),
            event_limit: q.events,
            chunk_start: 0,
            chunk_bytes: 0,
            chunk_binary: 0,
            codec_state: packed::State::default(),
            chunk_codec_start: packed::State::default(),
            closed: false,
            sealed: false,
            seal_ack: None,
        }));
        let mut solver = Solver::new(map.variables());
        solver.deadline = Some(shared.lock().unwrap().budget.deadline);
        install_formula(&mut solver, &map, &shared).unwrap();
        (temp, map, shared, solver)
    }
    fn checker() -> provenance::initial::Limits {
        provenance::initial::Limits {
            work: 16_000_000,
            bytes: 64 << 20,
            events: 100000,
            constraints: 100000,
        }
    }
    // Insert this test-only helper in sat_initial_native::tests and call immediately
    // after successful persist_map in the unchanged strict requested-profile test.
    // The replay ledger below is independent test replay work, NOT producer credit.
    // Do not change or replace the subsequent NativeInitial::prepare positive gate.
    fn assert_requested_profile_gate_round_trip(owner: &OriginalOwner) {
        #[derive(Default)]
        struct ReplayLedger {
            work: u64,
            bytes: u64,
        }
        impl provenance::Admission for ReplayLedger {
            fn reserve(&mut self, q: provenance::Quote) -> Result<(), provenance::Error> {
                self.work = self
                    .work
                    .checked_add(q.work)
                    .ok_or(provenance::Error::Cap)?;
                self.bytes = self
                    .bytes
                    .checked_add(q.bytes)
                    .ok_or(provenance::Error::Cap)?;
                Ok(())
            }
            fn check(&mut self) -> Result<(), provenance::Error> {
                Ok(())
            }
        }
        let original = provenance::Original {
            binding: owner.binding,
            a: &owner.math.a,
            b: &owner.math.b,
            irr: &owner.math.irr,
            twist: owner.original.config.twist,
            m: owner.original.config.m,
            target: owner.math.target.as_ref(),
            pattern: owner.pattern.unwrap(),
            codes: &owner.math.codes,
        };
        let mut replay = ReplayLedger::default();
        let rebuilt = provenance::gate_wire::decode_checked(
            &owner.retained_bytes,
            &original,
            provenance::gate_wire::DecodeLimits {
                wire_bytes: owner.capture_limits.max_payload_bytes as usize,
                work: u64::MAX,
                bytes: u64::MAX,
            },
            provenance::Limits {
                variables: 100_000,
                codes: owner.math.codes.len(),
                domain_clauses: 1_000_000,
                work: u64::MAX,
                bytes: u64::MAX,
            },
            &mut replay,
        )
        .expect("genuine requested-profile gate wire independent reconstruction");
        let before = owner.checked.as_ref().unwrap();
        assert_eq!(rebuilt.formula(), before.formula());
        assert_eq!(
            serde_json::to_vec(rebuilt.map()).unwrap(),
            serde_json::to_vec(before.map()).unwrap()
        );
        assert!(replay.work > 0 && replay.bytes > 0);
    }
    #[test]
    fn requested_profile_owner_map_native_snapshot_chain_requires_real_success() {
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
            assert_requested_profile_gate_round_trip(&owner);
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
            let state = stage.shared.lock().unwrap();
            assert_eq!(state.budget.ownership_id, ownership);
            assert_eq!(state.budget.deadline, deadline);
            assert!(state.closed && state.sealed);
            assert!(state.pending.is_none());
            assert_eq!(state.chunk_start, state.events.len());
            assert!(!state.events.is_empty());
            assert!(!state.acks.is_empty());
            assert!(state.acks.iter().all(|a| a.valid));
            assert_eq!(state.map_ack, map_position);
            let seal = state
                .seal_ack
                .as_ref()
                .expect("required durable checked-snapshot marker");
            assert_eq!(seal, &state.sink.committed_event_position());
            assert!(seal.event_count > state.acks.last().unwrap().position.event_count);
            assert!(state.acks[0].position.event_count > map_position.event_count);
            assert_eq!(stage.solver.as_ref().unwrap().stats.decisions, 0);
            let (mut batches, mut markers, mut maps) = (0usize, 0usize, 0usize);
            let mut replay_codec = packed::State::default();
            let mut replay_events = Vec::new();
            capture::verify_live_committed_prefix_visit(
                &f.root,
                &f.binding,
                &limits,
                &mut |line| {
                    let row: capture::EventRecord =
                        serde_json::from_slice(line).map_err(|e| e.to_string())?;
                    if row.event["kind"] == "capture_payload_committed"
                        && row.event["artifact"]["role"] == "sat-original-gate-map"
                    {
                        maps += 1;
                    }
                    if row.event["kind"] == "binary_sat_initial" {
                        match row.event["payload"]["schema"].as_str() {
                            Some("ml-dataset.sat-initial-batch/v3") => {
                                use base64::Engine;
                                batches += 1;
                                assert_eq!(
                                    row.event["payload"]["first_native_index"].as_u64(),
                                    Some(replay_events.len() as u64)
                                );
                                let text = row.event["payload"]["events_packed"].as_str().unwrap();
                                let raw = base64::engine::general_purpose::STANDARD
                                    .decode(text)
                                    .unwrap();
                                let before = replay_events.len();
                                // Separate test replay admission; no producer credit.
                                packed::decode_into(
                                    &raw,
                                    &mut replay_codec,
                                    &mut replay_events,
                                    packed::DecodeLimits {
                                        events: 100_000,
                                        slots: 1_000_000,
                                        words: 1_000_000,
                                    },
                                    &mut |_, _| Ok(()),
                                )
                                .unwrap();
                                assert_eq!(
                                    row.event["payload"]["native_event_count"].as_u64(),
                                    Some((replay_events.len() - before) as u64)
                                );
                            }
                            Some("ml-dataset.sat-initial-checked/v1") => {
                                markers += 1;
                                assert_eq!(row.event["payload"]["search_checked"], false);
                                assert_eq!(row.event["payload"]["unsat_proof_checked"], false);
                            }
                            _ => return Err("unexpected typed initial schema".into()),
                        }
                    }
                    Ok(())
                },
            )
            .unwrap()
            .expect("complete authenticated live prefix");
            assert_eq!(replay_events, state.events);
            assert_eq!(maps, 1);
            assert_eq!(batches, state.acks.len());
            assert_eq!(markers, 1);
        }
    }
    #[test]
    fn exact_capture_envelope_and_batch_overheads_fit_admitted_bounds() {
        // Exercise the real capture serializer at its maximum sequence. These
        // are serialization extrema, not claimed valid native solver states.
        let digest = "f".repeat(64);
        let events = vec![
            SolverEvent::Snapshot {
                version: u32::MAX,
                variables: u32::MAX,
                trail_limits: Vec::new(),
                already_unsat: false,
            },
            SolverEvent::Clause {
                clause_id: u32::MAX,
                literals: vec![i32::MIN, i32::MAX],
                existing: false,
                detached: false,
            },
            SolverEvent::XorConstraint {
                row_id: usize::MAX,
                mask: vec![u64::MAX; 3],
                rhs: false,
            },
        ];
        for count in [0, 1, 3] {
            let record = Batch {
                schema: "ml-dataset.sat-initial-batch/v3",
                original_binding: [255; 32],
                pattern: u32::MAX,
                map_capture_sequence: u64::MAX,
                map_capture_sha256: &digest,
                first_native_index: usize::MAX,
                native_event_count: count,
                events_packed: packed::Packed {
                    events: &events[..count],
                    state: packed::State::default(),
                },
            };
            let payload = serde_json::to_vec(&record).unwrap();
            let wire = capture::maximum_sequence_typed_wire(
                capture::TypedEventKind::BinarySatInitial,
                &record,
            );
            assert!(wire.len() >= payload.len());
            assert!(
                wire.len() - payload.len() <= 512,
                "hidden capture envelope exceeded prepaid bound"
            );
            let native_count = serde_json::to_vec(&packed::Packed {
                events: &events[..count],
                state: packed::State::default(),
            })
            .unwrap()
            .len();
            assert!(wire.len() >= native_count);
            assert!(
                wire.len() - native_count <= 1024,
                "batch plus capture envelope exceeded chunk reserve"
            );
        }
        let marker = InitialChecked {
            schema: "ml-dataset.sat-initial-checked/v1",
            original_binding: [255; 32],
            pattern: u32::MAX,
            map_capture_sequence: u64::MAX,
            map_capture_sha256: &digest,
            native_events: usize::MAX,
            native_capture_end_sequence: u64::MAX,
            native_capture_end_sha256: &digest,
            clauses: usize::MAX,
            xors: usize::MAX,
            roots: usize::MAX,
            insertion_contradiction: false,
            search_checked: false,
            unsat_proof_checked: false,
        };
        let payload = serde_json::to_vec(&marker).unwrap();
        let wire = capture::maximum_sequence_typed_wire(
            capture::TypedEventKind::BinarySatInitial,
            &marker,
        );
        assert!(wire.len() >= payload.len());
        assert!(
            wire.len() - payload.len() <= 512,
            "marker envelope exceeded prepaid bound"
        );
    }
    #[test]
    fn real_native_snapshot_is_checked_and_acknowledged_before_search() {
        for pattern in 0..4 {
            for infinity in [false, true] {
                let (_temp, map, shared, mut solver) = fixture(pattern, infinity);
                let owner = shared.lock().unwrap().budget.ownership_id;
                attach_initial(&mut solver, &shared).unwrap();
                seal_initial(&map, &shared, checker()).unwrap();
                let state = shared.lock().unwrap();
                assert_eq!(state.budget.ownership_id, owner);
                assert!(state.closed && state.sealed && state.pending.is_none());
                assert_eq!(state.chunk_start, state.events.len());
                assert!(!state.events.is_empty());
                assert!(state.events.iter().all(|e| !matches!(
                    e,
                    SolverEvent::SolveBegin { .. }
                        | SolverEvent::Decision { .. }
                        | SolverEvent::SolveEnd { .. }
                )));
                let mut next = 0;
                for ack in &state.acks {
                    assert!(ack.valid);
                    assert_eq!(ack.first, next);
                    assert!(ack.last > ack.first);
                    next = ack.last;
                }
                assert_eq!(next, state.events.len());
                assert_eq!(
                    state.seal_ack.as_ref(),
                    Some(&state.sink.committed_event_position())
                );
                assert_eq!(solver.stats.decisions, 0);
                assert!(solver.event_observer_error().is_none());
            }
        }
    }
    #[test]
    fn actual_capture_head_failure_retains_accepted_native_and_encoded_tail() {
        let (temp, map, shared, mut solver) = fixture(0, false);
        attach_initial(&mut solver, &shared).unwrap();
        let (previous, accepted, work, ownership, deadline) = {
            let state = shared.lock().unwrap();
            assert!(state.acks.is_empty());
            assert!(state.pending.is_none());
            assert!(!state.events.is_empty());
            (
                state.sink.committed_event_position(),
                state.events.clone(),
                state.budget.checking_work,
                state.budget.ownership_id,
                state.budget.deadline,
            )
        };
        // The packed fixture fits one batch. Fault its actual mandatory flush,
        // rather than assuming attachment itself crosses a chunk boundary.
        std::fs::remove_file(temp.0.join("live/committed.json")).unwrap();
        std::fs::create_dir(temp.0.join("live/committed.json")).unwrap();
        assert_eq!(seal_initial(&map, &shared, checker()), Err(Stop::Observer));
        let state = shared.lock().unwrap();
        assert_eq!(state.events, accepted);
        assert!(state.pending.is_none());
        assert!(state.closed);
        assert_eq!(state.chunk_start, 0);
        assert!(state.chunk_binary > 0);
        assert!(state.buffer.as_ref().unwrap().encoded_len() > 0);
        assert_eq!(state.budget.ownership_id, ownership);
        assert_eq!(state.budget.deadline, deadline);
        assert!(state.budget.checking_work > work);
        assert_eq!(state.sink.committed_event_position(), previous);
        assert!(state.acks.is_empty());
        assert!(!state.sealed);
        assert!(state.seal_ack.is_none());
        assert!(solver.event_observer_error().is_none());
        assert_eq!(solver.stats.decisions, 0);
    }
    #[test]
    fn prepaid_pending_event_survives_original_deadline_or_work_stop() {
        for deadline in [false, true] {
            let (_temp, _map, shared, mut solver) = fixture(0, false);
            let previous;
            {
                let mut state = shared.lock().unwrap();
                previous = state.sink.committed_event_position();
                if deadline {
                    state.budget.construction_deadline =
                        Instant::now() - std::time::Duration::from_secs(1);
                } else {
                    state.budget.checking_work =
                        state.budget.native_work_limit - state.budget.native.work;
                }
            }
            assert!(attach_initial(&mut solver, &shared).is_err());
            let state = shared.lock().unwrap();
            assert!(matches!(state.pending, Some(SolverEvent::Snapshot { .. })));
            assert_eq!(
                state.budget.stopped,
                Some(if deadline { Stop::Deadline } else { Stop::Work })
            );
            assert_eq!(state.sink.committed_event_position(), previous);
            assert!(state.events.is_empty());
            assert!(state.acks.is_empty());
            assert!(state.seal_ack.is_none());
            assert!(!state.sealed);
            assert_eq!(solver.stats.decisions, 0);
        }
    }
    #[test]
    fn real_native_snapshot_mutation_refuses_marker_and_preserves_owned_prefix() {
        let (_temp, map, shared, mut solver) = fixture(0, false);
        attach_initial(&mut solver, &shared).unwrap();
        {
            let mut state = shared.lock().unwrap();
            let row = state
                .events
                .iter_mut()
                .find(|e| matches!(e, SolverEvent::Clause { .. }))
                .unwrap();
            if let SolverEvent::Clause { clause_id, .. } = row {
                *clause_id = u32::MAX;
            }
        }
        assert!(seal_initial(&map, &shared, checker()).is_err());
        let state = shared.lock().unwrap();
        assert!(!state.sealed);
        assert!(state.seal_ack.is_none());
        assert!(!state.events.is_empty());
        assert_eq!(solver.stats.decisions, 0);
    }
}

impl NativeInitial<'_> {
    /// Move every owned native/evidence buffer. No clone, allocation, new budget,
    /// observer or sink. Fixed original hashes survive the ended borrowed token.
    pub(super) fn detach(self) -> NativeInitial<'static> {
        let Evidence {
            original: _,
            original_hashes,
            binding,
            launch_sha256,
            math,
            checked,
            unchecked,
            pattern,
            map_bytes,
            map_ack,
            publication_attempted,
            map_published,
        } = self.evidence;
        NativeInitial {
            evidence: Evidence {
                original: None,
                original_hashes,
                binding,
                launch_sha256,
                math,
                checked,
                unchecked,
                pattern,
                map_bytes,
                map_ack,
                publication_attempted,
                map_published,
            },
            shared: self.shared,
            solver: self.solver,
        }
    }
}
