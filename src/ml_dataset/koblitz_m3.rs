//! Adapter-side cumulative reservations for the bounded Koblitz m3 pipeline.
//!
//! This module is not yet registered in runtime dispatch. Its constructor must
//! receive limits from a validated original SolverExecutionContext, never from
//! a producer result. Reservations model admitted work/bytes, not RSS or retired
//! instructions. Native work and actual group additions have different units.
use std::time::{Duration, Instant};

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
    native_work_limit: u64,
    memory_limit: u64,
    trace_limit: u64,
    addition_limit: u64,
    deadline: Instant,
    construction_deadline: Instant,
    phase: Phase,
    native: Reservation,
    phase_native: Reservation,
    phase_limits: NativeLimits,
    adapter_bytes: u64,
    trace_bytes: u64,
    additions: u64,
    stopped: Option<Stop>,
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
        Ok(Self {
            native_work_limit: work.min(NATIVE_WORK_MAX),
            memory_limit: memory_bytes,
            trace_limit: trace_bytes,
            addition_limit: configured_work,
            deadline: original_deadline,
            construction_deadline,
            phase: Phase::Setup,
            native: Reservation::default(),
            phase_native: Reservation::default(),
            phase_limits: NativeLimits { work: 0, bytes: 0 },
            adapter_bytes: 0,
            trace_bytes: 0,
            additions: 0,
            stopped: None,
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
            work: self.native_work_limit - self.native.work,
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
        if current.work > self.phase_limits.work || dw > self.native_work_limit - self.native.work {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn budget(work: u64, bytes: u64) -> (Budget, Instant) {
        let now = Instant::now();
        (
            Budget::new(work, bytes, 100, now + Duration::from_secs(20), now).unwrap(),
            now,
        )
    }
    fn constructed(b: &mut Budget, now: Instant, r: Reservation) {
        b.begin_native(false, now).unwrap();
        b.observe(r, now).unwrap();
        b.finish_native(now).unwrap();
    }

    #[test]
    fn native_phases_share_work_and_retained_memory() {
        let (mut b, now) = budget(10, 1000);
        b.reserve_bytes(50, now).unwrap();
        constructed(
            &mut b,
            now,
            Reservation {
                work: 3000,
                bytes: 250,
            },
        );
        assert_eq!(
            b.begin_native(true, now),
            Ok(NativeLimits {
                work: 7240,
                bytes: 700
            })
        );
        b.observe(
            Reservation {
                work: 7240,
                bytes: 700,
            },
            now,
        )
        .unwrap();
        assert_eq!(
            b.native,
            Reservation {
                work: 10240,
                bytes: 950
            }
        );
        assert_eq!(b.reserve_bytes(1, now), Err(Stop::Bytes));
    }

    #[test]
    fn repeated_cumulative_event_is_not_double_charged() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now).unwrap();
        let r = Reservation {
            work: 100,
            bytes: 100,
        };
        b.observe(r, now).unwrap();
        b.observe(r, now).unwrap();
        assert_eq!(b.native, r);
        assert_eq!(
            b.observe(
                Reservation {
                    work: 99,
                    bytes: 100
                },
                now
            ),
            Err(Stop::Nonmonotone)
        );
        assert_eq!(b.finish_native(now), Err(Stop::Nonmonotone));
    }

    #[test]
    fn callback_memory_can_reduce_native_headroom() {
        let (mut b, now) = budget(10, 1000);
        constructed(
            &mut b,
            now,
            Reservation {
                work: 1,
                bytes: 200,
            },
        );
        b.begin_native(true, now).unwrap();
        b.reserve_bytes(750, now).unwrap();
        b.observe(Reservation { work: 1, bytes: 50 }, now).unwrap();
        assert_eq!(
            b.observe(Reservation { work: 1, bytes: 51 }, now),
            Err(Stop::Bytes)
        );
        assert_eq!(b.native.bytes, 250);
    }

    #[test]
    fn construction_deadline_includes_setup_and_never_restarts() {
        let (mut b, now) = budget(10, 1000);
        b.begin_native(false, now + Duration::from_secs(4)).unwrap();
        assert_eq!(
            b.observe(Reservation::default(), now + Duration::from_secs(5)),
            Err(Stop::Deadline)
        );
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Deadline));
    }

    #[test]
    fn matrix_uses_original_deadline() {
        let (mut b, now) = budget(10, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now + Duration::from_secs(6)).unwrap();
        assert_eq!(
            b.finish_native(now + Duration::from_secs(20)),
            Err(Stop::Deadline)
        );
    }

    #[test]
    fn exact_addition_limit_can_finish_but_extra_call_fails() {
        for extra in [false, true] {
            let (mut b, now) = budget(2, 1000);
            constructed(&mut b, now, Reservation::default());
            b.begin_extraction(false, now).unwrap();
            b.reserve_addition(now).unwrap();
            b.reserve_addition(now).unwrap();
            if extra {
                assert_eq!(b.reserve_addition(now), Err(Stop::Additions));
                assert_eq!(b.finish_extraction(now), Err(Stop::Additions));
            } else {
                b.finish_extraction(now).unwrap();
            }
        }
    }

    #[test]
    fn matrix_cannot_be_skipped_when_required_or_run_twice() {
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        assert_eq!(b.begin_extraction(true, now), Err(Stop::Phase));
        let (mut b, now) = budget(2, 1000);
        constructed(&mut b, now, Reservation::default());
        b.begin_native(true, now).unwrap();
        b.finish_native(now).unwrap();
        assert_eq!(b.begin_native(true, now), Err(Stop::Phase));
    }

    #[test]
    fn overflow_zero_trace_limit_and_observer_failure_are_explicit() {
        let (mut b, now) = budget(2, 1000);
        assert!(matches!(
            Budget::new(u64::MAX, 1000, 100, now + Duration::from_secs(1), now),
            Err(Stop::Overflow)
        ));
        assert!(matches!(
            Budget::new(1, 1000, 0, now + Duration::from_secs(1), now),
            Err(Stop::InvalidLimits)
        ));
        b.reserve_trace(100, now).unwrap();
        assert_eq!(b.reserve_trace(1, now), Err(Stop::Trace));
        assert_eq!(b.observer_failed(), Err(Stop::Trace));
        let (mut b, now) = budget(2, 1000);
        assert_eq!(b.observer_failed(), Err(Stop::Observer));
        assert_eq!(b.begin_native(false, now), Err(Stop::Observer));
    }

    #[test]
    fn native_one_over_work_or_builder_bytes_is_rejected_before_mutation() {
        for (r, stop) in [
            (
                Reservation {
                    work: 1025,
                    bytes: 0,
                },
                Stop::Work,
            ),
            (
                Reservation {
                    work: 0,
                    bytes: 251,
                },
                Stop::Bytes,
            ),
        ] {
            let (mut b, now) = budget(1, 1000);
            b.begin_native(false, now).unwrap();
            assert_eq!(b.observe(r, now), Err(stop));
            assert_eq!(b.native, Reservation::default());
        }
    }
}
