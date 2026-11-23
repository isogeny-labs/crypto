use std::time::{Duration,Instant};
const NATIVE_WORK_MAX: u64 = 16_777_216;
const NATIVE_BYTES_MAX: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Stop {
    InvalidLimits,
    Overflow,
    Deadline,
    Phase,
    Nonmonotone,
    Work,
    Bytes,
    Trace,
    Additions,
    Observer,
    NativeFailure,
    Allocation,
    Serialization,
    Events,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
    Setup,
    Construction,
    Constructed,
    Matrix,
    MatrixComplete,
    Extraction,
    Complete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Reservation {
    pub work: u64,
    pub bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct NativeLimits {
    pub work: u64,
    pub bytes: u64,
}

/// One ledger per original attempt. No phase refunds cumulative reservations.
#[derive(Debug)]
pub(super) struct Budget {
    // Process-local ownership token, never a scientific measured counter.
    ownership_id: u64,
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
    deadline: Instant,
    construction_deadline: Instant,
    phase: Phase,
    native: Reservation,
    checking_work: u64,
    checking_performed_work: u64,
    phase_native: Reservation,
    phase_limits: NativeLimits,
    adapter_bytes: u64,
    trace_bytes: u64,
    additions: u64,
    stopped: Option<Stop>,
    retention_started: bool,
}

impl Budget {
    pub fn new(
        configured_work: u64,
        memory_bytes: u64,
        trace_bytes: u64,
        original_deadline: Instant,
        entry: Instant,
    ) -> Result<Self, Stop> {
        if configured_work == 0 || memory_bytes == 0 || trace_bytes == 0 {
            return Err(Stop::InvalidLimits);
        }
        let work = configured_work.checked_mul(1024).ok_or(Stop::Overflow)?;
        if entry >= original_deadline {
            return Err(Stop::Deadline);
        }
        let construction_deadline = entry
            .checked_add(Duration::from_secs(5))
            .ok_or(Stop::Overflow)?
            .min(original_deadline);
        static NEXT_OWNERSHIP: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let ownership_id = NEXT_OWNERSHIP
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |id| id.checked_add(1),
            )
            .map_err(|_| Stop::Overflow)?;
        Ok(Self {
            ownership_id,
            native_work_limit: work.min(NATIVE_WORK_MAX),
            memory_limit: memory_bytes,
            trace_limit: trace_bytes,
            addition_limit: configured_work,
            deadline: original_deadline,
            construction_deadline,
            phase: Phase::Setup,
            native: Reservation::default(),
            checking_work: 0,
            checking_performed_work: 0,
            phase_native: Reservation::default(),
            phase_limits: NativeLimits { work: 0, bytes: 0 },
            adapter_bytes: 0,
            trace_bytes: 0,
            additions: 0,
            stopped: None,
            retention_started: false,
        })
    }

    fn fail<T>(&mut self, stop: Stop) -> Result<T, Stop> {
        let first = *self.stopped.get_or_insert(stop);
        Err(first)
    }

    fn check(&mut self, now: Instant) -> Result<(), Stop> {
        if let Some(stop) = self.stopped {
            return Err(stop);
        }
        let deadline = if matches!(self.phase, Phase::Setup | Phase::Construction) {
            self.construction_deadline
        } else {
            self.deadline
        };
        if now >= deadline {
            return self.fail(Stop::Deadline);
        }
        if self.phase == Phase::Complete {
            return self.fail(Stop::Phase);
        }
        Ok(())
    }

    /// Reserve adapter-owned allocations before allocating, including callback
    /// storage and serialized payload capacity. Native reservations are separate.
    pub fn reserve_bytes(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.adapter_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.memory_limit - self.native.bytes {
            return self.fail(Stop::Bytes);
        }
        self.adapter_bytes = next;
        Ok(())
    }

    pub fn reserve_trace(&mut self, bytes: u64, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let Some(next) = self.trace_bytes.checked_add(bytes) else {
            return self.fail(Stop::Overflow);
        };
        if next > self.trace_limit {
            return self.fail(Stop::Trace);
        }
        self.trace_bytes = next;
        Ok(())
    }

    /// The returned allowance is a phase ceiling. Call observe() at every native
    /// boundary; callback allocations can reduce remaining shared headroom.
    pub fn begin_native(&mut self, matrix: bool, now: Instant) -> Result<NativeLimits, Stop> {
        self.check(now)?;
        let expected = if matrix {
            Phase::Constructed
        } else {
            Phase::Setup
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        let remaining_bytes = self.memory_limit - self.adapter_bytes - self.native.bytes;
        let bytes = if matrix {
            remaining_bytes.min(NATIVE_BYTES_MAX)
        } else {
            remaining_bytes
                .min(self.memory_limit / 4)
                .min(NATIVE_BYTES_MAX)
        };
        let limits = NativeLimits {
            work: self.native_work_limit - self.native.work - self.checking_work,
            bytes,
        };
        self.phase = if matrix {
            Phase::Matrix
        } else {
            Phase::Construction
        };
        self.phase_native = Reservation::default();
        self.phase_limits = limits;
        Ok(limits)
    }

    /// Native APIs report cumulative phase reservations. Convert them to deltas
    /// exactly once and reject regression instead of silently refunding work.
    pub fn observe(&mut self, current: Reservation, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if !matches!(self.phase, Phase::Construction | Phase::Matrix) {
            return self.fail(Stop::Phase);
        }
        if current.work < self.phase_native.work || current.bytes < self.phase_native.bytes {
            return self.fail(Stop::Nonmonotone);
        }
        let dw = current.work - self.phase_native.work;
        let db = current.bytes - self.phase_native.bytes;
        if current.work > self.phase_limits.work
            || dw > self.native_work_limit - self.native.work - self.checking_work
        {
            return self.fail(Stop::Work);
        }
        if current.bytes > self.phase_limits.bytes
            || db > self.memory_limit - self.native.bytes - self.adapter_bytes
        {
            return self.fail(Stop::Bytes);
        }
        self.native.work += dw;
        self.native.bytes += db;
        self.phase_native = current;
        Ok(())
    }

    /// Successful phase completion is supplied only after native terminal status
    /// validation. A stopped native report must instead stop this ledger.
    pub fn finish_native(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        self.phase = match self.phase {
            Phase::Construction => Phase::Constructed,
            Phase::Matrix => Phase::MatrixComplete,
            _ => return self.fail(Stop::Phase),
        };
        Ok(())
    }

    pub fn begin_extraction(&mut self, matrix_required: bool, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        let expected = if matrix_required {
            Phase::MatrixComplete
        } else {
            Phase::Constructed
        };
        if self.phase != expected {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Extraction;
        Ok(())
    }

    /// Call immediately before each actual group.add, including identity starts.
    pub fn reserve_addition(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        if self.additions == self.addition_limit {
            return self.fail(Stop::Additions);
        }
        self.additions += 1;
        Ok(())
    }

    /// Admit an actual signed-candidate group addition during bounded matrix
    /// search. This shares the original addition count and deadline, and never
    /// resets native phase counters or begins final extraction.
    pub fn reserve_matrix_candidate_addition(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Matrix {
            return self.fail(Stop::Phase);
        }
        if self.additions == self.addition_limit {
            return self.fail(Stop::Additions);
        }
        self.additions += 1;
        Ok(())
    }

    /// The caller must separately establish the exact final cursor. Consuming
    /// exactly the allowance does not itself prevent successful completion.
    pub fn finish_extraction(&mut self, now: Instant) -> Result<(), Stop> {
        self.check(now)?;
        if self.phase != Phase::Extraction {
            return self.fail(Stop::Phase);
        }
        self.phase = Phase::Complete;
        Ok(())
    }

    pub fn observer_failed(&mut self) -> Result<(), Stop> {
        self.fail(Stop::Observer)
    }
}

