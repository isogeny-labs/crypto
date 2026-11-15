//! Opt-in exclusive phase clocks for the bounded generic IC worker.
//!
//! A session belongs to its controlling thread. A measured caller must keep
//! phase-changing work on that thread (a collector must use a serial outer loop)
//! and separately enforce its thread/CPU budget. Nested algebra may run under
//! the owning PDP interval. These labels deliberately differ from archived
//! producer schemas; a legacy interval parser must not silently admit them.
use serde::Serialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicU8, Ordering};
#[cfg(not(all(target_os = "linux", target_pointer_width = "64")))]
use std::time::Instant;

/// The monotonic clock in nanoseconds: `CLOCK_MONOTONIC`, the clock
/// `std::time::Instant` reads on Linux, kept as a plain count.  A boundary
/// runs once per relation (millions in a 32-bit child), and `Instant`'s
/// checked difference, `Duration` normalisation and 128-bit nanosecond
/// conversion cost more instructions than the clock read itself; the
/// interval between two reads is the same number either way.
///
/// The C function is declared here rather than taken from the `libc` crate,
/// because research crates compile this file by `#[path]` and do not all
/// depend on `libc`.  Elsewhere the clock is `Instant` from a fixed anchor.
#[inline]
fn now_ns() -> u64 {
    #[cfg(all(target_os = "linux", target_pointer_width = "64"))]
    {
        /// `struct timespec` on 64-bit Linux.
        #[repr(C)]
        struct Timespec {
            tv_sec: i64,
            tv_nsec: i64,
        }
        extern "C" {
            fn clock_gettime(clock: i32, tp: *mut Timespec) -> i32;
        }
        /// `CLOCK_MONOTONIC` in `<linux/time.h>`.
        const CLOCK_MONOTONIC: i32 = 1;
        let mut ts = Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: `ts` is a valid, writable `struct timespec` of the
        // 64-bit Linux ABI, and CLOCK_MONOTONIC always exists there.
        let rc = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut ts) };
        debug_assert_eq!(rc, 0);
        (ts.tv_sec as u64) * 1_000_000_000 + ts.tv_nsec as u64
    }
    #[cfg(not(all(target_os = "linux", target_pointer_width = "64")))]
    {
        static ANCHOR: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
        u64::try_from(ANCHOR.get_or_init(Instant::now).elapsed().as_nanos()).expect("bounded clock")
    }
}

/// The hot-path clock: raw ticks of the x86-64 time-stamp counter when the
/// CPU advertises an invariant TSC, otherwise [`now_ns`] itself.
///
/// A boundary runs twice per relation trial (millions of times in a 32-bit
/// child), and even the vDSO `clock_gettime` path (call, seqlock, TSC read,
/// scale and shift) cost more than the rest of a boundary; an ordered TSC
/// read (`rdtscp`, or `lfence; rdtsc`, as the vDSO reads it) is one or two
/// instructions.  Ticks are converted to nanoseconds once, when the session
/// finishes, at the session's own average TSC rate measured against
/// `CLOCK_MONOTONIC` between [`Session::begin`] and [`Session::finish`]
/// ([`TickScale`]).
///
/// **Precision.**  Each phase total is its tick count times that average
/// rate, so it inherits the rate's relative error: the two calibration
/// reads (TSC and `CLOCK_MONOTONIC` side by side) are each a few tens of
/// nanoseconds apart, which over a session of milliseconds or more is a
/// part-per-million-scale error, well under scheduling noise.  The phase
/// totals still sum exactly to `observed_wall_ns` (and the online ones to
/// `online_wall_ns`): the conversion is applied to running sums, so each
/// phase may move by at most one nanosecond against a direct conversion.
/// Without an invariant TSC, or off x86-64, ticks are nanoseconds and
/// nothing changes from the `clock_gettime` path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Clock {
    Monotonic,
    /// `lfence; rdtsc`.
    #[cfg(target_arch = "x86_64")]
    Tsc,
    /// `rdtscp`, when the CPU has it.
    #[cfg(target_arch = "x86_64")]
    Tscp,
}

impl Clock {
    /// Chosen once per session.
    fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            // CPUID 0x8000_0007 EDX bit 8: invariant TSC (constant rate
            // across P-, C- and T-states, the `nonstop_tsc` flag on Linux).
            // `cpuid` exists on every x86-64 CPU (a safe intrinsic).
            let max_ext = std::arch::x86_64::__cpuid(0x8000_0000).eax;
            if max_ext >= 0x8000_0007 && std::arch::x86_64::__cpuid(0x8000_0007).edx & (1 << 8) != 0
            {
                // CPUID 0x8000_0001 EDX bit 27: `rdtscp`.
                return if std::arch::x86_64::__cpuid(0x8000_0001).edx & (1 << 27) != 0 {
                    Self::Tscp
                } else {
                    Self::Tsc
                };
            }
        }
        Self::Monotonic
    }

    #[inline(always)]
    fn ticks(self) -> u64 {
        match self {
            Self::Monotonic => now_ns(),
            // An *ordered* TSC read, as the kernel's `rdtsc_ordered` (what
            // the vDSO `clock_gettime` reads too): a bare `rdtsc` may execute
            // before earlier instructions finish, which charges work still
            // in flight at a boundary (a division chain, a cache miss) to
            // the next phase.  Measured on the 32-bit `walk` child, a bare
            // `rdtsc` moved about a sixth of the online wall from
            // `target_query` to `target_pdp`; `rdtscp` (which waits for
            // every earlier instruction) and `lfence; rdtsc` keep the split
            // that `clock_gettime` gave.
            // SAFETY: `lfence`, `rdtsc` and (when CPUID reports it)
            // `rdtscp` are unprivileged on Linux x86-64; SSE2 is part of
            // the x86-64 baseline.
            #[cfg(target_arch = "x86_64")]
            Self::Tsc => unsafe {
                core::arch::x86_64::_mm_lfence();
                core::arch::x86_64::_rdtsc()
            },
            #[cfg(target_arch = "x86_64")]
            Self::Tscp => unsafe {
                let mut aux = 0u32;
                core::arch::x86_64::__rdtscp(&mut aux)
            },
        }
    }
}

/// The tick → nanosecond map of one session: `ns = ticks · ns_span /
/// tick_span`, both spans measured between the session's first and last
/// clock reads.  The identity for [`Clock::Monotonic`].
#[derive(Clone, Copy)]
struct TickScale {
    ns_span: u64,
    tick_span: u64,
}

impl TickScale {
    #[inline]
    fn ns(self, ticks: u64) -> u64 {
        if self.tick_span == 0 || self.tick_span == self.ns_span {
            return ticks;
        }
        (u128::from(ticks) * u128::from(self.ns_span) / u128::from(self.tick_span)) as u64
    }

    /// Per-entry nanoseconds of `ticks`, converted through running sums so
    /// they add up to `self.ns(ticks.sum())` exactly.
    fn split(self, ticks: impl Iterator<Item = u64>) -> Vec<u64> {
        let (mut cum_ticks, mut cum_ns) = (0u64, 0u64);
        ticks
            .map(|t| {
                cum_ticks += t;
                let next = self.ns(cum_ticks);
                let out = next - cum_ns;
                cum_ns = next;
                out
            })
            .collect()
    }
}

/// Whether phase boundaries issue Callgrind `DUMP_STATS_AT` requests:
/// only under Valgrind (the `RUNNING_ON_VALGRIND` client request, which
/// natively returns 0), and not when `IC_MEASUREMENT_PHASE_DUMPS=0`.
/// Decided once per session.  Natively a boundary therefore issues no
/// client request; under Callgrind the per-phase parts that
/// `icprog callgrind` sums are still written, and setting the variable to
/// `0` profiles a whole solve as one part without patching the binary.
fn phase_dumps_enabled() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        let args = [0x1001usize, 0, 0, 0, 0, 0];
        let running: usize;
        // SAFETY: the Valgrind client-request preamble; natively the
        // rotations cancel and `xchg rbx, rbx` is a no-op, so `rdx` keeps
        // its default 0.
        unsafe {
            std::arch::asm!(
                "rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51",
                "xchg rbx, rbx", in("rax") args.as_ptr(), inout("rdx") 0usize => running,
                out("rdi") _, options(nostack)
            );
        }
        running != 0 && std::env::var_os("IC_MEASUREMENT_PHASE_DUMPS").is_none_or(|v| v != "0")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Phase {
    Setup,
    FactorBase,
    Precompute,
    Queries,
    Pdp,
    RelationCheck,
    MatrixBuild,
    RelationLa,
    TargetQuery,
    TargetPdp,
    TargetRelationCheck,
    TargetDescent,
    RecoveryCheck,
    RhoSolve,
}

const PHASES: [Phase; 14] = [
    Phase::Setup,
    Phase::FactorBase,
    Phase::Precompute,
    Phase::Queries,
    Phase::Pdp,
    Phase::RelationCheck,
    Phase::MatrixBuild,
    Phase::RelationLa,
    Phase::TargetQuery,
    Phase::TargetPdp,
    Phase::TargetRelationCheck,
    Phase::TargetDescent,
    Phase::RecoveryCheck,
    Phase::RhoSolve,
];

impl Phase {
    fn label(self) -> &'static [u8] {
        match self {
            Self::Setup => b"generic_ic_setup\0",
            Self::FactorBase => b"generic_ic_factor_base\0",
            Self::Precompute => b"generic_ic_precompute\0",
            Self::Queries => b"generic_ic_queries\0",
            Self::Pdp => b"generic_ic_pdp\0",
            Self::RelationCheck => b"generic_ic_relation_check\0",
            Self::MatrixBuild => b"generic_ic_matrix_build\0",
            Self::RelationLa => b"generic_ic_relation_la\0",
            Self::TargetQuery => b"generic_ic_target_query\0",
            Self::TargetPdp => b"generic_ic_target_pdp\0",
            Self::TargetRelationCheck => b"generic_ic_target_relation_check\0",
            Self::TargetDescent => b"generic_ic_target_descent\0",
            Self::RecoveryCheck => b"generic_ic_recovery_check\0",
            Self::RhoSolve => b"generic_ic_rho_solve\0",
        }
    }

    fn name(self) -> &'static str {
        let label = self.label();
        std::str::from_utf8(&label[11..label.len() - 1]).expect("static ASCII phase")
    }

    fn online(self) -> bool {
        matches!(
            self,
            Self::TargetQuery
                | Self::TargetPdp
                | Self::TargetRelationCheck
                | Self::TargetDescent
                | Self::RecoveryCheck
                | Self::RhoSolve
        )
    }
}

struct Trace {
    generation: u64,
    phase: Phase,
    clock: Clock,
    /// Issue a Callgrind dump at each boundary ([`phase_dumps_enabled`]).
    dumps: bool,
    /// [`now_ns`] when the session began, the calibration anchor.
    started_ns: u64,
    /// Clock ticks ([`Clock::ticks`]) at the session's start and at the
    /// last boundary; every interval below is in ticks until `finish`.
    started: u64,
    last: u64,
    elapsed: [u64; 14],
    entered: [bool; 14],
    online_elapsed: [u64; 14],
    online_entered: [bool; 14],
    online_start: Option<u64>,
    online_ns: Option<u64>,
    error: Option<&'static str>,
}

impl Trace {
    fn boundary(&mut self, next: Phase) {
        if self.dumps {
            dump(self.phase.label());
        }
        // Monotonic clocks; the TSC is invariant and synchronised across
        // cores, and the clamp only guards a migrating read, keeping `last`
        // non-decreasing so the intervals still sum to `last − started`.
        let now = self.clock.ticks().max(self.last);
        let ns = now - self.last;
        self.elapsed[self.phase as usize] += ns;
        if self.online_start.is_some() {
            self.online_elapsed[self.phase as usize] += ns;
        }
        self.last = now;
        self.phase = next;
        self.entered[next as usize] = true;
        if self.online_start.is_some() && next.online() {
            self.online_entered[next as usize] = true;
        }
    }
}

#[derive(Default)]
struct State {
    generation: u64,
    trace: Option<Trace>,
}

thread_local! {
    static CURRENT: RefCell<State> = const {
        RefCell::new(State {
            generation: 0,
            trace: None,
        })
    };
}

// A worker's strict session must notice an accidentally uninstrumented Rayon
// callback. Ordinary library tests can still use isolated thread-local clocks.
// 0 = inactive, 1 = active, 2 = active with a foreign boundary. One atomic
// prevents a foreign marker being lost between session activation and reset.
static STRICT_STATE: AtomicU8 = AtomicU8::new(0);

fn foreign_boundary() {
    if STRICT_STATE.load(Ordering::Relaxed) == 1 {
        let _ = STRICT_STATE.compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst);
    }
}

/// Raw observed intervals. The external runner must add launch, input before
/// session entry and the report/exit tail to setup; this is not a cold total.
#[derive(Debug, Serialize)]
pub struct Snapshot {
    pub schema_version: u8,
    /// An unentered phase stays unknown; the caller must explicitly justify
    /// absent/fused stages before an admission adapter can assign zero cost.
    pub phases_ns: BTreeMap<&'static str, Option<u64>>,
    pub observed_wall_ns: u64,
    pub online_phases_ns: BTreeMap<&'static str, Option<u64>>,
    pub online_wall_ns: Option<u64>,
}

/// A thread-bound guard. Errors and unwinding disable clocks automatically.
#[must_use]
pub struct Session {
    generation: u64,
    strict: bool,
    _owner: PhantomData<Rc<()>>,
}

impl Session {
    pub fn begin() -> Result<Self, &'static str> {
        CURRENT.with(|slot| {
            let mut state = slot.borrow_mut();
            if state.trace.is_some() {
                return Err("measurement session already active");
            }
            if STRICT_STATE.load(Ordering::SeqCst) != 0 {
                return Err("strict measurement session already active");
            }
            state.generation = state
                .generation
                .checked_add(1)
                .expect("bounded session count");
            let generation = state.generation;
            let clock = Clock::detect();
            let dumps = phase_dumps_enabled();
            let started_ns = now_ns();
            let now = clock.ticks();
            let mut entered = [false; 14];
            entered[Phase::Setup as usize] = true;
            state.trace = Some(Trace {
                generation,
                phase: Phase::Setup,
                clock,
                dumps,
                started_ns,
                started: now,
                last: now,
                elapsed: [0; 14],
                entered,
                online_elapsed: [0; 14],
                online_entered: [false; 14],
                online_start: None,
                online_ns: None,
                error: None,
            });
            Ok(Self {
                generation,
                strict: false,
                _owner: PhantomData,
            })
        })
    }

    /// Fail closed if any phase-changing callback runs off the owner thread.
    /// Only one strict session is permitted in a process.
    pub fn begin_strict() -> Result<Self, &'static str> {
        let mut session = Self::begin()?;
        STRICT_STATE
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .map_err(|_| "strict measurement session already active")?;
        session.strict = true;
        Ok(session)
    }

    pub fn finish(self) -> Result<Snapshot, &'static str> {
        if self.strict && STRICT_STATE.load(Ordering::SeqCst) == 2 {
            return Err("phase boundary occurred off the controlling thread");
        }
        CURRENT.with(|slot| {
            let mut state = slot.borrow_mut();
            let trace = state.trace.as_mut().ok_or("measurement session missing")?;
            if trace.generation != self.generation {
                return Err("measurement session changed");
            }
            if trace.online_start.is_some() {
                return Err("online interval not closed");
            }
            if let Some(error) = trace.error {
                return Err(error);
            }
            trace.boundary(Phase::Setup);
            let scale = TickScale {
                ns_span: now_ns().saturating_sub(trace.started_ns),
                tick_span: trace.last - trace.started,
            };
            let scale = if trace.clock == Clock::Monotonic {
                TickScale {
                    ns_span: 1,
                    tick_span: 1,
                }
            } else {
                scale
            };
            let phases_ns = scale.split(PHASES.iter().map(|&p| trace.elapsed[p as usize]));
            let online: Vec<Phase> = PHASES.iter().copied().filter(|p| p.online()).collect();
            let online_ns = scale.split(online.iter().map(|&p| trace.online_elapsed[p as usize]));
            let snapshot = Snapshot {
                schema_version: 1,
                phases_ns: PHASES
                    .iter()
                    .zip(&phases_ns)
                    .map(|(&p, &ns)| (p.name(), trace.entered[p as usize].then_some(ns)))
                    .collect(),
                observed_wall_ns: scale.ns(trace.last - trace.started),
                online_phases_ns: online
                    .iter()
                    .zip(&online_ns)
                    .map(|(&p, &ns)| (p.name(), trace.online_entered[p as usize].then_some(ns)))
                    .collect(),
                online_wall_ns: trace.online_ns.map(|ticks| scale.ns(ticks)),
            };
            state.trace = None;
            Ok(snapshot)
        })
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        CURRENT.with(|slot| {
            let mut state = slot.borrow_mut();
            if state
                .trace
                .as_ref()
                .is_some_and(|t| t.generation == self.generation)
            {
                state.trace = None;
            }
        });
        if self.strict {
            STRICT_STATE.store(0, Ordering::SeqCst);
        }
    }
}

#[inline]
pub fn enabled() -> bool {
    CURRENT.with(|slot| slot.borrow().trace.is_some())
}

/// Disabled callers neither read a clock nor issue a profiler request.
#[inline]
pub fn mark(next: Phase) {
    CURRENT.with(|slot| {
        let mut state = slot.borrow_mut();
        if let Some(trace) = state.trace.as_mut() {
            if trace.online_start.is_some() && !next.online() {
                trace.error = Some("non-target phase inside online interval");
            }
            if trace.phase != next {
                trace.boundary(next);
            }
        } else {
            foreign_boundary();
        }
    });
}

/// One target per session. Preparation must be complete before this call.
pub fn begin_online(first: Phase) {
    CURRENT.with(|slot| {
        let mut state = slot.borrow_mut();
        if let Some(trace) = state.trace.as_mut() {
            if !first.online() || trace.online_start.is_some() || trace.online_ns.is_some() {
                trace.error = Some("invalid or repeated online start");
                return;
            }
            trace.boundary(first);
            trace.online_start = Some(trace.last);
            trace.online_entered[first as usize] = true;
        } else {
            foreign_boundary();
        }
    });
}

pub fn end_online() {
    CURRENT.with(|slot| {
        let mut state = slot.borrow_mut();
        if let Some(trace) = state.trace.as_mut() {
            let Some(start) = trace.online_start else {
                trace.error = Some("online interval was not started");
                return;
            };
            trace.boundary(Phase::Setup);
            trace.online_start = None;
            trace.online_ns = Some(trace.last - start);
        } else {
            foreign_boundary();
        }
    });
}

/// Return to the containing phase even on an early return from a substage.
#[must_use]
pub struct Scope {
    previous: Option<(u64, Phase)>,
    _owner: PhantomData<Rc<()>>,
}

pub fn scope(next: Phase) -> Scope {
    let previous = CURRENT.with(|slot| {
        slot.borrow()
            .trace
            .as_ref()
            .map(|t| (t.generation, t.phase))
    });
    mark(next);
    Scope {
        previous,
        _owner: PhantomData,
    }
}

/// Backend witness checking is shared by collection and individual descent.
pub fn relation_check_scope() -> Scope {
    let online = CURRENT.with(|slot| {
        slot.borrow()
            .trace
            .as_ref()
            .is_some_and(|t| t.online_start.is_some())
    });
    scope(if online {
        Phase::TargetRelationCheck
    } else {
        Phase::RelationCheck
    })
}

impl Drop for Scope {
    fn drop(&mut self) {
        if let Some((generation, previous)) = self.previous {
            let same = CURRENT.with(|slot| {
                slot.borrow()
                    .trace
                    .as_ref()
                    .is_some_and(|t| t.generation == generation)
            });
            if same {
                mark(previous);
            }
        }
    }
}

#[inline(never)]
fn dump(label: &'static [u8]) {
    #[cfg(test)]
    DUMPS.with(|count| count.set(count.get() + 1));
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // Valgrind amd64 DUMP_STATS_AT: dump and reset, keep collection enabled.
        let args = [0x4354_0003usize, label.as_ptr() as usize, 0, 0, 0, 0];
        std::arch::asm!(
            "rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51",
            "xchg rbx, rbx", in("rax") args.as_ptr(), inout("rdx") 0usize => _,
            out("rdi") _, options(nostack)
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = label;
}

/// Dump and reset Callgrind's current interval at an external measurement
/// boundary. Labels must be static, NUL-terminated ASCII. The client request
/// is inert when the process is not running under Valgrind.
pub(crate) fn callgrind_dump(label: &'static [u8]) {
    dump(label);
}

#[cfg(test)]
thread_local! { static DUMPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn exclusive_measurement_closes_online_and_observed_intervals() {
        let session = Session::begin().unwrap();
        mark(Phase::Precompute);
        std::thread::sleep(Duration::from_millis(1));
        begin_online(Phase::TargetQuery);
        std::thread::sleep(Duration::from_millis(1));
        {
            let _pdp = scope(Phase::TargetPdp);
            let _check = scope(Phase::TargetRelationCheck);
        }
        mark(Phase::RecoveryCheck);
        end_online();
        let snapshot = session.finish().unwrap();
        assert_eq!(
            snapshot.phases_ns.values().flatten().sum::<u64>(),
            snapshot.observed_wall_ns
        );
        assert_eq!(
            Some(snapshot.online_phases_ns.values().flatten().sum()),
            snapshot.online_wall_ns
        );
        assert!(snapshot.phases_ns["precompute"].unwrap() >= 1_000_000);
        assert!(snapshot.online_phases_ns["target_query"].unwrap() >= 1_000_000);
        assert_eq!(snapshot.phases_ns["relation_la"], None);
        assert_eq!(snapshot.online_phases_ns["rho_solve"], None);
        assert!(!enabled());
    }

    #[test]
    fn exclusive_measurement_disabled_is_noop_and_errors_drop_the_session() {
        let before = DUMPS.with(|count| count.get());
        mark(Phase::Pdp);
        begin_online(Phase::TargetQuery);
        end_online();
        drop(scope(Phase::RelationLa));
        drop(relation_check_scope());
        assert_eq!(before, DUMPS.with(|count| count.get()));
        let session = Session::begin().unwrap();
        assert!(Session::begin().is_err());
        begin_online(Phase::TargetQuery);
        assert_eq!(session.finish().unwrap_err(), "online interval not closed");
        assert!(!enabled());
        let session = Session::begin().unwrap();
        begin_online(Phase::TargetQuery);
        mark(Phase::Precompute);
        end_online();
        assert_eq!(
            session.finish().unwrap_err(),
            "non-target phase inside online interval"
        );
        assert!(!enabled());
    }

    #[test]
    fn exclusive_measurement_guard_is_thread_local_and_survives_unwind() {
        let session = Session::begin().unwrap();
        std::thread::spawn(|| {
            assert!(!enabled());
            mark(Phase::Pdp);
        })
        .join()
        .unwrap();
        let old_scope = scope(Phase::Precompute);
        drop(session);
        let next = Session::begin().unwrap();
        drop(old_scope); // Cannot change the next session's phase.
        assert_eq!(
            CURRENT.with(|s| s.borrow().trace.as_ref().unwrap().phase),
            Phase::Setup
        );
        drop(next);
        let _ = std::panic::catch_unwind(|| {
            let _session = Session::begin().unwrap();
            panic!("intentional unwind control");
        });
        assert!(!enabled());
    }

    #[test]
    fn exclusive_measurement_unstarted_and_repeated_online_are_distinct() {
        let snapshot = Session::begin().unwrap().finish().unwrap();
        assert_eq!(snapshot.online_wall_ns, None);
        assert!(snapshot.online_phases_ns.values().all(Option::is_none));
        assert_eq!(snapshot.phases_ns["pdp"], None);
        let session = Session::begin().unwrap();
        begin_online(Phase::TargetQuery);
        end_online();
        begin_online(Phase::TargetQuery);
        assert_eq!(
            session.finish().unwrap_err(),
            "invalid or repeated online start"
        );
        assert!(!enabled());
    }

    #[test]
    fn exclusive_measurement_relation_check_uses_the_correct_interval() {
        let session = Session::begin().unwrap();
        {
            let _check = relation_check_scope();
        }
        begin_online(Phase::TargetPdp);
        {
            let _check = relation_check_scope();
        }
        end_online();
        let snapshot = session.finish().unwrap();
        assert!(snapshot.phases_ns["relation_check"].is_some());
        assert!(snapshot.online_phases_ns["target_relation_check"].is_some());
        assert_eq!(
            Some(snapshot.online_phases_ns.values().flatten().sum()),
            snapshot.online_wall_ns
        );
    }

    #[test]
    #[ignore = "process-exclusive strict session; CI runs this isolated with --include-ignored"]
    fn exclusive_measurement_strict_session_rejects_foreign_boundaries() {
        let session = Session::begin_strict().unwrap();
        std::thread::spawn(|| {
            assert!(!enabled());
            mark(Phase::Pdp);
        })
        .join()
        .unwrap();
        assert_eq!(
            session.finish().unwrap_err(),
            "phase boundary occurred off the controlling thread"
        );
        assert!(!enabled());
        let session = Session::begin_strict().unwrap();
        mark(Phase::Pdp);
        assert!(session.finish().is_ok());
    }
}
