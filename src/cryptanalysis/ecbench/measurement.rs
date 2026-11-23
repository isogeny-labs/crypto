//! Shared process observations and frozen ecbench isolation grading.
//!
//! Kept independent of solver/record types so native command runners use
//! the same rules as ecbench. Existing record paths reexport this API.

use crate::cryptanalysis::ecbench::host::{is_virtual, HostCapsule};
use crate::cryptanalysis::ecbench::isolation::{
    cpu_schedstat, current_cpu, irq_counts, Conditions, CpuPlan, CpuSched, EvictionSummary,
    HwCounts, IrqCounts, Level, Preflight, SelfPlacement, Thresholds,
};
use serde::{Deserialize, Serialize};

/// The rules [`grade`] applies.  Version 2 added the unreserved-sibling
/// check, the observed siblings, the policy-and-pages NUMA read-back and
/// the session's own tick rate.  Version 3 measures the solve rather than
/// the process (involuntary switches, run-queue delay, IRQs and foreign
/// scheduling over the solve window alone), gives every gate an absolute
/// floor and normalises it by duration, counts the child's threads, adds
/// the minimum solve length, the loosened-threshold and intruder blockers.
/// A session is regraded under the version it was graded with
/// ([`grade_for_version`]); [`grade_v2`] is frozen.
pub const GRADING_VERSION: u32 = 3;

/// `/proc/self/schedstat`: on-CPU time, run-queue wait, timeslices.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedStat {
    pub cpu_ns: u64,
    pub run_delay_ns: u64,
    pub timeslices: u64,
}

pub fn schedstat() -> Option<SchedStat> {
    #[cfg(target_os = "linux")]
    {
        let s = std::fs::read_to_string("/proc/self/schedstat").ok()?;
        let v: Vec<u64> = s
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();
        if v.len() >= 3 {
            return Some(SchedStat {
                cpu_ns: v[0],
                run_delay_ns: v[1],
                timeslices: v[2],
            });
        }
    }
    None
}

/// What the child saw of its own process and CPU across the solve alone.
///
/// The process-wide figures `wait4` reports cover the exec, the set-up
/// and the exit as well; the gates that grade a *solve* read these
/// instead.  Every counter is a difference of two reads taken just
/// outside the timed window.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SolveObs {
    /// Entries of `/proc/self/task` before and after the solve.
    pub threads_before: Option<u32>,
    pub threads_after: Option<u32>,
    /// `getrusage(RUSAGE_SELF)` differences over the solve.
    pub involuntary_switches: Option<u64>,
    pub voluntary_switches: Option<u64>,
    pub minor_faults: Option<u64>,
    pub major_faults: Option<u64>,
    /// `/proc/self/task/*/schedstat` summed over every thread.
    pub schedstat_all: Option<SchedStat>,
    /// The CPU the counters below belong to (the child's CPU).
    pub cpu: Option<u32>,
    /// That CPU's interrupts and softirqs during the solve.
    pub irq: Option<IrqCounts>,
    /// That CPU's `/proc/schedstat` line (needs `CONFIG_SCHEDSTATS`).
    pub cpu_sched: Option<CpuSched>,
}

/// A point-in-time read of everything [`SolveObs`] differences.
pub struct SolveProbe {
    threads: Option<u32>,
    rusage: Option<[u64; 4]>,
    schedstat_all: Option<SchedStat>,
    cpu: Option<u32>,
    irq: Option<IrqCounts>,
    cpu_sched: Option<CpuSched>,
}

fn own_thread_count() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        Some(std::fs::read_dir("/proc/self/task").ok()?.count() as u32)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

/// `(nivcsw, nvcsw, minflt, majflt)` of the whole process.
fn own_rusage() -> Option<[u64; 4]> {
    #[cfg(unix)]
    {
        // SAFETY: getrusage fills a zeroed struct.
        unsafe {
            let mut ru: libc::rusage = std::mem::zeroed();
            if libc::getrusage(libc::RUSAGE_SELF, &mut ru) != 0 {
                return None;
            }
            Some([
                ru.ru_nivcsw as u64,
                ru.ru_nvcsw as u64,
                ru.ru_minflt as u64,
                ru.ru_majflt as u64,
            ])
        }
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// The sum of every thread's `schedstat`.
fn schedstat_all_tasks() -> Option<SchedStat> {
    #[cfg(target_os = "linux")]
    {
        let mut total = SchedStat::default();
        let mut any = false;
        for t in std::fs::read_dir("/proc/self/task").ok()?.flatten() {
            let Ok(s) = std::fs::read_to_string(t.path().join("schedstat")) else {
                continue;
            };
            let v: Vec<u64> = s
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            if v.len() >= 3 {
                total.cpu_ns += v[0];
                total.run_delay_ns += v[1];
                total.timeslices += v[2];
                any = true;
            }
        }
        any.then_some(total)
    }
    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

impl SolveProbe {
    pub fn read() -> Self {
        let cpu = current_cpu();
        Self {
            threads: own_thread_count(),
            irq: cpu.and_then(irq_counts),
            cpu_sched: cpu.and_then(cpu_schedstat),
            rusage: own_rusage(),
            schedstat_all: schedstat_all_tasks(),
            cpu,
        }
    }

    pub fn since(&self, earlier: &SolveProbe) -> SolveObs {
        let d = |a: Option<[u64; 4]>, b: Option<[u64; 4]>, i: usize| match (a, b) {
            (Some(a), Some(b)) => Some(a[i].saturating_sub(b[i])),
            _ => None,
        };
        // Per-CPU figures mean nothing if the child changed CPU.
        let same_cpu = self.cpu.is_some() && self.cpu == earlier.cpu;
        SolveObs {
            threads_before: earlier.threads,
            threads_after: self.threads,
            involuntary_switches: d(self.rusage, earlier.rusage, 0),
            voluntary_switches: d(self.rusage, earlier.rusage, 1),
            minor_faults: d(self.rusage, earlier.rusage, 2),
            major_faults: d(self.rusage, earlier.rusage, 3),
            schedstat_all: match (self.schedstat_all, earlier.schedstat_all) {
                (Some(b), Some(a)) => Some(SchedStat {
                    cpu_ns: b.cpu_ns.saturating_sub(a.cpu_ns),
                    run_delay_ns: b.run_delay_ns.saturating_sub(a.run_delay_ns),
                    timeslices: b.timeslices.saturating_sub(a.timeslices),
                }),
                _ => None,
            },
            cpu: self.cpu.filter(|_| same_cpu),
            irq: match (self.irq, earlier.irq) {
                (Some(b), Some(a)) if same_cpu => Some(b.since(&a)),
                _ => None,
            },
            cpu_sched: match (self.cpu_sched, earlier.cpu_sched) {
                (Some(b), Some(a)) if same_cpu => Some(b.since(&a)),
                _ => None,
            },
        }
    }
}

/// Exported producer clock status. An absent object in a historical record is
/// unknown; an object with `error: null` explicitly records no producer error.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlineClockDiagnostics {
    pub error: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Timing {
    /// In-process wall time of the algorithm alone.
    pub solve_wall_ns: Option<u64>,
    /// Shared enclosing method call, including per-call setup, in-child
    /// verification and report construction. Never added to its nested timers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_call_wall_ns: Option<u64>,
    /// Independent mathematical verification in the parent after child exit.
    /// Diagnostic audit overhead; does not inherit the child's L2 isolation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_audit_verification_ns: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_clock_diagnostics: Option<OnlineClockDiagnostics>,
    /// Fork to reap, as the runner timed it: a practicality figure.
    pub process_wall_ns: u64,
    pub user_ns: u64,
    pub sys_ns: u64,
    pub max_rss_kib: u64,
    pub minor_faults: u64,
    pub major_faults: u64,
    pub voluntary_switches: u64,
    pub involuntary_switches: u64,
    pub schedstat_solve: Option<SchedStat>,
    /// User-space instructions and cycles across the solve, where the
    /// host has a hardware PMU.
    #[serde(default)]
    pub hw_solve: Option<HwCounts>,
    /// The child's own view of the solve window (grading version 3).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solve: Option<SolveObs>,
}

// ── Grading ────────────────────────────────────────────────────────

/// Everything grading reads.  Each check is a blocker for the level it
/// belongs to; a run earns the highest level with no blocker at it or
/// below.
pub struct GradeInput<'a> {
    pub plan: Option<&'a CpuPlan>,
    pub preflight: Option<&'a Preflight>,
    pub eviction: Option<&'a EvictionSummary>,
    pub runner_on_run_cpu: bool,
    pub capsule: &'a HostCapsule,
    pub thresholds: &'a Thresholds,
    pub placement: &'a SelfPlacement,
    pub before: &'a Conditions,
    pub after: &'a Conditions,
    pub timing: &'a Timing,
    /// Clock ticks per second on the host that recorded the ticks.
    pub hz: f64,
}

pub struct Grade {
    pub level: Level,
    pub blockers: Vec<String>,
    pub foreign_busy_ticks: Option<i64>,
    pub steal_ticks: Option<u64>,
    pub idle_sibling_busy_ticks: Option<u64>,
}

/// The rules of grading version 2, frozen: sessions recorded under them
/// regrade identically.  Do not edit; add a version instead.
pub fn grade_v2(g: &GradeInput) -> Grade {
    let mut l1: Vec<String> = Vec::new();
    let mut l2: Vec<String> = Vec::new();
    let mut l3: Vec<String> = Vec::new();
    let mut foreign = None;
    let mut steal = None;
    let mut sib_busy = None;
    let hz = g.hz;
    let wall_s = g.timing.process_wall_ns as f64 / 1e9;
    let cpu_s = (g.timing.user_ns + g.timing.sys_ns) as f64 / 1e9;

    if g.capsule.build.debug_assertions {
        l1.push("debug build: its wall time measures the build, not the method".into());
    }
    match g.plan {
        None => {
            l1.push("no CPU reservation (--cpus none, or no affinity control on this OS)".into())
        }
        Some(plan) => {
            let run = plan.run_cpu;
            if !plan.unreserved_siblings.is_empty() {
                l1.push(format!(
                    "the run CPU's SMT siblings {:?} are outside the reservation",
                    plan.unreserved_siblings
                ));
            }
            if g.placement.affinity.as_deref() != Some(&[run][..]) {
                l1.push(format!(
                    "affinity read back inside the child was {:?}, not [{run}]",
                    g.placement.affinity
                ));
            }
            if g.placement.cpu_at_start != Some(run) || g.placement.cpu_at_end != Some(run) {
                l1.push(format!(
                    "child ran on CPU {:?} → {:?}, not {run}",
                    g.placement.cpu_at_start, g.placement.cpu_at_end
                ));
            }
            let core0 = g
                .capsule
                .stable
                .topology
                .place(0)
                .map(|p| p.siblings.clone())
                .unwrap_or_else(|| vec![0]);
            if core0.contains(&run) {
                l1.push("the run CPU shares CPU 0's core, where housekeeping lands".into());
            }
            if g.runner_on_run_cpu {
                l1.push("the runner itself had no CPU outside the reservation".into());
            }
            match g.eviction {
                Some(ev) if ev.failed_user_threads > 0 => l1.push(format!(
                    "{} user threads could not be moved off the reserved CPUs (not root?)",
                    ev.failed_user_threads
                )),
                None => l1.push("no eviction was performed".into()),
                _ => {}
            }
            if wall_s > 0.0 && cpu_s / wall_s > 1.05 {
                l1.push(format!(
                    "CPU time {cpu_s:.3}s over wall {wall_s:.3}s: more than one thread ran"
                ));
            }
            if g.capsule.stable.numa_nodes > 1 {
                if let Some(node) = plan.node {
                    // The policy read back inside the child, and where its
                    // anonymous pages actually are.  `Mems_allowed` is the
                    // cpuset's permission, not the policy, and is not used.
                    let want = format!("bind:{node}");
                    if g.placement.mempolicy.as_deref() != Some(&want[..]) {
                        l1.push(format!(
                            "memory policy read back as {:?}, not {want}",
                            g.placement.mempolicy
                        ));
                    }
                    match &g.placement.anon_pages_by_node {
                        Some(pages) => {
                            let total: u64 = pages.values().sum();
                            let local = pages.get(&node).copied().unwrap_or(0);
                            if total > 0 && (local as f64) < 0.99 * total as f64 {
                                l1.push(format!(
                                    "{} of {total} anonymous pages on node {node}",
                                    local
                                ));
                            }
                        }
                        None => l1.push("no /proc/self/numa_maps to confirm page placement".into()),
                    }
                }
            }
            // L2: the window was quiet.
            match g.preflight {
                Some(p) if p.quiet => {}
                Some(p) => l2.push(format!("preflight not quiet: {}", p.reasons.join("; "))),
                None => l2.push("no preflight".into()),
            }
            let tick = |c: &Conditions, cpu: u32| c.reserved_ticks.get(&cpu).copied();
            match (tick(g.before, run), tick(g.after, run)) {
                (Some(a), Some(b)) => {
                    let busy = b.busy.saturating_sub(a.busy) as f64;
                    let child_ticks = cpu_s * hz;
                    let f = (busy - child_ticks).round() as i64;
                    foreign = Some(f);
                    let allowance = (2.0f64).max(0.01 * wall_s * hz);
                    if f as f64 > allowance {
                        l2.push(format!(
                            "{f} ticks of foreign work on the run CPU during the run"
                        ));
                    }
                    let st = b.steal.saturating_sub(a.steal);
                    steal = Some(st);
                    if st > 0 {
                        l2.push(format!("{st} ticks of hypervisor steal on the run CPU"));
                    }
                }
                _ => l2.push("no /proc/stat ticks for the run CPU".into()),
            }
            // Every other observed CPU: the reserved idle siblings, and under
            // an inherited placement the run CPU's siblings that are not ours.
            let mut sb = 0u64;
            for c in plan.observed_cpus().into_iter().filter(|&c| c != run) {
                if let (Some(a), Some(b)) = (tick(g.before, c), tick(g.after, c)) {
                    sb += b.busy.saturating_sub(a.busy);
                }
            }
            sib_busy = Some(sb);
            if sb > 2 {
                l2.push(format!("{sb} busy ticks on the run CPU's idle siblings and the rest of the reservation"));
            }
            match (g.timing.schedstat_solve, g.timing.solve_wall_ns) {
                (Some(s), Some(w)) if w > 0 => {
                    if s.run_delay_ns as f64 > 0.005 * w as f64 {
                        l2.push(format!(
                            "run-queue delay {:.3}% of the solve",
                            100.0 * s.run_delay_ns as f64 / w as f64
                        ));
                    }
                }
                _ => l2.push("no schedstat for the solve".into()),
            }
            if wall_s > 0.0 {
                let rate = g.timing.involuntary_switches as f64 / wall_s;
                if rate > g.thresholds.max_preemptions_per_second {
                    l2.push(format!("{rate:.1} preemptions per second"));
                }
            }
            match (
                g.before.psi_memory_some_total_us,
                g.after.psi_memory_some_total_us,
            ) {
                (Some(a), Some(b)) => {
                    let stall = b.saturating_sub(a) as f64 / 1e6;
                    if stall > 0.005 * wall_s.max(0.001) {
                        l2.push(format!("{stall:.4}s of memory stall system-wide"));
                    }
                }
                _ => l2.push("no memory PSI".into()),
            }
            // L3: the host is configured for measurement.
            let s = g.capsule.stable.cpu_settings.get(&run);
            let isolated = s.map(|s| s.in_isolcpus == Some(true) || s.in_nohz_full == Some(true));
            if isolated != Some(true) {
                l3.push("run CPU is in neither isolcpus nor nohz_full".into());
            }
            if s.and_then(|s| s.governor.as_deref()) != Some("performance") {
                l3.push(format!(
                    "governor is {:?}, not performance",
                    s.and_then(|s| s.governor.clone())
                ));
            }
            let turbo_off =
                s.map(|s| s.no_turbo.as_deref() == Some("1") || s.boost.as_deref() == Some("0"));
            if turbo_off != Some(true) {
                l3.push("turbo/boost is not known to be off".into());
            }
            if is_virtual(&g.capsule.stable) != Some(false) {
                l3.push("not known to be bare metal".into());
            }
        }
    }
    let level = if !l1.is_empty() {
        Level::L0
    } else if !l2.is_empty() {
        Level::L1
    } else if !l3.is_empty() {
        Level::L2
    } else {
        Level::L3
    };
    let mut blockers = Vec::new();
    for (lv, list) in [("L1", l1), ("L2", l2), ("L3", l3)] {
        blockers.extend(list.into_iter().map(|b| format!("{lv}: {b}")));
    }
    Grade {
        level,
        blockers,
        foreign_busy_ticks: foreign,
        steal_ticks: steal,
        idle_sibling_busy_ticks: sib_busy,
    }
}

/// The shortest solve whose wall time grading version 3 admits.  Below it
/// one wake-up or one interrupt is a visible fraction of the figure, and
/// whatever the gates allow the number is dominated by events, not the
/// algorithm.
pub const MIN_SOLVE_WALL_NS_V3: u64 = 1_000_000;
/// The blocker's stable prefix: a reader (or `compare`) can tell a run
/// that was too short from one that was disturbed.
pub const TOO_SHORT_TO_ADMIT: &str = "solve too short to admit wall time";
const V3_SETTLE_SECONDS: f64 = 2.0;
const V3_MAX_OTHER_CPU: f64 = 0.10;
const V3_MAX_PSI: f64 = 5.0;
const V3_MAX_PREEMPTIONS_PER_SECOND: f64 = 50.0;
/// One wake-up's worth of run-queue delay, ns.
const V3_RUNQ_FLOOR_NS: u64 = 100_000;
/// Foreign task time on the run CPU one slice may take, ns.
const V3_FOREIGN_FLOOR_NS: u64 = 100_000;
/// Non-tick interrupts allowed on the run CPU: a floor and a rate.
const V3_IRQ_FLOOR: u64 = 2;
const V3_IRQS_PER_SECOND: f64 = 100.0;
/// System-wide memory stall allowed whatever the run's length, seconds.
const V3_PSI_FLOOR_S: f64 = 0.001;

/// The rules of grading version 3.
///
/// Version 2 graded a few-millisecond solve against figures taken over the
/// whole process (exec, set-up, exit included), where one context switch,
/// one wake-up or one 10 ms steal tick cost a level.  Version 3 measures the
/// solve alone, gives every gate an absolute floor (a single event cannot
/// fail a short solve) and scales the allowance with the duration; it
/// counts the child's threads (CPU time over wall can never exceed one on
/// a pinned child), reads IRQ and scheduler counters that see events
/// 10 ms ticks cannot, scans the reservation for intruders on every run,
/// names preflight thresholds that were loosened, and refuses to admit wall
/// time from a solve shorter than [`MIN_SOLVE_WALL_NS_V3`].
///
/// Every number below is part of the version: change one and the version
/// changes.
pub fn grade_v3(g: &GradeInput) -> Grade {
    let mut l1: Vec<String> = Vec::new();
    let mut l2: Vec<String> = Vec::new();
    let mut l3: Vec<String> = Vec::new();
    let mut foreign = None;
    let mut steal = None;
    let mut sib_busy = None;
    let hz = g.hz;
    let wall_s = g.timing.process_wall_ns as f64 / 1e9;
    let cpu_s = (g.timing.user_ns + g.timing.sys_ns) as f64 / 1e9;
    let solve_ns = g.timing.solve_wall_ns;
    let solve_s = solve_ns.map(|n| n as f64 / 1e9);
    let obs = g.timing.solve.as_ref();

    if g.capsule.build.debug_assertions {
        l1.push("debug build: its wall time measures the build, not the method".into());
    }
    match g.plan {
        None => {
            l1.push("no CPU reservation (--cpus none, or no affinity control on this OS)".into())
        }
        Some(plan) => {
            let run = plan.run_cpu;
            if !plan.unreserved_siblings.is_empty() {
                l1.push(format!(
                    "the run CPU's SMT siblings {:?} are outside the reservation",
                    plan.unreserved_siblings
                ));
            }
            if g.placement.affinity.as_deref() != Some(&[run][..]) {
                l1.push(format!(
                    "affinity read back inside the child was {:?}, not [{run}]",
                    g.placement.affinity
                ));
            }
            if g.placement.cpu_at_start != Some(run) || g.placement.cpu_at_end != Some(run) {
                l1.push(format!(
                    "child ran on CPU {:?} → {:?}, not {run}",
                    g.placement.cpu_at_start, g.placement.cpu_at_end
                ));
            }
            let core0 = g
                .capsule
                .stable
                .topology
                .place(0)
                .map(|p| p.siblings.clone())
                .unwrap_or_else(|| vec![0]);
            if core0.contains(&run) {
                l1.push("the run CPU shares CPU 0's core, where housekeeping lands".into());
            }
            if g.runner_on_run_cpu {
                l1.push("the runner itself had no CPU outside the reservation".into());
            }
            match g.eviction {
                Some(ev) if ev.failed_user_threads > 0 => l1.push(format!(
                    "{} user threads could not be moved off the reserved CPUs (not root?)",
                    ev.failed_user_threads
                )),
                None => l1.push("no eviction was performed".into()),
                _ => {}
            }
            // Threads that appeared on the reservation after the eviction,
            // or got a wide mask back: the larger of the scans taken before
            // and after the run, less the ones the eviction already failed.
            let known = g.eviction.map(|e| e.failed_user_threads).unwrap_or(0);
            let seen = [g.before.intruders.as_ref(), g.after.intruders.as_ref()]
                .into_iter()
                .flatten()
                .map(|s| s.user_threads as u64)
                .max();
            match seen {
                Some(n) if n > known => l1.push(format!(
                    "{} user threads other than the runner's could run on the reserved CPUs during the run",
                    n - known
                )),
                Some(_) => {}
                None => l1.push("no scan of the reservation for intruders".into()),
            }
            // The child's own threads: counted, since a pinned child's CPU
            // time can never exceed its wall time.
            match obs.map(|o| (o.threads_before, o.threads_after)) {
                Some((a, b)) => {
                    let n = a.into_iter().chain(b).max();
                    match n {
                        Some(n) if n > 1 => {
                            l1.push(format!("the child ran {n} threads: more than one thread"))
                        }
                        Some(_) => {}
                        None => l1.push("the child did not report its thread count".into()),
                    }
                }
                None => l1.push("the child did not report its thread count".into()),
            }
            if wall_s > 0.0 && cpu_s / wall_s > 1.05 {
                l1.push(format!(
                    "CPU time {cpu_s:.3}s over wall {wall_s:.3}s: more than one thread ran"
                ));
            }
            if g.capsule.stable.numa_nodes > 1 {
                if let Some(node) = plan.node {
                    let want = format!("bind:{node}");
                    if g.placement.mempolicy.as_deref() != Some(&want[..]) {
                        l1.push(format!(
                            "memory policy read back as {:?}, not {want}",
                            g.placement.mempolicy
                        ));
                    }
                    match &g.placement.anon_pages_by_node {
                        Some(pages) => {
                            let total: u64 = pages.values().sum();
                            let local = pages.get(&node).copied().unwrap_or(0);
                            if total > 0 && (local as f64) < 0.99 * total as f64 {
                                l1.push(format!(
                                    "{} of {total} anonymous pages on node {node}",
                                    local
                                ));
                            }
                        }
                        None => l1.push("no /proc/self/numa_maps to confirm page placement".into()),
                    }
                }
            }
            // L2: the window was quiet.
            match g.preflight {
                Some(p) if p.quiet => {}
                Some(p) => l2.push(format!("preflight not quiet: {}", p.reasons.join("; "))),
                None => l2.push("no preflight".into()),
            }
            // A session run with the preflight loosened says so.
            let mut loose = Vec::new();
            if g.thresholds.settle_seconds < V3_SETTLE_SECONDS {
                loose.push(format!(
                    "settle {} s < {V3_SETTLE_SECONDS} s",
                    g.thresholds.settle_seconds
                ));
            }
            if g.thresholds.max_other_cpu > V3_MAX_OTHER_CPU {
                loose.push(format!(
                    "max-other-cpu {} > {V3_MAX_OTHER_CPU}",
                    g.thresholds.max_other_cpu
                ));
            }
            if g.thresholds.max_psi > V3_MAX_PSI {
                loose.push(format!("max-psi {} > {V3_MAX_PSI}", g.thresholds.max_psi));
            }
            if g.thresholds.max_preemptions_per_second > V3_MAX_PREEMPTIONS_PER_SECOND {
                loose.push(format!(
                    "max-preemptions-per-second {} > {V3_MAX_PREEMPTIONS_PER_SECOND}",
                    g.thresholds.max_preemptions_per_second
                ));
            }
            if !loose.is_empty() {
                l2.push(format!(
                    "preflight thresholds looser than the defaults: {}",
                    loose.join(", ")
                ));
            }
            // Too short to time: a few tens of microseconds of solve are
            // dominated by one event whatever the gates allow.
            match solve_ns {
                Some(n) if n < MIN_SOLVE_WALL_NS_V3 => l2.push(format!(
                    "{TOO_SHORT_TO_ADMIT}: solve {:.1} µs is under the {:.1} ms minimum",
                    n as f64 / 1e3,
                    MIN_SOLVE_WALL_NS_V3 as f64 / 1e6
                )),
                Some(_) => {}
                None => l2.push(format!("{TOO_SHORT_TO_ADMIT}: no solve wall time")),
            }
            let tick = |c: &Conditions, cpu: u32| c.reserved_ticks.get(&cpu).copied();
            match (tick(g.before, run), tick(g.after, run)) {
                (Some(a), Some(b)) => {
                    let busy = b.busy.saturating_sub(a.busy) as f64;
                    let child_ticks = cpu_s * hz;
                    let f = (busy - child_ticks).round() as i64;
                    foreign = Some(f);
                    let allowance = (2.0f64).max(0.01 * wall_s * hz);
                    if f as f64 > allowance {
                        l2.push(format!(
                            "{f} ticks of foreign work on the run CPU during the run"
                        ));
                    }
                    let st = b.steal.saturating_sub(a.steal);
                    steal = Some(st);
                    // One tick is the resolution of the counter: more than
                    // one, or more than 1 % of the run, is steal.
                    let steal_allowance = (1.0f64).max(0.01 * wall_s * hz);
                    if st as f64 > steal_allowance {
                        l2.push(format!("{st} ticks of hypervisor steal on the run CPU"));
                    }
                }
                _ => l2.push("no /proc/stat ticks for the run CPU".into()),
            }
            let mut sb = 0u64;
            for c in plan.observed_cpus().into_iter().filter(|&c| c != run) {
                if let (Some(a), Some(b)) = (tick(g.before, c), tick(g.after, c)) {
                    sb += b.busy.saturating_sub(a.busy);
                }
            }
            sib_busy = Some(sb);
            if sb as f64 > (2.0f64).max(0.01 * wall_s * hz) {
                l2.push(format!("{sb} busy ticks on the run CPU's idle siblings and the rest of the reservation"));
            }
            // Run-queue delay over the solve, every thread summed; a single
            // wake-up (100 µs) is allowed whatever the length.
            let delay = obs
                .and_then(|o| o.schedstat_all)
                .or(g.timing.schedstat_solve);
            match (delay, solve_ns) {
                (Some(s), Some(w)) if w > 0 => {
                    let allowed = (V3_RUNQ_FLOOR_NS as f64).max(0.005 * w as f64);
                    if s.run_delay_ns as f64 > allowed {
                        l2.push(format!(
                            "run-queue delay {} µs is {:.3}% of the solve",
                            s.run_delay_ns / 1000,
                            100.0 * s.run_delay_ns as f64 / w as f64
                        ));
                    }
                }
                _ => l2.push("no schedstat for the solve".into()),
            }
            // Preemption over the solve: one involuntary switch is allowed;
            // beyond that the rate limit scales with the solve's length.
            match (obs.and_then(|o| o.involuntary_switches), solve_s) {
                (Some(n), Some(s)) => {
                    let allowed = (g.thresholds.max_preemptions_per_second * s)
                        .ceil()
                        .max(1.0);
                    if n as f64 > allowed {
                        l2.push(format!(
                            "{n} involuntary context switches during a {:.3} ms solve (at most {allowed})",
                            s * 1e3
                        ));
                    }
                }
                _ if wall_s > 0.0 => {
                    // No view of the solve: the whole process, with the
                    // exec's own switches allowed for.
                    let n = g.timing.involuntary_switches;
                    let allowed = (g.thresholds.max_preemptions_per_second * wall_s)
                        .ceil()
                        .max(2.0);
                    if n as f64 > allowed {
                        l2.push(format!(
                            "{n} involuntary context switches over the process (at most {allowed})"
                        ));
                    }
                }
                _ => {}
            }
            // Interrupts and scheduling on the run CPU during the solve:
            // exact counts, where /proc/stat sees only 10 ms ticks.
            if let (Some(s), Some(o)) = (solve_s, obs) {
                if let Some(irq) = o.irq {
                    let allowed = ((V3_IRQS_PER_SECOND * s).ceil() as u64).max(V3_IRQ_FLOOR);
                    if irq.non_tick() > allowed {
                        l2.push(format!(
                            "{} interrupts on the run CPU during the solve ({} device, {} IPI/other, {} softirq; at most {allowed})",
                            irq.non_tick(), irq.device, irq.ipi_other, irq.softirq_work
                        ));
                    }
                }
                if let Some(cs) = o.cpu_sched {
                    // Arrivals on the run CPU that were not the child's own:
                    // another task ran there during the solve.
                    let own = o.schedstat_all.map(|s| s.timeslices).unwrap_or(0);
                    let foreign_arrivals = cs.pcount.saturating_sub(own);
                    let foreign_ns = if o.involuntary_switches.unwrap_or(0)
                        + o.voluntary_switches.unwrap_or(0)
                        == 0
                    {
                        Some(cs.rq_cpu_ns)
                    } else {
                        None
                    };
                    let long = foreign_ns
                        .map(|n| n as f64 > (V3_FOREIGN_FLOOR_NS as f64).max(0.01 * s * 1e9))
                        .unwrap_or(false);
                    if foreign_arrivals > 1 || long {
                        l2.push(format!(
                            "{foreign_arrivals} foreign task slices on the run CPU during the solve ({} µs)",
                            foreign_ns.map(|n| n / 1000).unwrap_or(0)
                        ));
                    }
                }
            }
            match (
                g.before.psi_memory_some_total_us,
                g.after.psi_memory_some_total_us,
            ) {
                (Some(a), Some(b)) => {
                    let stall = b.saturating_sub(a) as f64 / 1e6;
                    if stall > (0.005 * wall_s.max(0.001)).max(V3_PSI_FLOOR_S) {
                        l2.push(format!("{stall:.4}s of memory stall system-wide"));
                    }
                }
                _ => l2.push("no memory PSI".into()),
            }
            // L3: the host is configured for measurement.
            let s = g.capsule.stable.cpu_settings.get(&run);
            let isolated = s.map(|s| s.in_isolcpus == Some(true) || s.in_nohz_full == Some(true));
            if isolated != Some(true) {
                l3.push("run CPU is in neither isolcpus nor nohz_full".into());
            }
            if s.and_then(|s| s.governor.as_deref()) != Some("performance") {
                l3.push(format!(
                    "governor is {:?}, not performance",
                    s.and_then(|s| s.governor.clone())
                ));
            }
            let turbo_off =
                s.map(|s| s.no_turbo.as_deref() == Some("1") || s.boost.as_deref() == Some("0"));
            if turbo_off != Some(true) {
                l3.push("turbo/boost is not known to be off".into());
            }
            if is_virtual(&g.capsule.stable) != Some(false) {
                l3.push("not known to be bare metal".into());
            }
        }
    }
    let level = if !l1.is_empty() {
        Level::L0
    } else if !l2.is_empty() {
        Level::L1
    } else if !l3.is_empty() {
        Level::L2
    } else {
        Level::L3
    };
    let mut blockers = Vec::new();
    for (lv, list) in [("L1", l1), ("L2", l2), ("L3", l3)] {
        blockers.extend(list.into_iter().map(|b| format!("{lv}: {b}")));
    }
    Grade {
        level,
        blockers,
        foreign_busy_ticks: foreign,
        steal_ticks: steal,
        idle_sibling_busy_ticks: sib_busy,
    }
}

/// Grade `g` under the rules of `version`; `None` for a version this
/// binary does not implement (version 1 predates the checks in code, and
/// a version from the future is not known).  An audit regrades a session
/// under the version recorded in `session.json` with this.
pub fn grade_for_version(version: u32, g: &GradeInput) -> Option<Grade> {
    match version {
        2 => Some(grade_v2(g)),
        3 => Some(grade_v3(g)),
        _ => None,
    }
}

/// Whether [`grade_for_version`] can regrade a session of `version`.
pub fn grading_version_supported(version: u32) -> bool {
    matches!(version, 2 | 3)
}

/// The current rules (`GRADING_VERSION`).
pub fn grade(g: &GradeInput) -> Grade {
    grade_v3(g)
}

#[cfg(test)]
mod grade_tests {
    use super::*;
    use crate::cryptanalysis::ecbench::host::capture;
    use crate::cryptanalysis::ecbench::isolation::{
        CpuPlace, CpuRequest, CpuTicks, IntruderScan, Topology,
    };

    struct Fixture {
        capsule: HostCapsule,
        plan: CpuPlan,
        eviction: EvictionSummary,
        preflight: Preflight,
        before: Conditions,
        after: Conditions,
        placement: SelfPlacement,
        timing: Timing,
        thresholds: Thresholds,
    }

    impl Fixture {
        /// A quiet 5 ms solve on CPU 3 of a four-core machine.
        fn quiet() -> Self {
            let mut capsule = capture().unwrap();
            capsule.build.debug_assertions = false;
            capsule.stable.numa_nodes = 1;
            capsule.stable.topology = Topology {
                cpus: (0..4)
                    .map(|c| CpuPlace {
                        cpu: c,
                        core_id: Some(c),
                        package: Some(0),
                        node: Some(0),
                        siblings: vec![c],
                    })
                    .collect(),
                nodes: vec![],
            };
            let plan = CpuPlan {
                run_cpu: 3,
                reserved: vec![3],
                node: None,
                request: CpuRequest::Explicit(vec![3]),
                unreserved_siblings: vec![],
                observed: vec![3],
            };
            let mut c = Conditions {
                psi_memory_some_total_us: Some(0),
                intruders: Some(IntruderScan::default()),
                ..Default::default()
            };
            c.reserved_ticks.insert(
                3,
                CpuTicks {
                    busy: 100,
                    idle: 0,
                    steal: 0,
                },
            );
            let one = SchedStat {
                cpu_ns: 0,
                run_delay_ns: 0,
                timeslices: 1,
            };
            let timing = Timing {
                solve_wall_ns: Some(5_000_000),
                process_wall_ns: 30_000_000,
                user_ns: 25_000_000,
                sys_ns: 3_000_000,
                schedstat_solve: Some(one),
                solve: Some(SolveObs {
                    threads_before: Some(1),
                    threads_after: Some(1),
                    involuntary_switches: Some(0),
                    voluntary_switches: Some(0),
                    schedstat_all: Some(one),
                    cpu: Some(3),
                    irq: Some(IrqCounts {
                        local_timer: 1,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            };
            Fixture {
                capsule,
                plan,
                eviction: EvictionSummary::default(),
                preflight: Preflight {
                    quiet: true,
                    ..Default::default()
                },
                before: c.clone(),
                after: c,
                placement: SelfPlacement {
                    affinity: Some(vec![3]),
                    cpu_at_start: Some(3),
                    cpu_at_end: Some(3),
                    ..Default::default()
                },
                timing,
                thresholds: Thresholds::default(),
            }
        }

        fn input(&self) -> GradeInput<'_> {
            GradeInput {
                plan: Some(&self.plan),
                preflight: Some(&self.preflight),
                eviction: Some(&self.eviction),
                runner_on_run_cpu: false,
                capsule: &self.capsule,
                thresholds: &self.thresholds,
                placement: &self.placement,
                before: &self.before,
                after: &self.after,
                timing: &self.timing,
                hz: 100.0,
            }
        }

        fn blockers(&self, level: &str) -> Vec<String> {
            grade_v3(&self.input())
                .blockers
                .into_iter()
                .filter(|b| b.starts_with(level))
                .collect()
        }
    }

    fn has(blockers: &[String], needle: &str) -> bool {
        blockers.iter().any(|b| b.contains(needle))
    }

    #[test]
    fn a_quiet_solve_has_no_l1_or_l2_blocker() {
        let f = Fixture::quiet();
        assert!(f.blockers("L1").is_empty(), "{:?}", f.blockers("L1"));
        assert!(f.blockers("L2").is_empty(), "{:?}", f.blockers("L2"));
        assert!(grade_v3(&f.input()).level >= Level::L2);
    }

    #[test]
    fn one_switch_and_one_wakeup_cannot_fail_a_short_solve() {
        let mut f = Fixture::quiet();
        // 2 involuntary switches over a 30 ms process is 66 per second: the
        // version-2 rule fails it.  Over the solve, one switch and one
        // 60 µs wake-up are within the floors.
        f.timing.involuntary_switches = 2;
        let solve = f.timing.solve.as_mut().unwrap();
        solve.involuntary_switches = Some(1);
        solve.schedstat_all = Some(SchedStat {
            cpu_ns: 0,
            run_delay_ns: 60_000,
            timeslices: 2,
        });
        f.timing.schedstat_solve = solve.schedstat_all;
        assert!(has(
            &grade_v2(&f.input()).blockers,
            "preemptions per second"
        ));
        assert!(has(&grade_v2(&f.input()).blockers, "run-queue delay"));
        assert!(f.blockers("L2").is_empty(), "{:?}", f.blockers("L2"));
        // Several switches, or a long wait, still fail it.
        let solve = f.timing.solve.as_mut().unwrap();
        solve.involuntary_switches = Some(4);
        assert!(has(&f.blockers("L2"), "involuntary context switches"));
        let solve = f.timing.solve.as_mut().unwrap();
        solve.involuntary_switches = Some(0);
        solve.schedstat_all = Some(SchedStat {
            cpu_ns: 0,
            run_delay_ns: 400_000,
            timeslices: 2,
        });
        assert!(has(&f.blockers("L2"), "run-queue delay"));
    }

    #[test]
    fn the_allowance_scales_with_the_solve() {
        let mut f = Fixture::quiet();
        // A 2 s solve may take 100 involuntary switches at 50 per second.
        f.timing.solve_wall_ns = Some(2_000_000_000);
        f.timing.process_wall_ns = 2_030_000_000;
        f.timing.user_ns = 2_000_000_000;
        f.timing.sys_ns = 10_000_000;
        f.timing.solve.as_mut().unwrap().involuntary_switches = Some(100);
        f.timing.solve.as_mut().unwrap().irq = Some(IrqCounts::default());
        assert!(
            !has(&f.blockers("L2"), "involuntary"),
            "{:?}",
            f.blockers("L2")
        );
        f.timing.solve.as_mut().unwrap().involuntary_switches = Some(101);
        assert!(has(&f.blockers("L2"), "involuntary"));
    }

    #[test]
    fn a_solve_below_the_minimum_is_too_short_to_admit_wall_time() {
        let mut f = Fixture::quiet();
        f.timing.solve_wall_ns = Some(130_000);
        let g = grade_v3(&f.input());
        assert!(has(&g.blockers, TOO_SHORT_TO_ADMIT), "{:?}", g.blockers);
        assert_eq!(g.level, Level::L1);
        // At the minimum it is admitted.
        f.timing.solve_wall_ns = Some(MIN_SOLVE_WALL_NS_V3);
        assert!(!has(&f.blockers("L2"), TOO_SHORT_TO_ADMIT));
        // Version 2 never had the check.
        f.timing.solve_wall_ns = Some(130_000);
        assert!(!has(&grade_v2(&f.input()).blockers, "too short"));
    }

    #[test]
    fn loosened_preflight_thresholds_are_named() {
        let mut f = Fixture::quiet();
        assert!(!has(&f.blockers("L2"), "looser than the defaults"));
        f.thresholds.settle_seconds = 0.0;
        f.thresholds.max_psi = 90.0;
        f.thresholds.max_other_cpu = 0.5;
        let b = f.blockers("L2");
        assert!(has(&b, "settle 0 s < 2 s"), "{b:?}");
        assert!(has(&b, "max-psi 90 > 5"), "{b:?}");
        assert!(has(&b, "max-other-cpu 0.5 > 0.1"), "{b:?}");
        // Tighter than the defaults is not a loosening.
        let mut f = Fixture::quiet();
        f.thresholds.settle_seconds = 5.0;
        f.thresholds.max_psi = 1.0;
        assert!(!has(&f.blockers("L2"), "looser than the defaults"));
    }

    #[test]
    fn threads_are_counted_not_inferred_from_cpu_time() {
        let mut f = Fixture::quiet();
        f.timing.solve.as_mut().unwrap().threads_after = Some(3);
        let b = f.blockers("L1");
        assert!(has(&b, "ran 3 threads"), "{b:?}");
        // A pinned child's CPU time cannot exceed its wall time, so the
        // version-2 rule cannot see the extra threads.
        assert!(!has(&grade_v2(&f.input()).blockers, "more than one thread"));
        // No report of the count is a blocker too.
        let mut f = Fixture::quiet();
        f.timing.solve = None;
        assert!(has(&f.blockers("L1"), "thread count"));
    }

    #[test]
    fn intruders_beyond_the_known_unmovable_ones_cost_l1() {
        let mut f = Fixture::quiet();
        f.after.intruders = Some(IntruderScan {
            user_threads: 2,
            kernel_threads: 5,
            examples: vec![],
        });
        assert!(has(
            &f.blockers("L1"),
            "2 user threads other than the runner's"
        ));
        // Two the eviction already failed to move are one blocker, not two.
        f.eviction.failed_user_threads = 2;
        assert!(!has(&f.blockers("L1"), "other than the runner's"));
        assert!(has(&f.blockers("L1"), "could not be moved"));
        // No scan at all is named.
        let mut f = Fixture::quiet();
        f.before.intruders = None;
        f.after.intruders = None;
        assert!(has(&f.blockers("L1"), "no scan of the reservation"));
    }

    #[test]
    fn interrupts_and_foreign_slices_on_the_run_cpu_are_gated_over_the_solve() {
        let mut f = Fixture::quiet();
        // The scheduler tick is not an intrusion.
        f.timing.solve.as_mut().unwrap().irq = Some(IrqCounts {
            local_timer: 40,
            softirq_tick: 40,
            ..Default::default()
        });
        assert!(f.blockers("L2").is_empty(), "{:?}", f.blockers("L2"));
        f.timing.solve.as_mut().unwrap().irq = Some(IrqCounts {
            device: 7,
            ipi_other: 3,
            ..Default::default()
        });
        assert!(has(&f.blockers("L2"), "10 interrupts on the run CPU"));
        // A single foreign wake-up is within the floor; two slices, or a
        // long one, are not.
        let mut f = Fixture::quiet();
        f.timing.solve.as_mut().unwrap().cpu_sched = Some(CpuSched {
            sched_count: 2,
            ttwu_count: 1,
            rq_cpu_ns: 30_000,
            run_delay_ns: 0,
            pcount: 2,
        });
        // pcount 2, the child's own timeslices 1: one foreign arrival.
        assert!(f.blockers("L2").is_empty(), "{:?}", f.blockers("L2"));
        f.timing.solve.as_mut().unwrap().cpu_sched = Some(CpuSched {
            pcount: 3,
            rq_cpu_ns: 30_000,
            ..Default::default()
        });
        assert!(has(&f.blockers("L2"), "foreign task slices"));
        f.timing.solve.as_mut().unwrap().cpu_sched = Some(CpuSched {
            pcount: 2,
            rq_cpu_ns: 900_000,
            ..Default::default()
        });
        assert!(has(&f.blockers("L2"), "foreign task slices"));
    }

    #[test]
    fn one_steal_tick_in_a_short_run_is_within_the_resolution() {
        let mut f = Fixture::quiet();
        f.after.reserved_ticks.get_mut(&3).unwrap().steal = 1;
        assert!(!has(&f.blockers("L2"), "steal"));
        assert!(has(&grade_v2(&f.input()).blockers, "steal"));
        f.after.reserved_ticks.get_mut(&3).unwrap().steal = 3;
        assert!(has(&f.blockers("L2"), "steal"));
    }

    #[test]
    fn grading_dispatches_on_the_recorded_version() {
        let mut f = Fixture::quiet();
        f.timing.involuntary_switches = 2;
        f.timing.solve.as_mut().unwrap().involuntary_switches = Some(1);
        let i = f.input();
        assert_eq!(GRADING_VERSION, 3);
        let v2 = grade_for_version(2, &i).unwrap();
        let v3 = grade_for_version(3, &i).unwrap();
        assert_eq!(v2.blockers, grade_v2(&i).blockers);
        assert_eq!(v3.blockers, grade_v3(&i).blockers);
        assert_eq!(grade(&i).blockers, v3.blockers);
        assert_ne!(v2.blockers, v3.blockers);
        assert!(grade_for_version(1, &i).is_none());
        assert!(grade_for_version(4, &i).is_none());
        assert!(grading_version_supported(2) && grading_version_supported(3));
        assert!(!grading_version_supported(1) && !grading_version_supported(4));
    }
}
