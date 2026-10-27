//! Per-phase CPU timers for the `gaudry-d4-bin` index-calculus pipeline
//! (`ecbench unit-price`, CPU_GAP.md).
//!
//! A phase is entered with [`enter`] (restored when the guard drops) or
//! switched with [`set`]; the time since the last switch is charged to the
//! phase that was current, so phases are exclusive and nest like a stack.
//! The clock is the calling thread's CPU time (`CLOCK_THREAD_CPUTIME_ID`,
//! nanosecond resolution, user plus system), and the thread's minor page
//! faults (`getrusage(RUSAGE_THREAD)`, exact) are charged the same way:
//! the kernel's user/system split of a thread is tick-sampled and too coarse
//! per phase, so system time is estimated from faults by the caller.
//!
//! Off unless [`start`] was called, and then meaningful only when the whole
//! solve runs on the calling thread (`unit_price::on_one_thread`): a rayon
//! worker has its own thread clock.  Timers read clocks and count nothing
//! the solver counts, so every operation count is unchanged.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum Phase {
    /// The relation loop outside the oracle: the walk, target draws, and
    /// the pipeline's own bookkeeping.
    Loop = 0,
    /// `TLineBase::build` inside the pipeline.
    FactorBase,
    /// The second `TLineBase::build`, run after the pipeline only to
    /// describe the factor base (`solve_ic_f2m4_native`).
    FactorBaseFacts,
    /// `BinD4Oracle::prepare`: the symmetrised precomputation.
    Precompute,
    /// `decompose` outside the solver: seeding, counters, lookups.
    OracleOther,
    /// The Weil-restriction system build (`t_of_x`, `system_*`).
    WeilBuild,
    /// Macaulay matrix: monomials, triangularisation, rows.
    MacaulayBuild,
    /// Echelon form (and the shape-replay top-up).
    Echelon,
    /// Normal forms of the border monomials, chains.
    NormalForm,
    /// Evaluations of det A and the interpolation of its polynomial.
    Charpoly,
    /// Roots of that polynomial over F_q.
    Roots,
    /// Eigenvectors, eigenspace splits, verification of candidates.
    Eigen,
    /// Roots of the quartic, multisets, lifts, sign checks.
    SplitLift,
    /// The relation matrix over Z/r: `add_row` and the pinned check.
    Linalg,
}

pub const N_PHASES: usize = 14;
pub const NAMES: [&str; N_PHASES] = [
    "loop",
    "factor_base",
    "factor_base_facts_rebuild",
    "precompute",
    "oracle_other",
    "weil_build",
    "macaulay_build",
    "echelon",
    "normal_form",
    "charpoly",
    "roots",
    "eigen",
    "split_lift",
    "linalg",
];

static ON: AtomicBool = AtomicBool::new(false);
static CPU_NS: [AtomicU64; N_PHASES] = [const { AtomicU64::new(0) }; N_PHASES];
static FAULTS: [AtomicU64; N_PHASES] = [const { AtomicU64::new(0) }; N_PHASES];
static ENTRIES: [AtomicU64; N_PHASES] = [const { AtomicU64::new(0) }; N_PHASES];

#[derive(Clone, Copy)]
struct State {
    cur: usize,
    cpu: u64,
    faults: u64,
}

thread_local! {
    static ST: Cell<Option<State>> = const { Cell::new(None) };
}

fn thread_cpu_ns() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) };
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn thread_faults() -> u64 {
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_THREAD, &mut ru) };
    ru.ru_minflt as u64 + ru.ru_majflt as u64
}

/// Charge the time since the last switch to the current phase and move to
/// `to`.  Returns the phase left.
fn switch(to: usize) -> Option<usize> {
    let st = ST.with(|s| s.get())?;
    let (cpu, faults) = (thread_cpu_ns(), thread_faults());
    CPU_NS[st.cur].fetch_add(cpu.saturating_sub(st.cpu), Ordering::Relaxed);
    FAULTS[st.cur].fetch_add(faults.saturating_sub(st.faults), Ordering::Relaxed);
    if to != st.cur {
        ENTRIES[to].fetch_add(1, Ordering::Relaxed);
    }
    ST.with(|s| {
        s.set(Some(State {
            cur: to,
            cpu,
            faults,
        }))
    });
    Some(st.cur)
}

/// Whether the timers are running.
pub fn is_on() -> bool {
    ON.load(Ordering::Relaxed)
}

/// Zero the totals and start timing on this thread, in [`Phase::Loop`].
pub fn start() {
    for i in 0..N_PHASES {
        CPU_NS[i].store(0, Ordering::Relaxed);
        FAULTS[i].store(0, Ordering::Relaxed);
        ENTRIES[i].store(0, Ordering::Relaxed);
    }
    ST.with(|s| {
        s.set(Some(State {
            cur: Phase::Loop as usize,
            cpu: thread_cpu_ns(),
            faults: thread_faults(),
        }))
    });
    ON.store(true, Ordering::SeqCst);
}

/// One phase's totals: thread CPU ns (user plus system), minor faults and
/// the number of times it was entered.
#[derive(Clone, Copy, Debug, Default)]
pub struct Total {
    pub cpu_ns: u64,
    pub faults: u64,
    pub entries: u64,
}

/// Stop timing and return the totals per phase, in [`NAMES`] order.
pub fn finish() -> [Total; N_PHASES] {
    if ON.swap(false, Ordering::SeqCst) {
        switch(Phase::Loop as usize);
        ST.with(|s| s.set(None));
    }
    std::array::from_fn(|i| Total {
        cpu_ns: CPU_NS[i].load(Ordering::Relaxed),
        faults: FAULTS[i].load(Ordering::Relaxed),
        entries: ENTRIES[i].load(Ordering::Relaxed),
    })
}

/// Restores the phase that was current at [`enter`].
pub struct Guard(Option<usize>);

impl Drop for Guard {
    fn drop(&mut self) {
        if let Some(prev) = self.0 {
            switch(prev);
        }
    }
}

/// Make `p` current until the guard drops.
pub fn enter(p: Phase) -> Guard {
    if !ON.load(Ordering::Relaxed) {
        return Guard(None);
    }
    Guard(switch(p as usize))
}

/// Like [`enter`], but only from [`Phase::Loop`]: a build that the caller
/// already filed under another phase keeps that phase.
pub fn enter_in_loop(p: Phase) -> Guard {
    if !ON.load(Ordering::Relaxed)
        || ST.with(|s| s.get()).map(|s| s.cur) != Some(Phase::Loop as usize)
    {
        return Guard(None);
    }
    Guard(switch(p as usize))
}

/// Make `p` current without a guard (the enclosing guard still restores
/// its own entry phase).
pub fn set(p: Phase) {
    if ON.load(Ordering::Relaxed) {
        switch(p as usize);
    }
}

/// The timers are process-global: tests that switch them on take this lock.
#[cfg(test)]
pub static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;

    fn spin(ms: u64) {
        let t = std::time::Instant::now();
        let mut x = 1u64;
        while t.elapsed().as_millis() < ms as u128 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            std::hint::black_box(x);
        }
    }

    #[test]
    fn phases_are_exclusive_and_sum_to_the_window() {
        let _lock = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let t0 = thread_cpu_ns();
        start();
        spin(20);
        {
            let _g = enter(Phase::Echelon);
            spin(30);
            {
                let _h = enter(Phase::Roots);
                spin(10);
            }
            spin(10);
        }
        set(Phase::Linalg);
        spin(5);
        let tot = finish();
        let window = (thread_cpu_ns() - t0) as f64;
        let sum: u64 = tot.iter().map(|t| t.cpu_ns).sum();
        assert!((sum as f64) <= window * 1.001);
        assert!((sum as f64) >= window * 0.9, "sum {sum} window {window}");
        let ms = |p: Phase| tot[p as usize].cpu_ns as f64 / 1e6;
        assert!(
            (ms(Phase::Echelon) - 40.0).abs() < 8.0,
            "{}",
            ms(Phase::Echelon)
        );
        assert!((ms(Phase::Roots) - 10.0).abs() < 5.0);
        assert!((ms(Phase::Loop) - 20.0).abs() < 8.0);
        assert!(ms(Phase::Linalg) > 3.0);
        // Off again: entering is inert.
        let _g = enter(Phase::Echelon);
    }
}
