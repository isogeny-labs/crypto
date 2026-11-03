//! Mandatory producer-ledger boundary after mathematical initial marking.
use super::*;
#[path = "sat_producer_ledger.rs"]
pub(super) mod ledger;
pub(in crate::ml_dataset::solvers) struct WithLedger<'a> {
    stage: NativeInitial<'a>,
    pending: Option<ledger::Checkpoint>,
    ack: Option<capture::CommittedEventPosition>,
}
pub(in crate::ml_dataset::solvers) struct Failed<'a> {
    pub owner: WithLedger<'a>,
    pub stop: Stop,
}
/// Consumes the actual native owner. No new Budget, deadline or sink is created.
pub(in crate::ml_dataset::solvers) fn finish<'a>(
    stage: NativeInitial<'a>,
) -> Result<WithLedger<'a>, Failed<'a>> {
    let mut owner = WithLedger {
        stage,
        pending: None,
        ack: None,
    };
    match owner.finish_inner() {
        Ok(()) => Ok(owner),
        Err(stop) => {
            let stop = {
                let mut state = owner.stage.shared.lock().unwrap_or_else(|e| e.into_inner());
                state.budget.fail::<()>(stop).unwrap_err()
            };
            Err(Failed { owner, stop })
        }
    }
}
impl WithLedger<'_> {
    fn finish_inner(&mut self) -> Result<(), Stop> {
        let mut state = self.stage.shared.lock().map_err(|_| Stop::Observer)?;
        if !state.sealed
            || !state.closed
            || state.pending.is_some()
            || state.chunk_start != state.events.len()
            || self.pending.is_some()
        {
            return Err(Stop::Phase);
        }
        // All String copies, snapshot construction and joins are paid first.
        state.budget.reserve_checking(8192, 2048, Instant::now())?;
        let before = state.sink.committed_event_position();
        if state.seal_ack.as_ref() != Some(&before) {
            return Err(Stop::Observer);
        }
        // Bounded maximum wire includes self-publication, before reading totals.
        // Hidden count, encoding, capture passes retain their original tariffs.
        let cap = ledger::MAX_WIRE_BYTES;
        if cap > state.limits.max_event_bytes {
            return Err(Stop::Bytes);
        }
        state.budget.reserve_checking(
            cap.checked_mul(56)
                .and_then(|n| n.checked_add(65536))
                .ok_or(Stop::Overflow)?,
            0,
            Instant::now(),
        )?;
        state.budget.reserve_trace(
            cap.checked_mul(4)
                .and_then(|n| n.checked_add(4096))
                .ok_or(Stop::Overflow)?,
            Instant::now(),
        )?;
        let b = &state.budget;
        let phase = match b.phase {
            super::super::super::Phase::Setup => ledger::Phase::Setup,
            super::super::super::Phase::Construction => ledger::Phase::Construction,
            super::super::super::Phase::Constructed => ledger::Phase::Constructed,
            super::super::super::Phase::Matrix => ledger::Phase::Matrix,
            super::super::super::Phase::MatrixComplete => ledger::Phase::MatrixComplete,
            super::super::super::Phase::Extraction => ledger::Phase::Extraction,
            super::super::super::Phase::Complete => ledger::Phase::Complete,
        };
        // A successful initial checkpoint cannot conceal an earlier stop.
        if b.stopped.is_some() {
            return Err(b.stopped.unwrap());
        }
        self.pending = Some(ledger::Checkpoint {
            schema: ledger::SCHEMA.into(),
            original_binding: state.binding,
            pattern: state.pattern,
            checkpoint_index: 0,
            previous_checkpoint_sequence: 0,
            previous_checkpoint_sha256: "0".repeat(64),
            covered_sequence: before.event_count,
            covered_sha256: before.last_event_sha256,
            launch_sha256: hex::encode(self.stage.evidence.launch_sha256),
            owner_ordinal: 0,
            limits: [
                b.native_work_limit,
                b.memory_limit,
                b.trace_limit,
                b.addition_limit,
            ],
            totals: [
                b.native.work,
                b.native.bytes,
                b.checking_work,
                b.checking_performed_work,
                b.adapter_bytes,
                b.trace_bytes,
                b.additions,
            ],
            phase,
            phase_native: [b.phase_native.work, b.phase_native.bytes],
            phase_limits: [b.phase_limits.work, b.phase_limits.bytes],
            first_stop: None,
        });
        let CaptureState { sink, buffer, .. } = &mut *state;
        let record = self.pending.as_ref().unwrap();
        sink.event_typed_admit_encoding_with_limit(
            capture::TypedEventKind::BinarySatInitial,
            record,
            buffer.as_mut().ok_or(Stop::Phase)?,
            cap as usize,
            &mut |n| {
                if n as u64 <= cap {
                    Ok(())
                } else {
                    Err(capture::CaptureError::Integrity("ledger wire cap".into()))
                }
            },
            &mut |bytes| {
                if bytes.len() as u64 <= cap {
                    Ok(())
                } else {
                    Err(capture::CaptureError::Integrity("ledger wire cap".into()))
                }
            },
        )
        .map_err(|_| Stop::Observer)?;
        let actual = sink.committed_event_position();
        let valid = actual.event_count
            == record
                .covered_sequence
                .checked_add(1)
                .ok_or(Stop::Overflow)?
            && actual.binding_sha256 == before.binding_sha256
            && actual.byte_end > before.byte_end;
        self.ack = Some(actual); // retain actual ack before post-commit checks
        if !valid {
            return Err(Stop::Observer);
        }
        state.budget.check(Instant::now())?;
        Ok(())
    }
}

impl WithLedger<'_> {
    pub(in crate::ml_dataset::solvers) fn acknowledged_receipt(
        &self,
    ) -> Option<&capture::CommittedEventPosition> {
        self.ack.as_ref()
    }
    pub(in crate::ml_dataset::solvers) fn detach(self) -> WithLedger<'static> {
        WithLedger {
            stage: self.stage.detach(),
            pending: self.pending,
            ack: self.ack,
        }
    }
}
