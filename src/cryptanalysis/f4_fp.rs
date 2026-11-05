//! Degree-bounded F4 over a small prime field `F_p`.
//!
//! The coordinate thread (`research/notes/index-calculus/RESEARCH_EXOTIC_COORDINATES.md` §12.4, §13.5)
//! ended on systems the repo's Buchberger (`groebner_f4::buchberger`, a
//! textbook Buchberger over `BigUint` field elements) could not finish:
//! three or four unknowns over `F₂₉` or `F₃₁`, equations of total degree
//! 4 to 12 with up to 125 terms.  This module is the tool that row of the
//! table asked for.
//!
//! ## What it is
//!
//! Faugère's F4 with the normal (degree-by-degree) selection strategy and
//! a **degree bound** `D`: critical pairs whose lcm has degree above `D`
//! are never formed, so the result is the degree-`D` truncation of the
//! Gröbner basis — enough to decide consistency and, for a zero-
//! dimensional ideal, to solve, whenever the solving degree is at most
//! `D`.  Each step builds one matrix by symbolic preprocessing (the pair
//! multiples plus one reducer per reducible monomial) and keeps the rows
//! of its reduced echelon form whose leading monomial is new.  The
//! reducers already have distinct leading monomials, so the step reduces
//! each (sparse) pair row by them alone and takes the dense reduced
//! echelon form of only what is left, on the columns no reducer leads —
//! the same rows the whole matrix's echelon form would give, from a much
//! smaller dense block.  Rows run in parallel once a step is large enough
//! to repay the dispatch. Gebauer–Möller criteria B, M and F supplement
//! the product criterion when pruning pairs;
//! polynomials whose leading monomial becomes divisible by a new one are
//! dropped.
//!
//! Solving ([`solve`]) is by substitution: after a basis at degree `D`,
//! a univariate element (when the basis has one) is factored by root
//! finding over `F_p`, each root substituted, and the smaller system run
//! again; without a univariate element every value of the first variable
//! is tried (`p` small).  Each run reports the degree at which the
//! system resolved — the number to compare across coordinate systems —
//! and stops cooperatively at a deadline, so a budget never leaves a
//! thread behind.
//! An opt-in diagnostic policy (`F4_FP_LINEAR_ROOTS=1`, default off) first
//! substitutes a pure univariate linear equation analytically. Other root
//! finding and the final exact input check are unchanged.
//!
//! [`interreduce`] and [`autoreduce`] reduce every element of a set by the
//! others over and over; with at most 16 unknowns and every exponent at most
//! 127 they do it on packed monomials (one `u128` each), and hand whatever
//! does not pack to the exponent-vector code, which gives the same answer.
//! A step too small to repay a hand-off to the thread pool builds its sparse
//! rows on the calling thread; `solve`'s recursion runs many such steps.
//!
//! ## Honest scope
//!
//! Small primes (`p < 2³²`), a few unknowns, degrees in the tens: the
//! residual matrices are dense. Gebauer–Möller pruning is enabled by
//! default (`F4_FP_GM=0` retains the product criterion alone). There are
//! no F5 signatures or sugar selection; this module was written so that
//! the descent systems of the coordinate thread can be timed at `m = 3`.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};

use rayon::prelude::*;

use super::fx_hash::{FxMap, FxSet};
use super::groebner_f4::cmp_monomial;
pub use super::groebner_f4::Ordering;

/// A polynomial over `F_p`: `(exponent vector, coefficient)` terms with
/// non-zero coefficients, sorted with the leading term first.
pub type Poly = Vec<(Vec<u32>, u64)>;

/// Optional, read-only observations of the regular F4/solve path. Degrees
/// describe this run, never a certified degree of regularity.
#[derive(Clone, Debug, serde::Serialize)]
pub struct F4Progress {
    pub event: &'static str,
    pub variables: usize,
    pub step: usize,
    pub degree: u32,
    pub rows: usize,
    pub columns: usize,
    pub productive_degree: u32,
    pub field_multiplications: u64,
    pub elapsed_ms: f64,
}

thread_local! {
    static OBSERVER: std::cell::RefCell<Option<std::sync::mpsc::Sender<F4Progress>>> = const { std::cell::RefCell::new(None) };
}

/// Observe this thread's F4 calls, including substitution-tree calls. The
/// previous observer is restored on return or unwind; arithmetic is unchanged.
pub fn observe_progress<T>(
    sender: std::sync::mpsc::Sender<F4Progress>,
    f: impl FnOnce() -> T,
) -> T {
    struct Restore(Option<std::sync::mpsc::Sender<F4Progress>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            OBSERVER.with(|slot| {
                slot.replace(self.0.take());
            });
        }
    }
    let previous = OBSERVER.with(|slot| slot.replace(Some(sender)));
    let _restore = Restore(previous);
    f()
}

fn notify_progress(progress: F4Progress) {
    OBSERVER.with(|slot| {
        if let Some(sender) = slot.borrow().as_ref() {
            let _ = sender.send(progress);
        }
    });
}

// ── Field arithmetic ───────────────────────────────────────────────

/// Process-wide count of `F_p` multiplications spent in row reduction
/// (reducer applications, pivot normalisation and elimination).  Reports
/// carry the difference over their own run, so nested and parallel runs
/// stay additive; one atomic add per row operation keeps it off the hot
/// loops.
static FIELD_OPS: AtomicU64 = AtomicU64::new(0);

/// Process-wide count of native (non-multiplication) operations the F4 driver
/// does outside row reduction and that no field-multiplication counter prices:
/// leading-monomial divisibility tests (symbolic preprocessing and the staircase
/// checks of the stopping rules) and monomial-table probes of symbolic
/// preprocessing.  A diagnostic for AGENTS.md section 5a; incomplete by design (pair
/// selection, matrix assembly and sorting are not counted), so it is a lower bound.
static NATIVE_OPS: AtomicU64 = AtomicU64::new(0);
/// The share of [`native_ops_total`] spent on staircase / Hilbert-function checks.
static NATIVE_STAIR: AtomicU64 = AtomicU64::new(0);

pub fn native_ops_total() -> u64 {
    NATIVE_OPS.load(AtomicOrdering::Relaxed) + NATIVE_STAIR.load(AtomicOrdering::Relaxed)
}

/// Staircase and Hilbert-function checks only (inside [`native_ops_total`]).
pub fn native_stair_ops_total() -> u64 {
    NATIVE_STAIR.load(AtomicOrdering::Relaxed)
}

#[inline]
fn count_native(n: u64) {
    NATIVE_OPS.fetch_add(n, AtomicOrdering::Relaxed);
}

#[inline]
fn count_stair(n: u64) {
    NATIVE_STAIR.fetch_add(n, AtomicOrdering::Relaxed);
}

thread_local! {
    /// The same count, on the thread that did the work.
    static FIELD_OPS_THREAD: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// `F4_FP_THREAD_OPS=1`: [`field_ops_total`] and every report read this
/// thread's count instead of the process-wide one.  For a study that runs
/// several solves side by side, each confined to one thread (a one-thread
/// pool per run), so that no run's count takes in another's.
fn thread_ops_mode() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_THREAD_OPS").as_deref() == Ok("1"))
}

/// Per-step diagnostics of the most recent F4 run on this thread (taken with
/// [`take_run_log`]): they cost nothing in the counted multiplications.
#[derive(Clone, Debug, Default)]
pub struct StepStat {
    pub degree: u32,
    pub pairs: usize,
    /// Pair rows, and all rows of the step's matrix (pairs plus reducers).
    pub rows: usize,
    pub all_rows: usize,
    pub ops: u64,
    pub new_polys: usize,
    pub staircase: usize,
}
#[derive(Clone, Debug, Default)]
pub struct RunLog {
    pub steps: Vec<StepStat>,
    pub dropped_pairs: usize,
    pub violated: bool,
}
thread_local! {
    static RUN_LOG: std::cell::RefCell<RunLog> = std::cell::RefCell::new(RunLog::default());
}
pub fn take_run_log() -> RunLog {
    RUN_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}
/// Degree-`d` monomials in `n` variables divisible by no leading monomial.
fn uncovered_in_degree(lms: &[&[u32]], n: usize, d: u32) -> usize {
    fn rec(
        k: usize,
        left: u32,
        mono: &mut Vec<u32>,
        lms: &[&[u32]],
        cnt: &mut usize,
        tests: &mut u64,
    ) {
        if k + 1 == mono.len() {
            mono[k] = left;
            match lms.iter().position(|l| divides(l, mono)) {
                Some(i) => *tests += i as u64 + 1,
                None => {
                    *tests += lms.len() as u64;
                    *cnt += 1;
                }
            }
            return;
        }
        for e in 0..=left {
            mono[k] = e;
            rec(k + 1, left - e, mono, lms, cnt, tests);
        }
    }
    let mut cnt = 0;
    let mut tests = 0u64;
    let mut mono = vec![0u32; n];
    rec(0, d, &mut mono, lms, &mut cnt, &mut tests);
    count_stair(tests);
    cnt
}

/// The uncovered monomials of each degree (those divisible by no leading monomial
/// of the basis so far), kept up to date as basis elements are appended, for the
/// Hilbert stopping rule.  Coverage only grows: an element is killed (`alive`
/// false) only when a newer leading monomial divides its own, so the union over
/// all basis elements is the union over the alive ones.  The counts equal
/// [`uncovered_in_degree`] on the alive leading monomials (asserted in tests).
/// Work is charged to the native staircase counter: one unit per divisibility
/// test and per monomial generated.
struct StairTracker {
    n: usize,
    done: usize,
    /// Leading monomials by total degree: only those of degree `<= k` can divide a
    /// monomial of degree `k`.
    lms_by_deg: Vec<Vec<Vec<u32>>>,
    /// `u[k]`: uncovered monomials of degree `k`, for `k <= u.len() - 1`.
    u: Vec<Vec<Vec<u32>>>,
}

impl StairTracker {
    fn new(n: usize) -> Self {
        Self {
            n,
            done: 0,
            lms_by_deg: Vec::new(),
            u: vec![vec![vec![0u32; n]]],
        }
    }

    /// Take in the basis elements appended since the last call.
    fn absorb(&mut self, basis: &[Poly]) {
        let mut work = 0u64;
        while self.done < basis.len() {
            let l = basis[self.done][0].0.clone();
            let dl = total_degree(&l) as usize;
            for level in self.u.iter_mut().skip(dl) {
                work += level.len() as u64;
                level.retain(|m| !divides(&l, m));
            }
            if self.lms_by_deg.len() <= dl {
                self.lms_by_deg.resize(dl + 1, Vec::new());
            }
            self.lms_by_deg[dl].push(l);
            self.done += 1;
        }
        count_stair(work);
    }

    /// Build the levels up to degree `k`.
    fn ensure(&mut self, k: u32) {
        let mut work = 0u64;
        while self.u.len() <= k as usize {
            let prev = self.u.last().unwrap();
            let mut next: Vec<Vec<u32>> = Vec::new();
            for m in prev {
                let mx = m.iter().rposition(|&e| e > 0).unwrap_or(0);
                for i in mx..self.n {
                    let mut c = m.clone();
                    c[i] += 1;
                    work += 1;
                    let mut covered = false;
                    for bucket in self.lms_by_deg.iter().take(self.u.len() + 1) {
                        for l in bucket {
                            work += 1;
                            if divides(l, &c) {
                                covered = true;
                                break;
                            }
                        }
                        if covered {
                            break;
                        }
                    }
                    if !covered {
                        next.push(c);
                    }
                }
            }
            self.u.push(next);
        }
        count_stair(work);
    }

    #[allow(unused_variables)]
    fn count(&mut self, basis: &[Poly], alive: &[bool], k: u32) -> usize {
        self.absorb(basis);
        self.ensure(k);
        let c = self.u[k as usize].len();
        #[cfg(test)]
        {
            let lms: Vec<&[u32]> = basis
                .iter()
                .zip(alive)
                .filter(|(_, &a)| a)
                .map(|(f, _)| f[0].0.as_slice())
                .collect();
            assert_eq!(c, uncovered_in_degree(&lms, self.n, k), "tracker vs scan");
        }
        c
    }
}

fn stair_scan() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_STAIR").as_deref() == Ok("scan"))
}

fn field_ops_now() -> u64 {
    if thread_ops_mode() {
        FIELD_OPS_THREAD.with(|c| c.get())
    } else {
        FIELD_OPS.load(AtomicOrdering::Relaxed)
    }
}

/// The process-wide row-reduction counter, for callers that run several
/// F4 solves concurrently and want the batch's total as one difference
/// (per-report differences interleave under concurrency).
pub fn field_ops_total() -> u64 {
    field_ops_now()
}

#[inline]
fn count_ops(n: usize) {
    FIELD_OPS.fetch_add(n as u64, AtomicOrdering::Relaxed);
    FIELD_OPS_THREAD.with(|c| c.set(c.get() + n as u64));
    // bitop/v1 (`jv_bitops`, off unless a study turns it on): one row
    // operation is a multiply-subtract
    super::jv_bitops::row_ops(n as u64);
}

#[inline]
fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn invmod(a: u64, p: u64) -> u64 {
    super::jv_bitops::charge(super::jv_bitops::Kind::FieldInv, 1);
    // p prime, a != 0
    if (2..1 << 32).contains(&p) {
        // the same powers, each product reduced by Barrett's method
        // instead of a 128-bit division
        let fp = Barrett::new(p);
        let mut r = 1u64;
        let mut base = a % p;
        let mut e = p - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = fp.reduce(r * base);
            }
            base = fp.reduce(base * base);
            e >>= 1;
        }
        return r;
    }
    let mut r = 1u64;
    let mut base = a % p;
    let mut e = p - 2;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, base, p);
        }
        base = mulmod(base, base, p);
        e >>= 1;
    }
    r
}

/// Reduction modulo a fixed `p < 2³²` by Barrett's method: a precomputed
/// `⌊2⁶⁴/p⌋` turns each remainder in the eliminations into a high
/// multiply and at most one subtraction, instead of a hardware division
/// by a `p` known only at run time.
#[derive(Clone, Copy)]
struct Barrett {
    p: u64,
    m: u64,
}

impl Barrett {
    fn new(p: u64) -> Self {
        assert!(
            (2..1 << 32).contains(&p),
            "f4_fp: the prime must be below 2^32"
        );
        Barrett {
            p,
            m: (u128::from(u64::MAX) + 1).div_euclid(u128::from(p)) as u64,
        }
    }
    /// `x mod p` for any `x < 2⁶⁴`: `q = ⌊x·m/2⁶⁴⌋` undershoots `⌊x/p⌋`
    /// by at most one.
    #[inline(always)]
    fn reduce(self, x: u64) -> u64 {
        let q = ((u128::from(x) * u128::from(self.m)) >> 64) as u64;
        let r = x - q * self.p;
        if r >= self.p {
            r - self.p
        } else {
            r
        }
    }
}

/// Narrow reduction for `p <= 2^16`.  Every elimination update is at most
/// `(p - 1) + (p - 1)^2 = p(p - 1) < 2^32`, so both the matrix and the
/// multiply-add accumulator can stay in 32-bit lanes.
#[derive(Clone, Copy)]
struct Barrett32 {
    p: u32,
    m: u32,
}

impl Barrett32 {
    fn new(p: u64) -> Self {
        assert!((2..=1 << 16).contains(&p));
        Self {
            p: p as u32,
            m: ((1u64 << 32) / p) as u32,
        }
    }

    /// `x mod p` for the bounded product-plus-accumulator above.  The
    /// quotient estimate undershoots by at most one; the mask avoids a
    /// data-dependent branch in the row's inner loop.
    #[inline(always)]
    fn reduce(self, x: u32) -> u32 {
        let q = ((x as u64 * self.m as u64) >> 32) as u32;
        let r = x - q * self.p;
        let less = 0u32.wrapping_sub((r < self.p) as u32);
        r.wrapping_sub(self.p).wrapping_add(less & self.p)
    }
}

// ── Polynomial helpers ─────────────────────────────────────────────

fn total_degree(m: &[u32]) -> u32 {
    m.iter().sum()
}

/// Scale `f` (non-empty, coefficients below `p`) to be monic.
fn scale_monic(f: &mut Poly, p: u64) {
    let inv = invmod(f[0].1, p);
    super::jv_bitops::charge(super::jv_bitops::Kind::FieldMul, f.len() as u64);
    if (2..1 << 32).contains(&p) {
        let fp = Barrett::new(p);
        for (_, c) in f.iter_mut() {
            *c = fp.reduce(*c * inv);
        }
    } else {
        for (_, c) in f.iter_mut() {
            *c = mulmod(*c, inv, p);
        }
    }
}

/// Normalise: merge duplicate monomials, drop zeros, sort leading-first,
/// make monic.
pub fn normalise(terms: &[(Vec<u32>, u64)], p: u64, ord: Ordering) -> Poly {
    normalise_owned(
        terms.iter().filter(|(_, c)| c % p != 0).cloned().collect(),
        p,
        ord,
    )
}

/// [`normalise`] of terms the caller has no further use for, so that their
/// exponent vectors are moved, not cloned.  Sorting and merging replace the
/// ordered map the merge used to go through; the terms that come out are
/// the same.
fn normalise_owned(mut out: Poly, p: u64, ord: Ordering) -> Poly {
    out.retain_mut(|(_, c)| {
        *c %= p;
        *c != 0
    });
    // The sort need not keep equal monomials in their order: they are
    // adjacent either way, and merged by a sum that has no order.
    out.sort_unstable_by(|(a, _), (b, _)| cmp_monomial(b, a, ord));
    let mut merges = 0u64;
    let scanned = out.len() as u64;
    out.dedup_by(|later, kept| {
        let same = later.0 == kept.0;
        if same {
            kept.1 = (kept.1 + later.1) % p;
            merges += 1;
        }
        same
    });
    super::jv_bitops::words(super::jv_bitops::sort_cost(scanned as usize) + scanned);
    super::jv_bitops::charge(super::jv_bitops::Kind::FieldAdd, merges);
    // drop the sums that cancelled
    out.retain(|(_, c)| *c != 0);
    if !out.is_empty() {
        scale_monic(&mut out, p);
    }
    out
}

fn divides(a: &[u32], b: &[u32]) -> bool {
    a.iter().zip(b).all(|(x, y)| x <= y)
}

/// Render a polynomial with variable names.
pub fn render(f: &Poly, names: &[String], p: u64) -> String {
    if f.is_empty() {
        return "0".to_string();
    }
    f.iter()
        .map(|(e, c)| {
            let mono: Vec<String> = e
                .iter()
                .enumerate()
                .filter(|(_, &k)| k > 0)
                .map(|(i, &k)| {
                    if k == 1 {
                        names[i].clone()
                    } else {
                        format!("{}^{}", names[i], k)
                    }
                })
                .collect();
            let cs = if *c > p / 2 {
                format!("−{}", p - c)
            } else {
                c.to_string()
            };
            if mono.is_empty() {
                cs
            } else if *c == 1 {
                mono.join("·")
            } else {
                format!("{cs}·{}", mono.join("·"))
            }
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

// ── The algorithm ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct F4Options {
    pub order: Ordering,
    /// Critical pairs with lcm degree above this are never formed.
    pub max_degree: u32,
    /// Cooperative stop: the run returns `timed_out` once past it.
    pub deadline: Option<Instant>,
    /// Stop once the leading monomials of the basis so far leave at most
    /// this many standard monomials.
    ///
    /// The basis generates a subideal of the input ideal whose quotient then
    /// has at most that dimension. Every solution of the input is therefore
    /// among at most that many points, which can be enumerated and checked
    /// against the input directly. The pairs still pending would only
    /// certify the Gröbner basis, and are left unprocessed.
    ///
    /// Zero-dimensionality alone is not a stopping criterion. Field-like
    /// equations (`y² = …` for every variable) make the leading ideal
    /// zero-dimensional from the start, with an exponentially large
    /// staircase. `None` (the default) never stops early; a stopped run
    /// reports the staircase it stopped at in
    /// `F4Report::staircase_at_stop`.
    pub stop_staircase: Option<usize>,
    /// A `Z/3`-weight per variable.  When set, every step whose rows are
    /// all weight-homogeneous (each row's monomials in one class
    /// `Σ wᵢeᵢ (mod 3)`) reduces the three weight blocks of its matrix
    /// separately: the row spaces are direct summands, so the pivots and
    /// the reduced rows are those of the whole matrix.  A step with a
    /// mixed row falls back to the plain reduction.
    pub weights: Option<Vec<u8>>,
    /// Hilbert-driven stop (opt-in, `Some(chunk)`); see [`F4Options::stopping_by_hilbert`].
    pub stop_hilbert: Option<usize>,
    /// Set by the Hilbert guard once the input is certified: the standard
    /// monomials per degree of the leading-term ideal.
    hilbert_expect: Option<Vec<usize>>,
    /// Process at most this many pairs of the lowest degree per step (the
    /// rest wait, in increasing lcm).  `None`: all of them, one matrix per
    /// degree.  With [`F4Options::stop_staircase`] it lets the staircase
    /// check run between batches of one degree.
    pub max_pairs_per_step: Option<usize>,
    /// Collect per-step diagnostics for explicit research runs. Disabled by default.
    pub capture_run_log: bool,
    /// Write the run's state to this file (and resume from it if it already
    /// holds a checkpoint of this very run), at the top of a step once the
    /// given time has passed since the last write.  `None` (the default)
    /// leaves the run untouched.  See [`F4Options::checkpointing`].
    pub checkpoint: Option<(std::path::PathBuf, Duration)>,
    /// Test hook for [`F4Options::checkpointing`]: stop (reported as
    /// `timed_out`) right after writing the checkpoint of the first step
    /// whose number is at least this.
    pub checkpoint_stop_after: Option<usize>,
    /// With [`F4Options::checkpointing`]: also snapshot the dense row reduction
    /// of a step every this often, so a long step is not lost with its host.
    /// See [`F4Options::checkpointing_in_step`].
    pub checkpoint_in_step: Option<Duration>,
    /// Test hook for [`F4Options::checkpointing_in_step`]: stop (reported as
    /// `timed_out`) right after this many in-step snapshots.
    pub checkpoint_stop_in_rref: Option<usize>,
}

impl F4Options {
    pub fn new(order: Ordering, max_degree: u32) -> Self {
        F4Options {
            order,
            max_degree,
            deadline: None,
            stop_staircase: None,
            weights: None,
            stop_hilbert: None,
            hilbert_expect: None,
            max_pairs_per_step: None,
            capture_run_log: false,
            checkpoint: None,
            checkpoint_stop_after: None,
            checkpoint_in_step: None,
            checkpoint_stop_in_rref: None,
        }
    }
    /// Checkpoint the run to `path` and resume from it.
    ///
    /// At the top of a step, when `every` has passed since the last write, the
    /// state that survives a step (basis, liveness flags, pending and
    /// over-the-bound pairs, the step counters, the field operations and the
    /// time spent so far) is written atomically to `path`; the previous file
    /// is kept as `path.prev`.  A later call with the same input and options
    /// that finds a valid checkpoint continues from it: the counted field
    /// operations, the steps and the matrix sizes are those of an
    /// uninterrupted run, and the deadline counts the time already spent.  A
    /// checkpoint of another system is refused.  A step in flight when the
    /// process dies is redone.  The `ms` of the report is the run's total.
    /// Not combined with traces, the Hilbert or staircase stops, or the run
    /// log.
    pub fn checkpointing(mut self, path: std::path::PathBuf, every: Duration) -> Self {
        self.checkpoint = Some((path, every));
        self
    }
    /// Snapshot the dense row reduction inside a step.
    ///
    /// A step's long part is the echelon form of its dense matrix. Once `every`
    /// has passed since the last snapshot, at a column boundary of that
    /// reduction, the loop-top state of the step (written with the checkpoint
    /// file if it is not already there) and the partly reduced matrix with its
    /// pivots and position (`path.rref`) are written. A resumed run re-derives
    /// the step's matrix shape deterministically from the loop-top state,
    /// skips the reducer pass and the finished part of the reduction, and
    /// continues; its counted field operations are those of an uninterrupted
    /// run. Only the wide arithmetic (`p > 2^16`) without weights: elsewhere
    /// nothing extra is written. The snapshot is the size of the dense matrix
    /// (tens of GB for the large steps), so `every` should be long.
    pub fn checkpointing_in_step(mut self, every: Duration) -> Self {
        self.checkpoint_in_step = Some(every);
        self
    }
    /// Block-aware reduction by the given `Z/3`-weights.
    pub fn with_weights(mut self, weights: Vec<u8>) -> Self {
        self.weights = Some(weights);
        self
    }
    pub fn with_budget(mut self, budget: Duration) -> Self {
        self.deadline = Some(Instant::now() + budget);
        self
    }
    /// See [`F4Options::stop_staircase`].
    pub fn stopping_below(mut self, standard_monomials: usize) -> Self {
        self.stop_staircase = Some(standard_monomials);
        self
    }
    /// Hilbert-driven early stop, sound for square systems whose leading
    /// forms are a regular sequence (checked, with a fallback to the plain
    /// run when it fails).
    ///
    /// When `f_1..f_n` in `n` variables have leading forms `h_i` of degrees
    /// `d_i` forming a regular sequence, `LT(I) = LT((h))` has the standard
    /// monomials of the complete intersection `R/(h)`, with Hilbert series
    /// `prod (1-t^d_i)/(1-t)^n`.  The run then (a) takes at most `chunk`
    /// pairs of the lowest degree per step, in increasing lcm, and (b) drops
    /// every pending pair of degree `<= d` once the leading monomials found
    /// leave exactly the Hilbert function's standard monomials in every
    /// degree `<= d`: each such pair reduces to zero (README, Lemma 2).
    /// The regularity check is an F4 run on the leading forms; its
    /// multiplications, and those of a fallback run, are in `field_ops`.
    /// `F4Report::hilbert` says which path ran.  Not combined with traces.
    pub fn stopping_by_hilbert(mut self, chunk: usize) -> Self {
        self.stop_hilbert = Some(chunk.max(1));
        self
    }
    /// Record the next F4 run's per-step research diagnostics.
    pub fn capturing_run_log(mut self) -> Self {
        self.capture_run_log = true;
        self
    }
}

// ── Checkpoint and resume ──────────────────────────────────────────

/// The time a checkpoint at `path` (or its `.prev`) has already spent, or
/// `None` when there is none.  Only the header is read: the run itself
/// verifies the whole file before it resumes, and reports the time it
/// resumed with in `F4Report::resumed_ms`.
pub fn checkpoint_elapsed(path: &std::path::Path) -> Option<Duration> {
    ckpt::header_elapsed(path)
}

/// On-disk state of a run between two steps; see [`F4Options::checkpointing`].
mod ckpt {
    use super::*;
    use std::fs::File;
    use std::io::{self, BufReader, BufWriter, Read, Write};
    use std::path::{Path, PathBuf};

    const MAGIC: &[u8; 8] = b"F4CKPT01";

    #[inline]
    fn mix(h: u64, v: u64) -> u64 {
        (h ^ v)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(29)
            .wrapping_add(0x6A09_E667_F3BC_C909)
    }

    /// Writer that folds every value into a running hash.
    struct W<T: Write> {
        w: BufWriter<T>,
        h: u64,
    }
    impl<T: Write> W<T> {
        fn u64(&mut self, v: u64) -> io::Result<()> {
            self.h = mix(self.h, v);
            self.w.write_all(&v.to_le_bytes())
        }
        fn u32(&mut self, v: u32) -> io::Result<()> {
            self.h = mix(self.h, v as u64);
            self.w.write_all(&v.to_le_bytes())
        }
        fn poly(&mut self, f: &Poly) -> io::Result<()> {
            self.u64(f.len() as u64)?;
            for (e, c) in f {
                for &x in e {
                    self.u32(x)?;
                }
                self.u64(*c)?;
            }
            Ok(())
        }
        fn pairs(&mut self, ps: &[Pair]) -> io::Result<()> {
            self.u64(ps.len() as u64)?;
            for pr in ps {
                self.u64(pr.i as u64)?;
                self.u64(pr.j as u64)?;
                for &x in &pr.lcm {
                    self.u32(x)?;
                }
                self.u64(pr.lcm_p as u64)?;
                self.u64((pr.lcm_p >> 64) as u64)?;
                self.u32(pr.degree)?;
            }
            Ok(())
        }
    }

    struct R<T: Read> {
        r: BufReader<T>,
        h: u64,
    }
    impl<T: Read> R<T> {
        fn raw8(&mut self) -> io::Result<u64> {
            let mut b = [0u8; 8];
            self.r.read_exact(&mut b)?;
            Ok(u64::from_le_bytes(b))
        }
        fn u64(&mut self) -> io::Result<u64> {
            let v = self.raw8()?;
            self.h = mix(self.h, v);
            Ok(v)
        }
        fn u32(&mut self) -> io::Result<u32> {
            let mut b = [0u8; 4];
            self.r.read_exact(&mut b)?;
            let v = u32::from_le_bytes(b);
            self.h = mix(self.h, v as u64);
            Ok(v)
        }
        /// A length, bounded so a corrupt file cannot ask for absurd memory.
        fn len(&mut self) -> io::Result<usize> {
            let v = self.u64()?;
            if v > 1 << 40 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "absurd length"));
            }
            Ok(v as usize)
        }
        fn poly(&mut self, n: usize) -> io::Result<Poly> {
            let len = self.len()?;
            let mut f = Poly::with_capacity(len.min(1 << 20));
            for _ in 0..len {
                let mut e = Vec::with_capacity(n);
                for _ in 0..n {
                    e.push(self.u32()?);
                }
                f.push((e, self.u64()?));
            }
            Ok(f)
        }
        fn pairs(&mut self, n: usize) -> io::Result<Vec<Pair>> {
            let len = self.len()?;
            let mut ps = Vec::with_capacity(len.min(1 << 20));
            for _ in 0..len {
                let i = self.u64()? as usize;
                let j = self.u64()? as usize;
                let mut lcm = Vec::with_capacity(n);
                for _ in 0..n {
                    lcm.push(self.u32()?);
                }
                let lo = self.u64()? as u128;
                let hi = self.u64()? as u128;
                ps.push(Pair {
                    i,
                    j,
                    lcm,
                    lcm_p: lo | hi << 64,
                    degree: self.u32()?,
                });
            }
            Ok(ps)
        }
    }

    pub(super) struct Snap<'a> {
        pub fingerprint: u64,
        pub n_vars: usize,
        pub elapsed: Duration,
        pub field_ops: u64,
        pub basis: &'a [Poly],
        pub alive: &'a [bool],
        pub pairs: &'a [Pair],
        pub above: &'a [Pair],
        pub pairs_above_bound: usize,
        pub steps: usize,
        pub max_rows: usize,
        pub max_cols: usize,
        pub degree_reached: u32,
        pub solving_degree: u32,
        pub learned: (u32, usize, usize),
        pub blocked_steps: usize,
    }

    pub(super) struct Loaded {
        pub elapsed: Duration,
        pub field_ops: u64,
        pub basis: Vec<Poly>,
        pub alive: Vec<bool>,
        pub pairs: Vec<Pair>,
        pub above: Vec<Pair>,
        pub pairs_above_bound: usize,
        pub steps: usize,
        pub max_rows: usize,
        pub max_cols: usize,
        pub degree_reached: u32,
        pub solving_degree: u32,
        pub learned: (u32, usize, usize),
        pub blocked_steps: usize,
    }

    pub(super) fn rref_path(path: &Path) -> PathBuf {
        sibling(path, ".rref")
    }

    const RREF_MAGIC: &[u8; 8] = b"F4RREF01";

    /// The dense reduction of one step, partly done.
    pub(super) struct RrefSnap<'a> {
        pub fingerprint: u64,
        /// The loop-top checkpoint this belongs to: its step counter and elapsed time.
        pub top_steps: usize,
        pub top_elapsed: Duration,
        pub ops: u64,
        pub elapsed: Duration,
        pub phase: u32,
        pub c: usize,
        pub pr: usize,
        pub i: usize,
        pub pivots: &'a [usize],
        pub rows: &'a [Vec<u64>],
    }

    pub(super) struct RrefLoaded {
        pub ops: u64,
        pub elapsed: Duration,
        pub phase: u32,
        pub c: usize,
        pub pr: usize,
        pub i: usize,
        pub pivots: Vec<usize>,
        pub rows: Vec<Vec<u64>>,
    }

    pub(super) fn save_rref(path: &Path, s: &RrefSnap) -> io::Result<u64> {
        let tmp = sibling(path, ".tmp");
        let mut w = W {
            w: BufWriter::with_capacity(1 << 20, File::create(&tmp)?),
            h: 0,
        };
        w.w.write_all(RREF_MAGIC)?;
        w.u64(s.fingerprint)?;
        w.u64(s.top_steps as u64)?;
        w.u64(s.top_elapsed.as_nanos() as u64)?;
        w.u64(s.ops)?;
        w.u64(s.elapsed.as_nanos() as u64)?;
        w.u32(s.phase)?;
        w.u64(s.c as u64)?;
        w.u64(s.pr as u64)?;
        w.u64(s.i as u64)?;
        w.u64(s.pivots.len() as u64)?;
        for &x in s.pivots {
            w.u64(x as u64)?;
        }
        let n_cols = s.rows.first().map_or(0, |r| r.len());
        w.u64(s.rows.len() as u64)?;
        w.u64(n_cols as u64)?;
        for row in s.rows {
            debug_assert_eq!(row.len(), n_cols);
            for &x in row {
                w.u64(x)?;
            }
        }
        let h = w.h;
        w.w.write_all(&h.to_le_bytes())?;
        w.w.flush()?;
        let file = w.w.into_inner().map_err(|e| e.into_error())?;
        file.sync_all()?;
        let bytes = file.metadata()?.len();
        drop(file);
        std::fs::rename(&tmp, path)?;
        Ok(bytes)
    }

    /// The snapshot at `path` if it is a valid one of this run for the step
    /// that follows the loop-top checkpoint `(top_steps, top_elapsed)`.
    pub(super) fn load_rref(
        path: &Path,
        fingerprint: u64,
        top_steps: usize,
        top_elapsed: Duration,
    ) -> Option<RrefLoaded> {
        if !path.exists() {
            return None;
        }
        let read = || -> io::Result<RrefLoaded> {
            let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, m.to_string());
            let mut r = R {
                r: BufReader::with_capacity(1 << 20, File::open(path)?),
                h: 0,
            };
            let mut magic = [0u8; 8];
            r.r.read_exact(&mut magic)?;
            if &magic != RREF_MAGIC {
                return Err(bad("not an in-step snapshot"));
            }
            if r.u64()? != fingerprint {
                return Err(bad("snapshot of another system or options"));
            }
            if r.u64()? as usize != top_steps || r.u64()? != top_elapsed.as_nanos() as u64 {
                return Err(bad("snapshot of another step than the checkpoint's"));
            }
            let ops = r.u64()?;
            let elapsed = Duration::from_nanos(r.u64()?);
            let phase = r.u32()?;
            let c = r.u64()? as usize;
            let pr = r.u64()? as usize;
            let i = r.u64()? as usize;
            let np = r.len()?;
            let mut pivots = Vec::with_capacity(np.min(1 << 20));
            for _ in 0..np {
                pivots.push(r.u64()? as usize);
            }
            let n_rows = r.len()?;
            let n_cols = r.len()?;
            let mut rows = Vec::with_capacity(n_rows.min(1 << 20));
            for _ in 0..n_rows {
                let mut row = Vec::with_capacity(n_cols);
                for _ in 0..n_cols {
                    row.push(r.u64()?);
                }
                rows.push(row);
            }
            let h = r.h;
            if r.raw8()? != h {
                return Err(bad("checksum mismatch"));
            }
            let mut extra = [0u8; 1];
            if r.r.read(&mut extra)? != 0 {
                return Err(bad("trailing bytes"));
            }
            Ok(RrefLoaded {
                ops,
                elapsed,
                phase,
                c,
                pr,
                i,
                pivots,
                rows,
            })
        };
        match read() {
            Ok(l) => Some(l),
            Err(e) => {
                eprintln!("f4: in-step snapshot {} refused: {e}", path.display());
                None
            }
        }
    }

    fn sibling(path: &Path, ext: &str) -> PathBuf {
        let mut s = path.as_os_str().to_owned();
        s.push(ext);
        PathBuf::from(s)
    }

    /// Identity of a run: the system, the field and every option that
    /// changes the steps.  A checkpoint of another identity is refused.
    pub(super) fn fingerprint(
        input: &[Poly],
        n_vars: usize,
        p: u64,
        opts: &F4Options,
        mode: u8,
        gm: bool,
        packed: bool,
    ) -> u64 {
        let mut h = 0x4634_434B_5054u64;
        for v in [
            n_vars as u64,
            p,
            opts.max_degree as u64,
            mode as u64,
            gm as u64,
            packed as u64,
        ] {
            h = mix(h, v);
        }
        for b in format!(
            "{:?}|{:?}|{:?}",
            opts.order, opts.max_pairs_per_step, opts.weights
        )
        .bytes()
        {
            h = mix(h, b as u64);
        }
        h = mix(h, input.len() as u64);
        for f in input {
            h = mix(h, f.len() as u64);
            for (e, c) in f {
                for &x in e {
                    h = mix(h, x as u64);
                }
                h = mix(h, *c);
            }
        }
        h
    }

    /// Write atomically: a temporary file, synced, then renamed over `path`
    /// (the file it replaces becomes `path.prev`).  Returns the size.
    pub(super) fn save(path: &Path, s: &Snap) -> io::Result<u64> {
        let tmp = sibling(path, ".tmp");
        let mut w = W {
            w: BufWriter::with_capacity(1 << 20, File::create(&tmp)?),
            h: 0,
        };
        w.w.write_all(MAGIC)?;
        w.u64(s.fingerprint)?;
        w.u64(s.n_vars as u64)?;
        w.u64(s.elapsed.as_nanos() as u64)?;
        w.u64(s.field_ops)?;
        w.u64(s.steps as u64)?;
        w.u64(s.max_rows as u64)?;
        w.u64(s.max_cols as u64)?;
        w.u32(s.degree_reached)?;
        w.u32(s.solving_degree)?;
        w.u32(s.learned.0)?;
        w.u64(s.learned.1 as u64)?;
        w.u64(s.learned.2 as u64)?;
        w.u64(s.pairs_above_bound as u64)?;
        w.u64(s.blocked_steps as u64)?;
        w.u64(s.basis.len() as u64)?;
        for (f, &a) in s.basis.iter().zip(s.alive) {
            w.u32(a as u32)?;
            w.poly(f)?;
        }
        w.pairs(s.pairs)?;
        w.pairs(s.above)?;
        let h = w.h;
        w.w.write_all(&h.to_le_bytes())?;
        w.w.flush()?;
        let file = w.w.into_inner().map_err(|e| e.into_error())?;
        file.sync_all()?;
        let bytes = file.metadata()?.len();
        drop(file);
        if path.exists() {
            std::fs::rename(path, sibling(path, ".prev"))?;
        }
        std::fs::rename(&tmp, path)?;
        Ok(bytes)
    }

    fn read_one(path: &Path, want: Option<u64>) -> io::Result<Loaded> {
        let bad = |m: &str| io::Error::new(io::ErrorKind::InvalidData, m.to_string());
        let mut r = R {
            r: BufReader::with_capacity(1 << 20, File::open(path)?),
            h: 0,
        };
        let mut magic = [0u8; 8];
        r.r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(bad("not a checkpoint"));
        }
        let fp = r.u64()?;
        if want.is_some_and(|w| w != fp) {
            return Err(bad("checkpoint of another system or options"));
        }
        let n = r.u64()? as usize;
        if n == 0 || n > 4096 {
            return Err(bad("absurd variable count"));
        }
        let elapsed = Duration::from_nanos(r.u64()?);
        let field_ops = r.u64()?;
        let steps = r.u64()? as usize;
        let max_rows = r.u64()? as usize;
        let max_cols = r.u64()? as usize;
        let degree_reached = r.u32()?;
        let solving_degree = r.u32()?;
        let l0 = r.u32()?;
        let l1 = r.u64()? as usize;
        let l2 = r.u64()? as usize;
        let pairs_above_bound = r.u64()? as usize;
        let blocked_steps = r.u64()? as usize;
        let nb = r.len()?;
        let mut basis = Vec::with_capacity(nb.min(1 << 20));
        let mut alive = Vec::with_capacity(nb.min(1 << 20));
        for _ in 0..nb {
            alive.push(r.u32()? != 0);
            basis.push(r.poly(n)?);
        }
        let pairs = r.pairs(n)?;
        let above = r.pairs(n)?;
        let h = r.h;
        if r.raw8()? != h {
            return Err(bad("checksum mismatch"));
        }
        let mut extra = [0u8; 1];
        if r.r.read(&mut extra)? != 0 {
            return Err(bad("trailing bytes"));
        }
        Ok(Loaded {
            elapsed,
            field_ops,
            basis,
            alive,
            pairs,
            above,
            pairs_above_bound,
            steps,
            max_rows,
            max_cols,
            degree_reached,
            solving_degree,
            learned: (l0, l1, l2),
            blocked_steps,
        })
    }

    /// The elapsed time in the header of `path`, else of `path.prev`.
    pub(super) fn header_elapsed(path: &Path) -> Option<Duration> {
        for p in [path.to_path_buf(), sibling(path, ".prev")] {
            let Ok(f) = File::open(&p) else { continue };
            let mut r = R {
                r: BufReader::new(f),
                h: 0,
            };
            let mut magic = [0u8; 8];
            if r.r.read_exact(&mut magic).is_err() || &magic != MAGIC {
                continue;
            }
            if let (Ok(_fp), Ok(_n), Ok(ns)) = (r.u64(), r.u64(), r.u64()) {
                return Some(Duration::from_nanos(ns));
            }
        }
        None
    }

    /// The newest valid checkpoint: `path`, else `path.prev`.  A file that
    /// exists but fails to read is reported on stderr, not skipped silently.
    pub(super) fn load_best(path: &Path, want: Option<u64>) -> Option<Loaded> {
        for p in [path.to_path_buf(), sibling(path, ".prev")] {
            if !p.exists() {
                continue;
            }
            match read_one(&p, want) {
                Ok(l) => return Some(l),
                Err(e) => eprintln!("f4: checkpoint {} refused: {e}", p.display()),
            }
        }
        None
    }
}

/// One step of a recorded run: the pair rows that did not reduce to zero
/// by the reducers alone (as `(basis index, multiplier)`), and the leading
/// monomials of the polynomials the step added.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct TraceStep {
    pub degree: u32,
    pub rows: Vec<(usize, Vec<u32>)>,
    pub new_lms: Vec<Vec<u32>>,
}

/// The trace of one run (Joux–Vitse's "F4 remake": the useful rows of
/// every step, to be replayed on later systems of the same shape, skipping
/// the pair selection and every row that reduced to zero).  A replay
/// checks at every step that the leading monomials it finds are the
/// recorded ones and gives up otherwise (`F4Report::trace_mismatch`).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct F4Trace {
    /// Leading monomials of the normalised input, in order.
    pub input_lms: Vec<Vec<u32>>,
    pub steps: Vec<TraceStep>,
    /// The recorded run stopped at the staircase (`stop_staircase`).
    pub stopped: bool,
}

/// Gauss–Jordan over `F_p` on dense `rows` (over the free columns), tracking
/// for every pivot row which input rows it is a combination of.  Returns
/// `(pivot column, contributing input rows)` per pivot.  Used once per `p`
/// when a trace is recorded; its cost is not part of the replays.
fn tracked_rref_contributors(rows: &[Vec<u64>], p: u64) -> Vec<(usize, Vec<usize>)> {
    let n = rows.len();
    if n == 0 {
        return Vec::new();
    }
    let ncols = rows[0].len();
    let words = n.div_ceil(64);
    let mut m: Vec<Vec<u64>> = rows.to_vec();
    let mut who: Vec<Vec<u64>> = (0..n)
        .map(|i| {
            let mut w = vec![0u64; words];
            w[i / 64] |= 1 << (i % 64);
            w
        })
        .collect();
    let mut piv: Vec<(usize, usize)> = Vec::new(); // (row, col)
    let mut r = 0usize;
    for c in 0..ncols {
        if r >= n {
            break;
        }
        let Some(pr) = (r..n).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(r, pr);
        who.swap(r, pr);
        let inv = inv_mod_u64(m[r][c], p);
        for v in m[r].iter_mut() {
            *v = ((*v as u128 * inv as u128) % p as u128) as u64;
        }
        let pivot_row = m[r].clone();
        let pivot_who = who[r].clone();
        for i in 0..n {
            if i != r && m[i][c] != 0 {
                let f = m[i][c];
                let neg = p - f;
                for (x, &y) in m[i].iter_mut().zip(&pivot_row) {
                    *x = ((*x as u128 + neg as u128 * y as u128) % p as u128) as u64;
                }
                for (a, &b) in who[i].iter_mut().zip(&pivot_who) {
                    *a |= b;
                }
            }
        }
        piv.push((r, c));
        r += 1;
    }
    piv.into_iter()
        .map(|(row, col)| {
            let contributors: Vec<usize> = (0..n)
                .filter(|&i| who[row][i / 64] >> (i % 64) & 1 == 1)
                .collect();
            (col, contributors)
        })
        .collect()
}

fn inv_mod_u64(a: u64, p: u64) -> u64 {
    // Fermat
    let mut base = a % p;
    let mut e = p - 2;
    let mut acc = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            acc = ((acc as u128 * base as u128) % p as u128) as u64;
        }
        base = ((base as u128 * base as u128) % p as u128) as u64;
        e >>= 1;
    }
    acc
}

enum TraceMode<'a> {
    Off,
    Record(&'a mut F4Trace),
    Replay(&'a F4Trace),
}

/// [`f4`] recording the trace of the run.
pub fn f4_record(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options) -> (F4Report, F4Trace) {
    let mut trace = F4Trace::default();
    let r = f4_with_arithmetic(
        input,
        n_vars,
        p,
        opts,
        arithmetic_mode(),
        TraceMode::Record(&mut trace),
    );
    (r, trace)
}

/// [`f4`] replaying a recorded trace on a system of the same shape; the
/// report's `trace_mismatch` is set (and its basis is then not a basis of
/// anything) when the shapes diverge.
pub fn f4_replay(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    trace: &F4Trace,
) -> F4Report {
    f4_with_arithmetic(
        input,
        n_vars,
        p,
        opts,
        arithmetic_mode(),
        TraceMode::Replay(trace),
    )
}

fn arithmetic_mode() -> u8 {
    static MODE: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *MODE.get_or_init(|| {
        std::env::var("F4_FP_NARROW")
            .ok()
            .and_then(|v| v.parse().ok())
            .filter(|&mode| mode <= 2)
            .unwrap_or(2)
    })
}

/// The number of monomials divisible by none of `lms` (the staircase of a
/// zero-dimensional leading ideal), or `None` when it exceeds `bound` or is
/// infinite.
///
/// The search is depth first over the variables. A monomial that is
/// already divisible with the remaining exponents at zero stays divisible
/// however they grow, so such branches are pruned. The search stops at
/// `bound + 1` leaves, so a huge staircase costs as little as a small one.
fn staircase_at_most(lms: &[&[u32]], n_vars: usize, bound: usize) -> Option<usize> {
    // the exponent of the smallest pure power of each variable among the LMs
    let mut cap = vec![0u32; n_vars];
    for (k, c) in cap.iter_mut().enumerate() {
        *c = lms
            .iter()
            .filter(|m| m.iter().enumerate().all(|(i, &e)| (i == k) == (e > 0)))
            .map(|m| m[k])
            .min()?;
    }
    fn walk(
        k: usize,
        mono: &mut Vec<u32>,
        cap: &[u32],
        lms: &[(u32, &[u32])],
        count: &mut usize,
        bound: usize,
        tests: &mut u64,
    ) -> bool {
        // only a leading monomial of degree <= deg(mono) can divide it
        let dm: u32 = mono.iter().sum();
        for (dl, l) in lms {
            if *dl > dm {
                break;
            }
            *tests += 1;
            if divides(l, mono) {
                return true;
            }
        }
        if k == cap.len() {
            *count += 1;
            return *count <= bound;
        }
        for e in 0..cap[k] {
            mono[k] = e;
            if !walk(k + 1, mono, cap, lms, count, bound, tests) {
                mono[k] = 0;
                return false;
            }
        }
        mono[k] = 0;
        true
    }
    let mut sorted: Vec<(u32, &[u32])> = lms.iter().map(|l| (total_degree(l), *l)).collect();
    sorted.sort_by_key(|x| x.0);
    let mut count = 0usize;
    let mut tests = 0u64;
    let mut mono = vec![0u32; n_vars];
    let ok = walk(0, &mut mono, &cap, &sorted, &mut count, bound, &mut tests);
    count_stair(tests);
    ok.then_some(count)
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct F4Report {
    /// The (interreduced) degree-bounded basis, `[1]` when inconsistent.
    pub basis: Vec<Poly>,
    pub inconsistent: bool,
    /// Highest pair degree processed.
    pub degree_reached: u32,
    /// Degree of the step at which `1` appeared, or at which the last
    /// new basis element was found.
    pub solving_degree: u32,
    /// The highest step degree at which the run learned something new (a
    /// new basis element, or `1`).  This is the framework's solving degree
    /// (`docs/ic/FRAMEWORK.md` §3).  It differs from `solving_degree` when
    /// step degrees fall: on tower and chain systems the normal strategy
    /// climbs, learns low-degree elements and descends again, so the last
    /// productive step need not be the highest one.
    pub solving_degree_max: u32,
    /// The widest matrix up to and including the last productive step.
    /// This is what reaching the basis costs, without the trailing steps
    /// whose pairs all reduce to zero.
    pub max_cols_to_solution: usize,
    /// Steps up to and including the last productive one.
    pub steps_to_solution: usize,
    pub steps: usize,
    pub max_rows: usize,
    pub max_cols: usize,
    pub ms: f64,
    pub timed_out: bool,
    /// Pairs dropped because their lcm degree exceeded the bound: `> 0`
    /// means the basis may be a strict truncation.
    pub pairs_above_bound: usize,
    /// When the run stopped early (`F4Options::stop_staircase`), the number
    /// of standard monomials it stopped at, with pairs still pending: the
    /// basis is then not certified complete.
    pub staircase_at_stop: Option<usize>,
    /// `F_p` multiplications spent in row reduction over this run.
    pub field_ops: u64,
    /// Steps reduced block by block under `F4Options::weights`.
    pub blocked_steps: usize,
    /// A replay found leading monomials other than the recorded ones.
    pub trace_mismatch: bool,
    /// Outcome of [`F4Options::stopping_by_hilbert`].
    pub hilbert: HilbertStop,
    /// Multiplications of the regularity check (inside `field_ops`).
    pub hilbert_check_ops: u64,
    /// Milliseconds of the run (inside `ms`) that a resumed checkpoint had
    /// already spent; 0 when the run did not resume.
    pub resumed_ms: f64,
}

/// What [`F4Options::stopping_by_hilbert`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum HilbertStop {
    /// Not requested.
    Off,
    /// The leading forms were certified regular and the Hilbert rule ran.
    Applied,
    /// The certification failed (or the input is not square): plain run.
    CheckFailed,
    /// The Hilbert rule saw fewer standard monomials than the Hilbert
    /// function allows (impossible after certification): plain run.
    Violated,
}

fn z3_weight(e: &[u32], w: &[u8]) -> u8 {
    (e.iter().zip(w).map(|(&a, &b)| a * b as u32).sum::<u32>() % 3) as u8
}

/// Reduce `rows` (over the free columns, whose weight classes are
/// `col_class`) block by block: every row lies in one class, so each
/// class is reduced on its own by `reduce`, and the pivots and reduced
/// rows are re-assembled in column order.  A `None` from `reduce` (the
/// deadline) is passed through.
fn rref_blocked<T: Copy + Default + PartialEq + Send>(
    rows: &mut Vec<Vec<T>>,
    col_class: &[u8],
    reduce: impl Fn(&mut Vec<Vec<T>>) -> Option<Vec<usize>>,
) -> Option<Vec<usize>> {
    let width = col_class.len();
    let mut out: Vec<(usize, Vec<T>)> = Vec::new();
    for class in 0..3u8 {
        let bcols: Vec<usize> = (0..width).filter(|&c| col_class[c] == class).collect();
        if bcols.is_empty() {
            continue;
        }
        let mut block: Vec<Vec<T>> = rows
            .iter()
            .filter(|r| {
                r.iter()
                    .position(|v| *v != T::default())
                    .is_some_and(|c| col_class[c] == class)
            })
            .map(|r| bcols.iter().map(|&c| r[c]).collect())
            .collect();
        if block.is_empty() {
            continue;
        }
        let piv = reduce(&mut block)?;
        for (row, &lc) in block.iter().zip(&piv) {
            let mut full = vec![T::default(); width];
            for (j, &v) in row.iter().enumerate() {
                full[bcols[j]] = v;
            }
            out.push((bcols[lc], full));
        }
    }
    out.sort_by_key(|(c, _)| *c);
    let pivots = out.iter().map(|(c, _)| *c).collect();
    *rows = out.into_iter().map(|(_, r)| r).collect();
    Some(pivots)
}

/// Pack an exponent vector into 8-bit fields of a `u128` (variable `i` in
/// bits `8i..8i + 7`).  With every exponent below 128 the product of two
/// monomials whose degree stays below 128 is the sum of their packings,
/// and the top bit of every field is free to guard divisibility tests.
fn pack(e: &[u32]) -> u128 {
    e.iter()
        .enumerate()
        .fold(0u128, |acc, (i, &x)| acc | (x as u128) << (8 * i))
}

fn unpack(x: u128, n: usize) -> Vec<u32> {
    (0..n).map(|i| ((x >> (8 * i)) & 0xff) as u32).collect()
}

/// Whether large steps run symbolic preprocessing on several threads
/// ([`Packed::symbolic_par`]); `F4_FP_SYMBOLIC_PAR=0` keeps it sequential.
fn symbolic_parallel() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_SYMBOLIC_PAR").as_deref() != Ok("0"))
}

/// Whether symbolic preprocessing and the matrix build use packed
/// monomials ([`Packed`]); `F4_FP_PACKED=0` keeps the `Vec<u32>` path as a
/// same-binary control.  The rows, columns and reducers are the same.
fn packed_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_PACKED").as_deref() != Ok("0"))
}

/// The hasher of maps keyed by packed monomials.  [`FxHasher`]'s generic
/// path copies a `u128` byte by byte, and its final multiply leaves the low
/// bits (which pick the bucket) as poorly mixed as the key's own: packed
/// monomials, small values in a few bytes, then crowd into a few buckets.
/// Folding both words and finishing with splitmix64 spreads every key bit
/// over every hash bit.
#[derive(Default, Clone, Copy)]
struct PackedHasher(u64);

impl PackedHasher {
    #[inline]
    fn mix(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

impl std::hash::Hasher for PackedHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = Self::mix(self.0 ^ u64::from(b));
        }
    }
    #[inline]
    fn write_u64(&mut self, x: u64) {
        self.0 = Self::mix(self.0 ^ x);
    }
    #[inline]
    fn write_usize(&mut self, x: usize) {
        self.write_u64(x as u64);
    }
    #[inline]
    fn write_u128(&mut self, x: u128) {
        let (lo, hi) = (x as u64, (x >> 64) as u64);
        self.0 = Self::mix(self.0 ^ lo ^ hi.wrapping_mul(0x9e37_79b9_7f4a_7c15));
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

type PkBuild = std::hash::BuildHasherDefault<PackedHasher>;
type PkMap<K, V> = std::collections::HashMap<K, V, PkBuild>;
type PkSet<K> = std::collections::HashSet<K, PkBuild>;

/// Packed monomials of the basis for grevlex runs with at most 15
/// variables and every degree below 128: products are additions, a
/// divisibility test is one guarded subtraction, and hashing and sorting
/// work on one machine word pair instead of a heap vector.
struct Packed {
    n: usize,
    /// The top bit of every used field.
    guard: u128,
    /// Every basis element's terms, packed, in the element's order.
    terms: Vec<Vec<u128>>,
    lms: Vec<u128>,
}

impl Packed {
    fn new(n: usize) -> Self {
        let guard = (0..n).fold(0u128, |g, i| g | 0x80u128 << (8 * i));
        Self {
            n,
            guard,
            terms: Vec::new(),
            lms: Vec::new(),
        }
    }

    /// Pack the basis elements added since the last call.
    fn extend(&mut self, basis: &[Poly]) {
        while self.terms.len() < basis.len() {
            let t: Vec<u128> = basis[self.terms.len()]
                .iter()
                .map(|(e, _)| pack(e))
                .collect();
            self.lms.push(t[0]);
            self.terms.push(t);
        }
    }

    /// Does `a` divide `b`?  Each field of `b | guard` minus `a`'s keeps
    /// its guard bit exactly when it does not borrow.
    #[inline]
    fn divides(&self, a: u128, b: u128) -> bool {
        ((b | self.guard) - a) & self.guard == self.guard
    }

    /// A key ordered as `cmp_monomial(·, ·, Grevlex)`: the degree above
    /// `127 − e_i` with the last variable most significant (a smaller last
    /// exponent is the larger monomial).
    fn grevlex_key(&self, x: u128) -> u128 {
        let mut deg = 0u128;
        let mut key = 0u128;
        for i in 0..self.n {
            let e = (x >> (8 * i)) & 0xff;
            deg += e;
            key |= (127 - e) << (8 * i);
        }
        deg << (8 * self.n) | key
    }

    /// [`Packed::symbolic`] spread over threads, for large steps: the
    /// reducer rows are found round by round (the rows of one round
    /// introduce the monomials whose reducers form the next).  Within a
    /// round every row's terms are looked up in the monomial table in
    /// parallel, read-only; the few monomials not yet in it are inserted
    /// afterwards in sorted order, and their reducers (the same first alive
    /// divisor) are found in parallel.  The rows are the same set as the
    /// sequential pass produces, possibly in another order (reducers by
    /// round), which the reduction does not depend on: it orders reducers
    /// by leading column.  Columns and their order are the same.
    fn symbolic_par(
        &self,
        seeds: &[(usize, Vec<u32>)],
        alive: &[bool],
    ) -> (Vec<(usize, u128)>, Vec<u128>, Vec<Vec<u32>>) {
        const UNKNOWN: u32 = u32::MAX;
        let alive_idx: Vec<usize> = (0..self.lms.len()).filter(|&i| alive[i]).collect();
        let mut ids: PkMap<u128, u32> = PkMap::default();
        let mut monos: Vec<u128> = Vec::new();
        let mut is_lead: Vec<bool> = Vec::new();
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let mut extra: Vec<(usize, u128)> = Vec::new();
        let mut frontier: Vec<(usize, u128)> = seeds.iter().map(|(i, m)| (*i, pack(m))).collect();
        while !frontier.is_empty() {
            // the frontier rows' terms against the monomials known so far
            count_native(
                frontier
                    .iter()
                    .map(|&(i, _)| self.terms[i].len() as u64)
                    .sum(),
            );
            let known = &ids;
            let mut frows: Vec<Vec<u32>> = frontier
                .par_iter()
                .map(|&(idx, m)| {
                    self.terms[idx]
                        .iter()
                        .map(|&e| known.get(&(e + m)).copied().unwrap_or(UNKNOWN))
                        .collect()
                })
                .collect();
            // the monomials not met before, deduplicated per thread first
            // (in the first round every term is new)
            let mut new: Vec<u128> = frontier
                .par_iter()
                .zip(&frows)
                .fold(PkSet::<u128>::default, |mut set, (&(idx, m), row)| {
                    for (&e, &id) in self.terms[idx].iter().zip(row) {
                        if id == UNKNOWN {
                            set.insert(e + m);
                        }
                    }
                    set
                })
                .reduce(PkSet::<u128>::default, |mut a, b| {
                    if a.len() < b.len() {
                        return b.into_iter().fold(a, |mut b2, x| {
                            b2.insert(x);
                            b2
                        });
                    }
                    a.extend(b);
                    a
                })
                .into_iter()
                .collect();
            new.sort_unstable();
            let first_new = monos.len();
            for &x in &new {
                ids.insert(x, monos.len() as u32);
                monos.push(x);
                is_lead.push(false);
            }
            let known = &ids;
            frows
                .par_iter_mut()
                .zip(&frontier)
                .for_each(|(row, &(idx, m))| {
                    for (id, &e) in row.iter_mut().zip(&self.terms[idx]) {
                        if *id == UNKNOWN {
                            *id = known[&(e + m)];
                        }
                    }
                });
            for row in &frows {
                is_lead[row[0] as usize] = true;
            }
            rows.extend(frows);
            // the reducers of this round's new monomials: the next round
            let lead = &is_lead;
            let next: Vec<(usize, u128)> = (first_new..monos.len())
                .into_par_iter()
                .filter(|&id| !lead[id])
                .filter_map(|id| {
                    let mono = monos[id];
                    let k = alive_idx
                        .iter()
                        .position(|&i| self.divides(self.lms[i], mono));
                    count_native(k.map_or(alive_idx.len(), |k| k + 1) as u64);
                    k.map(|k| (alive_idx[k], mono - self.lms[alive_idx[k]]))
                })
                .collect();
            extra.extend(next.iter().copied());
            frontier = next;
        }
        let mut order: Vec<(u128, u32)> = monos
            .iter()
            .enumerate()
            .map(|(id, &m)| (self.grevlex_key(m), id as u32))
            .collect();
        order.par_sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let mut col_of_id = vec![0u32; order.len()];
        for (col, &(_, id)) in order.iter().enumerate() {
            col_of_id[id as usize] = col as u32;
        }
        rows.par_iter_mut().for_each(|row| {
            for x in row.iter_mut() {
                *x = col_of_id[*x as usize];
            }
        });
        let cols = order
            .into_iter()
            .map(|(_, id)| monos[id as usize])
            .collect();
        (extra, cols, rows)
    }

    /// Symbolic preprocessing over packed monomials, exactly as the
    /// unpacked loop does it (same queue order, same first alive divisor as
    /// reducer).  Returns the reducer rows `(element, multiplier)` it adds,
    /// the matrix's monomials in decreasing order, and every row's terms
    /// (the seeds', then the reducers') as column indices: each monomial is
    /// hashed once, when it is first met, and its column is read off the
    /// sort instead of a second lookup.
    fn symbolic(
        &self,
        seeds: &[(usize, Vec<u32>)],
        alive: &[bool],
    ) -> (Vec<(usize, u128)>, Vec<u128>, Vec<Vec<u32>>) {
        struct Interned {
            ids: PkMap<u128, u32>,
            monos: Vec<u128>,
            done: Vec<bool>,
            queue: Vec<u32>,
        }
        impl Interned {
            fn id(&mut self, x: u128) -> u32 {
                match self.ids.entry(x) {
                    std::collections::hash_map::Entry::Occupied(o) => *o.get(),
                    std::collections::hash_map::Entry::Vacant(v) => {
                        let id = self.monos.len() as u32;
                        v.insert(id);
                        self.monos.push(x);
                        self.done.push(false);
                        self.queue.push(id);
                        id
                    }
                }
            }
        }
        let seeds: Vec<(usize, u128)> = seeds.iter().map(|(i, m)| (*i, pack(m))).collect();
        let mut seen: PkSet<(usize, u128)> = seeds.iter().copied().collect();
        let mut t = Interned {
            ids: PkMap::default(),
            monos: Vec::new(),
            done: Vec::new(),
            queue: Vec::new(),
        };
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(seeds.len());
        for &(idx, m) in &seeds {
            let row: Vec<u32> = self.terms[idx].iter().map(|&e| t.id(e + m)).collect();
            t.done[row[0] as usize] = true;
            count_native(row.len() as u64);
            rows.push(row);
        }
        let alive_idx: Vec<usize> = (0..self.lms.len()).filter(|&i| alive[i]).collect();
        let mut extra: Vec<(usize, u128)> = Vec::new();
        while let Some(id) = t.queue.pop() {
            if t.done[id as usize] {
                continue;
            }
            t.done[id as usize] = true;
            let mono = t.monos[id as usize];
            let found = alive_idx
                .iter()
                .position(|&i| self.divides(self.lms[i], mono));
            count_native(found.map_or(alive_idx.len(), |k| k + 1) as u64);
            if let Some(idx) = found.map(|k| alive_idx[k]) {
                let m = mono - self.lms[idx];
                if seen.insert((idx, m)) {
                    let row: Vec<u32> = self.terms[idx].iter().map(|&e| t.id(e + m)).collect();
                    count_native(row.len() as u64);
                    rows.push(row);
                    extra.push((idx, m));
                }
            }
        }
        let mut order: Vec<(u128, u32)> = t
            .monos
            .iter()
            .enumerate()
            .map(|(id, &m)| (self.grevlex_key(m), id as u32))
            .collect();
        order.sort_unstable_by_key(|a| std::cmp::Reverse(a.0));
        let mut col_of_id = vec![0u32; order.len()];
        for (col, &(_, id)) in order.iter().enumerate() {
            col_of_id[id as usize] = col as u32;
        }
        for row in &mut rows {
            for x in row.iter_mut() {
                *x = col_of_id[*x as usize];
            }
        }
        let cols = order
            .into_iter()
            .map(|(_, id)| t.monos[id as usize])
            .collect();
        (extra, cols, rows)
    }
}

/// The state a step starts from, kept while the step runs so an in-step
/// snapshot can write the checkpoint of its top.
struct StepTop {
    pairs: Vec<Pair>,
    steps: usize,
    max_rows: usize,
    max_cols: usize,
    degree_reached: u32,
    solving_degree: u32,
    learned: (u32, usize, usize),
    blocked_steps: usize,
    elapsed: Duration,
    ops: u64,
}

#[derive(Clone)]
struct Pair {
    i: usize,
    j: usize,
    lcm: Vec<u32>,
    /// `lcm` packed, when the run packs its monomials (else 0).
    lcm_p: u128,
    degree: u32,
}

/// The top bit of every 8-bit field of a packed monomial.
const PACKED_GUARD: u128 = 0x8080_8080_8080_8080_8080_8080_8080_8080;
/// The low bit of every field.
const PACKED_ONES: u128 = 0x0101_0101_0101_0101_0101_0101_0101_0101;

#[inline]
fn packed_divides(a: u128, b: u128) -> bool {
    ((b | PACKED_GUARD) - a) & PACKED_GUARD == PACKED_GUARD
}

/// Per field `max(a, b)`: the guard bit survives `(a | G) − b` exactly where
/// `a ≥ b`, and spreading it over its field selects `a` there.
#[inline]
fn packed_lcm(a: u128, b: u128) -> u128 {
    let ge = ((a | PACKED_GUARD) - b) & PACKED_GUARD;
    let sel = (ge >> 7) * 0xff;
    (a & sel) | (b & !sel)
}

/// No variable in both: the nonzero fields of `a` and `b` are disjoint.
#[inline]
fn packed_coprime(a: u128, b: u128) -> bool {
    let nz = |x: u128| ((x | PACKED_GUARD) - PACKED_ONES) & PACKED_GUARD;
    nz(a) & nz(b) == 0
}

/// The Gebauer–Möller update for a new leading monomial `h` (Becker and
/// Weispfenning's UPDATE), over any monomial representation: criterion B
/// on the pending pairs (and those held above the degree bound), the
/// `alive` flags of elements `h` makes redundant, and the new pairs
/// `(g, lcm)` that survive criteria M and F and the product criterion
/// (all of them, bar the coprime ones, when `gm` is off).
#[allow(clippy::too_many_arguments)]
fn gm_update<K: PartialEq>(
    h: &K,
    lm: impl Fn(usize) -> K,
    pairs: &mut Vec<Pair>,
    above: &mut Vec<Pair>,
    pair_lcm: impl Fn(&Pair) -> K,
    alive: &mut [bool],
    k: usize,
    gm: bool,
    div: impl Fn(&K, &K) -> bool,
    lcmf: impl Fn(&K, &K) -> K,
    coprime: impl Fn(&K, &K) -> bool,
) -> Vec<(usize, K)> {
    // bitop/v1: criterion B tests every held pair (a divisibility and two
    // lcm compares, a packed monomial being two words each), every alive
    // element gets a divisibility, coprimality and lcm, and criteria M/F
    // compare candidate pairs (charged at the quadratic bound)
    super::jv_bitops::words(
        if gm {
            6 * (pairs.len() + above.len()) as u64
        } else {
            0
        } + 6 * k as u64,
    );
    if gm {
        // Criterion B: an old pair (g1, g2) whose lcm `h` divides is covered
        // by (g1, h) and (h, g2) unless one of those has the same lcm.  Pairs
        // held above the degree bound are tested too: a pair the criterion
        // retires was never needed.
        let keep = |pr: &Pair| {
            let l = pair_lcm(pr);
            !div(h, &l) || lcmf(&lm(pr.i), h) == l || lcmf(&lm(pr.j), h) == l
        };
        pairs.retain(|pr| keep(pr));
        above.retain(|pr| keep(pr));
    }
    // the new pairs (h, g) with every alive g
    let mut cand: Vec<(usize, K, bool)> = Vec::new();
    for i in 0..k {
        if !alive[i] {
            continue;
        }
        let lmi = lm(i);
        if div(h, &lmi) {
            // `basis[i]` is redundant in the final basis, but its pair with
            // the new element must still be processed (Buchberger's
            // criterion is about the generating set, not the reduced one);
            // only its role as an output element ends here.
            alive[i] = false;
        }
        let cp = coprime(h, &lmi);
        cand.push((i, lcmf(h, &lmi), cp));
    }
    let mut kept = vec![true; cand.len()];
    if gm {
        super::jv_bitops::words(2 * (cand.len() * cand.len()) as u64);
        // Criteria M and F: drop (h, g1) when another pair (h, g2) still
        // standing has an lcm dividing its own; of several with equal lcms
        // the last one stays.  A coprime pair is never dropped here (it may
        // drop others) and never kept below.
        for a in 0..cand.len() {
            if cand[a].2 {
                continue;
            }
            if (0..cand.len()).any(|b| b != a && kept[b] && div(&cand[b].1, &cand[a].1)) {
                kept[a] = false;
            }
        }
    }
    cand.into_iter()
        .zip(kept)
        .filter(|((_, _, cp), keep)| !cp && *keep)
        .map(|((i, l, _), _)| (i, l))
        .collect()
}

fn lcm(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().zip(b).map(|(x, y)| *x.max(y)).collect()
}

/// Whether new pairs pass the Gebauer–Möller criteria (Buchberger's chain
/// criterion as criteria B, M and F) on top of the product criterion.  On
/// `quad_n9_p65521` the basis is complete at degree 10 and every later step
/// only reduces redundant pairs to zero.  `F4_FP_GM=0` keeps the product
/// criterion alone, as a same-binary control; the reduced basis is the same.
fn gebauer_moeller() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_GM").as_deref() != Ok("0"))
}

/// Rows × columns of remaining elimination work below which a pivot's
/// elimination runs on one thread: under it, rayon's dispatch costs more
/// than the arithmetic it spreads.
const PAR_WORK: usize = 1 << 15;

/// Subtract `f · pivot` from `row` over the columns `from..`, with
/// `f = row[col]`.  `p < 2³²`, so `x + (p − f)·y < 2⁶⁴` fits a `u64`
/// and one Barrett reduction replaces the `u128` remainder of [`mulmod`].
#[inline]
fn eliminate(row: &mut [u64], pivot: &[u64], col: usize, fp: Barrett) {
    let f = row[col];
    if f == 0 {
        return;
    }
    let neg = fp.p - f;
    let mut ops = 0usize;
    for (x, &y) in row[col..].iter_mut().zip(&pivot[col..]) {
        if y != 0 {
            *x = fp.reduce(*x + neg * y);
            ops += 1;
        }
    }
    count_ops(ops);
}

#[inline]
fn eliminate32(row: &mut [u32], pivot: &[u32], col: usize, fp: Barrett32) {
    let f = row[col];
    if f == 0 {
        return;
    }
    let neg = fp.p - f;
    for (x, &y) in row[col..].iter_mut().zip(&pivot[col..]) {
        *x = fp.reduce(*x + neg * y);
    }
    count_ops(row.len() - col);
}

fn rref32(
    rows: &mut Vec<Vec<u32>>,
    fp: Barrett32,
    deadline: Option<Instant>,
) -> Option<Vec<usize>> {
    let n_cols = rows.first().map_or(0, |r| r.len());
    let mut pivots = Vec::new();
    let mut pr = 0usize;
    for c in 0..n_cols {
        if pr >= rows.len() {
            break;
        }
        if c % 32 == 0 && deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let found = (pr..rows.len()).find(|&r| rows[r][c] != 0);
        super::jv_bitops::words(found.map_or(rows.len(), |r| r + 1) as u64 - pr as u64);
        let Some(r) = found else {
            continue;
        };
        rows.swap(pr, r);
        let inv = invmod(rows[pr][c] as u64, fp.p as u64) as u32;
        for x in &mut rows[pr][c..] {
            *x = fp.reduce(*x * inv);
        }
        count_ops(n_cols - c);
        let (head, tail) = rows.split_at_mut(pr + 1);
        let prow = &head[pr];
        if tail.len() * (n_cols - c) > PAR_WORK {
            tail.par_iter_mut()
                .for_each(|row| eliminate32(row, prow, c, fp));
        } else {
            tail.iter_mut()
                .for_each(|row| eliminate32(row, prow, c, fp));
        }
        pivots.push(c);
        pr += 1;
    }
    rows.truncate(pr);
    for i in (1..pr).rev() {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let c = pivots[i];
        let (head, tail) = rows.split_at_mut(i);
        let prow = &tail[0];
        if head.len() * (n_cols - c) > PAR_WORK {
            head.par_iter_mut()
                .for_each(|row| eliminate32(row, prow, c, fp));
        } else {
            head.iter_mut()
                .for_each(|row| eliminate32(row, prow, c, fp));
        }
    }
    Some(pivots)
}

#[inline(always)]
fn add_lazy_scalar(acc: &mut [u64], pivot: &[u32], from: usize, factor: u64) {
    for (x, &y) in acc[from..].iter_mut().zip(&pivot[from..]) {
        *x += factor * u64::from(y);
    }
}

/// Four bounded 32-bit products accumulated in 64-bit lanes. The caller's
/// existing normalization interval proves these additions cannot overflow.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn add_lazy_avx2(acc: &mut [u64], pivot: &[u32], from: usize, factor: u64) {
    use std::arch::x86_64::{
        __m128i, __m256i, _mm256_add_epi64, _mm256_cvtepu32_epi64, _mm256_loadu_si256,
        _mm256_mul_epu32, _mm256_set1_epi64x, _mm256_storeu_si256, _mm_loadu_si128,
    };
    let (acc, pivot) = (&mut acc[from..], &pivot[from..]);
    let multiplier = _mm256_set1_epi64x(factor as i64);
    let mut i = 0;
    while i + 4 <= acc.len() {
        // SAFETY: the loop bounds cover four u32 pivot entries and four
        // u64 accumulator entries; both intrinsics accept unaligned data.
        let values = unsafe { _mm_loadu_si128(pivot.as_ptr().add(i) as *const __m128i) };
        let wide = _mm256_cvtepu32_epi64(values);
        let products = _mm256_mul_epu32(wide, multiplier);
        let current = unsafe { _mm256_loadu_si256(acc.as_ptr().add(i) as *const __m256i) };
        let updated = _mm256_add_epi64(current, products);
        unsafe { _mm256_storeu_si256(acc.as_mut_ptr().add(i) as *mut __m256i, updated) };
        i += 4;
    }
    for (x, &y) in acc[i..].iter_mut().zip(&pivot[i..]) {
        *x += factor * u64::from(y);
    }
}

/// Eight bounded 32-bit products accumulated in 64-bit lanes. The same
/// 256-update normalization interval used by the scalar and AVX2 paths
/// bounds every lane before it is reduced.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn add_lazy_avx512(acc: &mut [u64], pivot: &[u32], from: usize, factor: u64) {
    use std::arch::x86_64::{
        __m256i, __m512i, _mm256_loadu_si256, _mm512_add_epi64, _mm512_cvtepu32_epi64,
        _mm512_loadu_si512, _mm512_mul_epu32, _mm512_set1_epi64, _mm512_storeu_si512,
    };
    let (acc, pivot) = (&mut acc[from..], &pivot[from..]);
    let multiplier = _mm512_set1_epi64(factor as i64);
    let mut i = 0;
    while i + 8 <= acc.len() {
        // SAFETY: the loop bounds cover eight pivot and accumulator entries;
        // both loads and the store accept unaligned pointers.
        let values = unsafe { _mm256_loadu_si256(pivot.as_ptr().add(i) as *const __m256i) };
        let wide = _mm512_cvtepu32_epi64(values);
        let products = _mm512_mul_epu32(wide, multiplier);
        let current = unsafe { _mm512_loadu_si512(acc.as_ptr().add(i) as *const __m512i) };
        let updated = _mm512_add_epi64(current, products);
        unsafe { _mm512_storeu_si512(acc.as_mut_ptr().add(i) as *mut __m512i, updated) };
        i += 8;
    }
    for (x, &y) in acc[i..].iter_mut().zip(&pivot[i..]) {
        *x += factor * u64::from(y);
    }
}

/// Zero selects scalar accumulation, one AVX2, two AVX-512F.  The widest
/// vector path the CPU has is the default, since the Faugère–Lachartre
/// step runs its dense updates through this kernel (`f4_fp_bench 2 large`
/// on an AVX-512 host: quad_n9 6.7 s with AVX-512, 7.2 s with AVX2, 8.5 s
/// scalar); `F4_FP_LAZY_SIMD=0` forces scalar, `F4_FP_LAZY_AVX2=1` forces
/// AVX2 and `F4_FP_LAZY_AVX512=1` AVX-512, as before.
fn lazy_simd_kind() -> u8 {
    #[cfg(target_arch = "x86_64")]
    {
        static KIND: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
        *KIND.get_or_init(|| {
            let avx2 = std::arch::is_x86_feature_detected!("avx2");
            let avx512 = avx2 && std::arch::is_x86_feature_detected!("avx512f");
            if std::env::var("F4_FP_LAZY_SIMD").as_deref() == Ok("0") {
                0
            } else if std::env::var("F4_FP_LAZY_AVX512").as_deref() == Ok("1") && avx512 {
                2
            } else if std::env::var("F4_FP_LAZY_AVX2").as_deref() == Ok("1") && avx2 {
                1
            } else if avx512 {
                2
            } else if avx2 {
                1
            } else {
                0
            }
        })
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

#[inline]
fn add_lazy(acc: &mut [u64], pivot: &[u32], from: usize, factor: u64, simd: u8) {
    debug_assert_eq!(acc.len(), pivot.len());
    #[cfg(target_arch = "x86_64")]
    if simd == 0 {
        add_lazy_scalar(acc, pivot, from, factor);
    } else if simd == 2 {
        // SAFETY: `lazy_simd_kind` verified AVX-512F and AVX2.
        unsafe { add_lazy_avx512(acc, pivot, from, factor) };
    } else {
        // SAFETY: `lazy_simd_kind` verified AVX2 before this call.
        unsafe { add_lazy_avx2(acc, pivot, from, factor) };
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = simd;
        add_lazy_scalar(acc, pivot, from, factor);
    }
    count_ops(acc.len() - from);
}

#[inline]
fn normalize_lazy(acc: &mut [u64], fp: Barrett) {
    for x in acc {
        *x = fp.reduce(*x);
    }
}

/// Rows of a step reduced together against the pivots known so far.  A
/// chunk's rows are independent of each other until they are stored, so
/// their reductions run in parallel; a smaller chunk finds pivots sooner
/// for the rows after it, a larger one spreads better.
const DEFERRED_CHUNK: usize = 64;

/// `F4_FP_DEFERRED_PAR=0` runs the deferred elimination row by row on one
/// thread, as it did before the chunks; a same-binary control.
fn deferred_parallel() -> bool {
    static PAR: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PAR.get_or_init(|| std::env::var("F4_FP_DEFERRED_PAR").as_deref() != Ok("0"))
}

/// Reduce `acc` (64-bit lazy accumulators of one row) by `known`, in
/// increasing pivot column, with the bounded normalisation that keeps the
/// accumulators far below `u64` overflow; `pending` carries the count of
/// unnormalised updates in and out.
#[inline]
fn reduce_lazy_by(
    acc: &mut [u64],
    known: &[(usize, Vec<u32>)],
    fp: Barrett,
    simd: u8,
    pending: &mut usize,
) {
    const NORMALIZE_AFTER: usize = 256;
    for (c, pivot) in known {
        let f = fp.reduce(acc[*c]);
        if f != 0 {
            add_lazy(acc, pivot, *c, fp.p - f, simd);
            *pending += 1;
            if *pending == NORMALIZE_AFTER {
                normalize_lazy(acc, fp);
                *pending = 0;
            }
        }
    }
}

/// Row-oriented RREF.  Each row keeps a 64-bit scratch accumulator while
/// it meets the existing 32-bit pivots, reducing only the next pivot
/// coefficient and the row when it is stored.  A bounded normalization
/// keeps even the largest supported prime far below `u64` overflow.
///
/// Rows are taken in chunks of [`DEFERRED_CHUNK`]: every row of a chunk is
/// reduced in parallel against the pivots known before the chunk, then the
/// rows are stored one at a time, each first reduced again against the
/// whole pivot list so the pivots its chunk-mates contributed (and any
/// earlier pivot column those re-dirtied) are cleared — a pass that costs
/// one coefficient reduction per pivot already clear.  The result is the
/// unique reduced echelon form either way.  Back-substitution reduces every
/// pivot row by the *unreduced* later pivots in increasing column order,
/// which clears each later pivot column for good and so gives the same
/// fully reduced rows as the sequential pass did, with the rows now
/// independent and in parallel.
fn rref32_deferred(
    rows: &mut Vec<Vec<u32>>,
    fp32: Barrett32,
    fp: Barrett,
    deadline: Option<Instant>,
) -> Option<Vec<usize>> {
    let simd = lazy_simd_kind();
    let parallel = deferred_parallel();
    let mut known: Vec<(usize, Vec<u32>)> = Vec::new();
    let sources = std::mem::take(rows);
    let chunk_rows = if parallel { DEFERRED_CHUNK } else { 1 };
    for chunk in sources.chunks(chunk_rows) {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let n_cols = chunk[0].len();
        let reduce_one = |source: &Vec<u32>| -> (Vec<u64>, usize) {
            let mut acc: Vec<u64> = source.iter().copied().map(u64::from).collect();
            let mut pending = 0usize;
            reduce_lazy_by(&mut acc, &known, fp, simd, &mut pending);
            (acc, pending)
        };
        let mut reduced: Vec<(Vec<u64>, usize)> =
            if chunk.len() > 1 && chunk.len() * known.len().max(1) * n_cols > PAR_WORK {
                chunk.par_iter().map(reduce_one).collect()
            } else {
                chunk.iter().map(reduce_one).collect()
            };
        let known_before = known.len();
        for (acc, pending) in reduced.iter_mut() {
            if known.len() > known_before {
                // chunk-mates became pivots: clear them, and whatever
                // earlier pivot columns they re-dirtied
                reduce_lazy_by(acc, &known, fp, simd, pending);
            }
            let mut row: Vec<u32> = acc.iter().map(|&v| fp.reduce(v) as u32).collect();
            let Some(c) = row.iter().position(|&v| v != 0) else {
                continue;
            };
            let inv = invmod(u64::from(row[c]), fp.p) as u32;
            for x in &mut row[c..] {
                *x = fp32.reduce(*x * inv);
            }
            count_ops(row.len() - c);
            let at = known.partition_point(|(pc, _)| *pc < c);
            known.insert(at, (c, row));
        }
    }
    if deadline.is_some_and(|d| Instant::now() > d) {
        return None;
    }
    let n_cols = known.first().map_or(0, |(_, r)| r.len());
    let back = |i: usize| -> Vec<u32> {
        let (_, row) = &known[i];
        let mut acc: Vec<u64> = row.iter().copied().map(u64::from).collect();
        let mut pending = 0usize;
        reduce_lazy_by(&mut acc, &known[i + 1..], fp, simd, &mut pending);
        acc.into_iter().map(|v| fp.reduce(v) as u32).collect()
    };
    let reduced: Vec<Vec<u32>> = if parallel && known.len() * n_cols > PAR_WORK {
        (0..known.len()).into_par_iter().map(back).collect()
    } else {
        (0..known.len()).map(back).collect()
    };
    let pivots = known.iter().map(|(c, _)| *c).collect();
    *rows = reduced;
    Some(pivots)
}

/// Most entries of `B'` (reducers × free columns, 4 bytes each) the
/// Faugère–Lachartre form is allowed; above it the step falls back to
/// reducing each pair row through the reducer cascade.
const FL_MAX_ENTRIES: usize = 1 << 28;

/// Reducers finished together in [`reduce_pairs_fl`].
const FL_BLOCK: usize = 64;

/// Columns a step needs before the Faugère–Lachartre form is used.
const FL_MIN_COLS: usize = 256;

/// `F4_FP_FL=0` keeps the per-pair-row reducer cascade; a same-binary
/// control for the Faugère–Lachartre form.
fn fl_enabled() -> bool {
    static FL: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *FL.get_or_init(|| std::env::var("F4_FP_FL").as_deref() != Ok("0"))
}

/// The pair rows reduced by the reducers, over the free columns, in the
/// Faugère–Lachartre form (see the call site).  `reducer_at[c]` is the
/// reducer led by column `c` or `u32::MAX`; `free_cols` is ascending.
/// Entries are `< p`; `None` when the deadline passed.
#[allow(clippy::too_many_arguments)]
fn reduce_pairs_fl(
    pair_rows: &[Vec<(u32, u64)>],
    reducers: &[Vec<(u32, u64)>],
    reducer_at: &[u32],
    free_cols: &[usize],
    n_cols: usize,
    fp: Barrett,
    deadline: Option<Instant>,
) -> Option<Vec<Vec<u32>>> {
    const NONE: u32 = u32::MAX;
    const NORMALIZE_AFTER: usize = 256;
    let simd = lazy_simd_kind();
    let n_free = free_cols.len();
    let mut free_index = vec![NONE; n_cols];
    for (i, &c) in free_cols.iter().enumerate() {
        free_index[c] = i as u32;
    }
    // A row of `B'` is zero on the free columns left of its reducer's lead
    // (every term of a reducer, and of everything it is reduced by, lies
    // right of that lead), so its updates start at the first free column
    // past the lead.
    let bstart: Vec<usize> = reducers
        .iter()
        .map(|r| free_cols.partition_point(|&c| c <= r[0].0 as usize))
        .collect();
    // reducers by leading column, rightmost first: a reducer's entry at
    // another reducer's column lies right of its own lead, so that
    // reducer is finished before this one is reached
    let mut order: Vec<usize> = (0..reducers.len()).collect();
    order.sort_unstable_by_key(|&k| std::cmp::Reverse(reducers[k][0].0));
    let mut position = vec![0usize; reducers.len()];
    for (pos, &k) in order.iter().enumerate() {
        position[k] = pos;
    }
    let mut bprime: Vec<Vec<u32>> = vec![Vec::new(); reducers.len()];
    // `(acc, pending, in-block dependencies)` of one reducer against the
    // finished blocks
    let against_finished = |k: usize, block_start: usize, bprime: &Vec<Vec<u32>>| {
        let mut acc = vec![0u64; n_free];
        let mut pending = 0usize;
        let mut later: Vec<(usize, u64)> = Vec::new();
        for &(c, v) in &reducers[k][1..] {
            let fi = free_index[c as usize];
            if fi != NONE {
                acc[fi as usize] += v;
            } else {
                let j = reducer_at[c as usize] as usize;
                if position[j] < block_start {
                    add_lazy(&mut acc, &bprime[j], bstart[j], fp.p - v, simd);
                    pending += 1;
                    if pending == NORMALIZE_AFTER {
                        normalize_lazy(&mut acc, fp);
                        pending = 0;
                    }
                } else {
                    later.push((j, v));
                }
            }
        }
        (acc, pending, later)
    };
    for block in order.chunks(FL_BLOCK) {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let block_start = position[block[0]];
        let bp = &bprime;
        let mut partial: Vec<(Vec<u64>, usize, Vec<(usize, u64)>)> =
            if block.len() * n_free > PAR_WORK {
                block
                    .par_iter()
                    .map(|&k| against_finished(k, block_start, bp))
                    .collect()
            } else {
                block
                    .iter()
                    .map(|&k| against_finished(k, block_start, bp))
                    .collect()
            };
        for (&k, (acc, pending, later)) in block.iter().zip(partial.iter_mut()) {
            // in-block dependencies are earlier in the block, so finished
            for &(j, v) in later.iter() {
                debug_assert!(position[j] < position[k]);
                add_lazy(acc, &bprime[j], bstart[j], fp.p - v, simd);
                *pending += 1;
                if *pending == NORMALIZE_AFTER {
                    normalize_lazy(acc, fp);
                    *pending = 0;
                }
            }
            bprime[k] = acc.iter().map(|&x| fp.reduce(x) as u32).collect();
        }
    }
    if fl_blocked_pairs() {
        return reduce_pairs_blocked(
            pair_rows,
            &bprime,
            &bstart,
            reducer_at,
            &free_index,
            n_free,
            fp,
            deadline,
        );
    }
    let reduce_pair = |r: &Vec<(u32, u64)>| -> Option<Vec<u32>> {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let mut acc = vec![0u64; n_free];
        let mut pending = 0usize;
        for &(c, v) in r {
            let fi = free_index[c as usize];
            if fi != NONE {
                acc[fi as usize] += v;
            } else {
                let j = reducer_at[c as usize] as usize;
                add_lazy(&mut acc, &bprime[j], bstart[j], fp.p - v, simd);
                pending += 1;
                if pending == NORMALIZE_AFTER {
                    normalize_lazy(&mut acc, fp);
                    pending = 0;
                }
            }
        }
        Some(acc.iter().map(|&x| fp.reduce(x) as u32).collect())
    };
    if pair_rows.len() * n_free > PAR_WORK {
        pair_rows.par_iter().map(reduce_pair).collect()
    } else {
        pair_rows.iter().map(reduce_pair).collect()
    }
}

/// `F4_FP_FL_BLOCKED=0` reduces each pair row on its own against `B'`, as
/// before the blocked product; a same-binary control.
fn fl_blocked_pairs() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_FL_BLOCKED").as_deref() != Ok("0"))
}

/// Pair rows reduced together in [`reduce_pairs_blocked`].
const FL_PAIR_BLOCK: usize = 64;

/// Free columns per tile in [`reduce_pairs_blocked`]: a block's
/// accumulators over one tile (64 × 256 × 8 bytes) stay in L2.
const FL_PAIR_TILE: usize = 256;

/// `D − C·B'` for the pair rows `C | D`, a block of rows and a tile of
/// free columns at a time.
///
/// Each pair row is one dense update per entry at a reducer column, by
/// that reducer's row of `B'`.  Row by row, every update streams its `B'`
/// row from memory; here the block's updates are grouped by reducer, so a
/// tile of a `B'` row is read once and added into every row of the block
/// that needs it, while the block's accumulators over the tile stay in
/// cache.  The updates and their sum are unchanged; only their order is,
/// and the sum is exact (below): every reduced row is the same.
///
/// With `p ≤ 2¹⁶` (the only arithmetic this runs under) every product is
/// below `2³²` and an entry receives at most one per reducer, so the
/// 64-bit accumulators cannot overflow without four billion reducers;
/// they are reduced once, at the end.
#[allow(clippy::too_many_arguments)]
fn reduce_pairs_blocked(
    pair_rows: &[Vec<(u32, u64)>],
    bprime: &[Vec<u32>],
    bstart: &[usize],
    reducer_at: &[u32],
    free_index: &[u32],
    n_free: usize,
    fp: Barrett,
    deadline: Option<Instant>,
) -> Option<Vec<Vec<u32>>> {
    const NONE: u32 = u32::MAX;
    assert!(
        fp.p <= 1 << 16 && bprime.len() < 1 << 31,
        "accumulator bound"
    );
    let simd = lazy_simd_kind();
    let block = |rows: &[Vec<(u32, u64)>]| -> Option<Vec<Vec<u32>>> {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        let mut acc: Vec<Vec<u64>> = vec![vec![0u64; n_free]; rows.len()];
        // (reducer, row of the block, factor), grouped by reducer
        let mut hits: Vec<(u32, u32, u64)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            for &(c, v) in r {
                let fi = free_index[c as usize];
                if fi != NONE {
                    acc[i][fi as usize] += v;
                } else {
                    hits.push((reducer_at[c as usize], i as u32, fp.p - v));
                }
            }
        }
        hits.sort_unstable_by_key(|&(j, i, _)| (j, i));
        let mut from = 0;
        while from < n_free {
            let to = (from + FL_PAIR_TILE).min(n_free);
            for &(j, i, factor) in &hits {
                let j = j as usize;
                let start = bstart[j].max(from);
                if start >= to {
                    continue;
                }
                add_lazy_range(&mut acc[i as usize], &bprime[j], start, to, factor, simd);
            }
            from = to;
        }
        count_ops(
            hits.iter()
                .map(|&(j, _, _)| n_free - bstart[j as usize])
                .sum(),
        );
        Some(
            acc.into_iter()
                .map(|a| a.into_iter().map(|x| fp.reduce(x) as u32).collect())
                .collect(),
        )
    };
    let blocks: Vec<Option<Vec<Vec<u32>>>> = if pair_rows.len() * n_free > PAR_WORK {
        pair_rows.par_chunks(FL_PAIR_BLOCK).map(block).collect()
    } else {
        pair_rows.chunks(FL_PAIR_BLOCK).map(block).collect()
    };
    let mut out = Vec::with_capacity(pair_rows.len());
    for b in blocks {
        out.extend(b?);
    }
    Some(out)
}

/// [`add_lazy`] over the columns `from..to` only, uncounted (the caller
/// counts the whole update once).
#[inline]
fn add_lazy_range(acc: &mut [u64], pivot: &[u32], from: usize, to: usize, factor: u64, simd: u8) {
    let (acc, pivot) = (&mut acc[..to], &pivot[..to]);
    #[cfg(target_arch = "x86_64")]
    if simd == 0 {
        add_lazy_scalar(acc, pivot, from, factor);
    } else if simd == 2 {
        // SAFETY: `lazy_simd_kind` verified AVX-512F and AVX2.
        unsafe { add_lazy_avx512(acc, pivot, from, factor) };
    } else {
        // SAFETY: `lazy_simd_kind` verified AVX2 before this call.
        unsafe { add_lazy_avx2(acc, pivot, from, factor) };
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = simd;
        add_lazy_scalar(acc, pivot, from, factor);
    }
}

/// Where a dense reduction stands, shown to the in-step snapshot hook at a
/// column boundary (`phase` 0, forward, `c` the next column) or before a
/// back-substitution row (`phase` 1, `i` the row about to be cleared).
struct RrefView<'a> {
    phase: u32,
    c: usize,
    pr: usize,
    i: usize,
    pivots: &'a [usize],
    rows: &'a [Vec<u64>],
}

/// Row-reduce `rows` (dense, entries in `[0, p)`) to reduced echelon form
/// in place: zero rows are dropped, the rest sorted by pivot column, whose
/// list is returned.  `None` when the deadline passed first.
fn rref(rows: &mut Vec<Vec<u64>>, fp: Barrett, deadline: Option<Instant>) -> Option<Vec<usize>> {
    rref_ckpt(rows, fp, deadline, None, None)
}

/// [`rref`] that can start from a snapshot `(phase, c, pr, i, pivots)` of its
/// own partly reduced `rows`, and that shows its position to `hook` at column
/// boundaries; a hook returning `true` stops the reduction (`None`).
fn rref_ckpt(
    rows: &mut Vec<Vec<u64>>,
    fp: Barrett,
    deadline: Option<Instant>,
    resume: Option<(u32, usize, usize, usize, Vec<usize>)>,
    mut hook: Option<&mut dyn FnMut(&RrefView) -> bool>,
) -> Option<Vec<usize>> {
    let p = fp.p;
    let n_cols = rows.first().map_or(0, |r| r.len());
    let (phase0, c0, mut pr, i0, mut pivots) = match resume {
        Some((ph, c, pr, i, pv)) => (ph, c, pr, i, pv),
        None => (0, 0, 0, 0, Vec::new()),
    };
    // backward phase: rows `1..i_excl` are still to be cleared, from the top down
    let mut i_excl = i0 + 1;
    if phase0 == 0 {
        // forward: echelon form
        for c in c0..n_cols {
            if pr >= rows.len() {
                break;
            }
            if c % 32 == 0 && deadline.is_some_and(|d| Instant::now() > d) {
                return None;
            }
            if let Some(h) = hook.as_mut() {
                let view = RrefView {
                    phase: 0,
                    c,
                    pr,
                    i: 0,
                    pivots: &pivots,
                    rows,
                };
                if h(&view) {
                    return None;
                }
            }
            let found = (pr..rows.len()).find(|&r| rows[r][c] != 0);
            super::jv_bitops::words(found.map_or(rows.len(), |r| r + 1) as u64 - pr as u64);
            let Some(r) = found else {
                continue;
            };
            rows.swap(pr, r);
            let inv = invmod(rows[pr][c], p);
            let mut ops = 0usize;
            for x in rows[pr][c..].iter_mut() {
                if *x != 0 {
                    *x = fp.reduce(*x * inv);
                    ops += 1;
                }
            }
            count_ops(ops);
            let (head, tail) = rows.split_at_mut(pr + 1);
            let prow = &head[pr];
            if tail.len() * (n_cols - c) > PAR_WORK {
                tail.par_iter_mut()
                    .for_each(|row| eliminate(row, prow, c, fp));
            } else {
                tail.iter_mut().for_each(|row| eliminate(row, prow, c, fp));
            }
            pivots.push(c);
            pr += 1;
        }
        rows.truncate(pr);
        i_excl = pr;
    }
    // backward: clear each pivot column above its pivot
    for i in (1..i_excl).rev() {
        if deadline.is_some_and(|d| Instant::now() > d) {
            return None;
        }
        if let Some(h) = hook.as_mut() {
            let view = RrefView {
                phase: 1,
                c: 0,
                pr,
                i,
                pivots: &pivots,
                rows,
            };
            if h(&view) {
                return None;
            }
        }
        let c = pivots[i];
        let (head, tail) = rows.split_at_mut(i);
        let prow = &tail[0];
        if head.len() * (n_cols - c) > PAR_WORK {
            head.par_iter_mut()
                .for_each(|row| eliminate(row, prow, c, fp));
        } else {
            head.iter_mut().for_each(|row| eliminate(row, prow, c, fp));
        }
    }
    Some(pivots)
}

/// Degree-bounded F4.
pub fn f4(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options) -> F4Report {
    notify_progress(F4Progress {
        event: "begin",
        variables: n_vars,
        step: 0,
        degree: 0,
        rows: input.len(),
        columns: 0,
        productive_degree: 0,
        field_multiplications: 0,
        elapsed_ms: 0.0,
    });
    let report = if let Some(chunk) = opts.stop_hilbert {
        f4_hilbert_guarded(input, n_vars, p, opts, chunk)
    } else {
        f4_with_arithmetic(input, n_vars, p, opts, arithmetic_mode(), TraceMode::Off)
    };
    notify_progress(F4Progress {
        event: "complete",
        variables: n_vars,
        step: report.steps,
        degree: report.degree_reached,
        rows: report.max_rows,
        columns: report.max_cols,
        productive_degree: report.solving_degree_max,
        field_multiplications: report.field_ops,
        elapsed_ms: report.ms,
    });
    report
}

/// The Hilbert function `prod_i (1 + t + ... + t^(d_i - 1))` of a complete
/// intersection of forms of degrees `d_i`, as coefficients `h[0..=sum(d_i-1)]`.
pub fn complete_intersection_hilbert(degrees: &[u32]) -> Vec<usize> {
    let mut h = vec![1usize];
    for &d in degrees {
        let mut next = vec![0usize; h.len() + d as usize - 1];
        for (i, &c) in h.iter().enumerate() {
            for e in 0..d as usize {
                next[i + e] += c;
            }
        }
        h = next;
    }
    h
}

/// [`F4Options::stopping_by_hilbert`]: certify that the leading forms are a
/// regular sequence, run the Hilbert rule, and fall back to the plain run
/// when the certificate fails.
fn f4_hilbert_guarded(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    chunk: usize,
) -> F4Report {
    let ops0 = field_ops_now();
    let t0 = Instant::now();
    let degs: Vec<u32> = input
        .iter()
        .map(|f| f.iter().map(|(e, _)| total_degree(e)).max().unwrap_or(0))
        .collect();
    let mut plain = opts.clone();
    plain.stop_hilbert = None;
    let mut check_ops = 0u64;
    let mut status = HilbertStop::CheckFailed;
    if input.len() == n_vars
        && degs.iter().all(|&d| d >= 1)
        && matches!(opts.order, Ordering::Grevlex)
    {
        // F4 on the leading forms.  A finite staircase of any subset of the
        // ideal `(h)` makes `R/(h)` finite-dimensional, so the `n` forms in
        // `n` variables are a regular sequence (Lemma 1); the Macaulay
        // degree `sum(d_i - 1) + 1` bounds the degrees that matter.
        let lead: Vec<Poly> = input
            .iter()
            .zip(&degs)
            .map(|(f, &d)| {
                let t: Vec<(Vec<u32>, u64)> = f
                    .iter()
                    .filter(|(e, _)| total_degree(e) == d)
                    .cloned()
                    .collect();
                normalise(&t, p, opts.order)
            })
            .collect();
        let bound: usize = degs.iter().map(|&d| d as usize).product();
        let dreg: u32 = degs.iter().map(|d| d - 1).sum::<u32>() + 1;
        let mut co = F4Options::new(opts.order, dreg.min(opts.max_degree));
        co.deadline = opts.deadline;
        co.stop_staircase = Some(bound);
        // Which pairs the check skips does not matter for its soundness, only
        // that the staircase it ends with is finite; skipping by the Hilbert
        // function is a heuristic here that makes the check cheap.
        co.hilbert_expect = Some(complete_intersection_hilbert(&degs));
        co.max_pairs_per_step = Some(chunk);
        let r = f4_with_arithmetic(&lead, n_vars, p, &co, arithmetic_mode(), TraceMode::Off);
        check_ops = r.field_ops;
        let lms: Vec<&[u32]> = r.basis.iter().map(|g| g[0].0.as_slice()).collect();
        if !r.timed_out
            && !r.inconsistent
            && lead.iter().all(|f| !f.is_empty())
            && staircase_at_most(&lms, n_vars, bound).is_some()
        {
            status = HilbertStop::Applied;
        }
    }
    if status == HilbertStop::Applied {
        let mut o = plain.clone();
        o.hilbert_expect = Some(complete_intersection_hilbert(&degs));
        o.max_pairs_per_step = Some(chunk);
        let r = f4_with_arithmetic(input, n_vars, p, &o, arithmetic_mode(), TraceMode::Off);
        if r.hilbert == HilbertStop::Applied {
            return F4Report {
                field_ops: field_ops_now() - ops0,
                ms: t0.elapsed().as_secs_f64() * 1e3,
                hilbert_check_ops: check_ops,
                ..r
            };
        }
        status = HilbertStop::Violated;
    }
    let r = f4_with_arithmetic(input, n_vars, p, &plain, arithmetic_mode(), TraceMode::Off);
    F4Report {
        field_ops: field_ops_now() - ops0,
        ms: t0.elapsed().as_secs_f64() * 1e3,
        hilbert: status,
        hilbert_check_ops: check_ops,
        ..r
    }
}

fn f4_with_arithmetic(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    mode: u8,
    mut trace: TraceMode<'_>,
) -> F4Report {
    let t0 = Instant::now();
    let ops0 = field_ops_now();
    // Time and field operations spent before a resumed checkpoint.
    let carried = std::cell::Cell::new(Duration::ZERO);
    let ops_carry = std::cell::Cell::new(0u64);
    let mut blocked_steps = 0usize;
    if opts.checkpoint.is_some() {
        assert!(
            matches!(trace, TraceMode::Off)
                && opts.hilbert_expect.is_none()
                && opts.stop_hilbert.is_none()
                && opts.stop_staircase.is_none()
                && !opts.capture_run_log,
            "checkpointing is not combined with traces, the Hilbert or staircase stops, or the run log"
        );
    }
    let debug = std::env::var("F4_DEBUG").is_ok();
    if debug {
        eprintln!(
            "f4: {} polys in {n_vars} vars over F_{p}, degrees {:?}",
            input.len(),
            input
                .iter()
                .map(|f| f.iter().map(|(e, _)| total_degree(e)).max().unwrap_or(0))
                .collect::<Vec<_>>()
        );
    }
    let fp = Barrett::new(p);
    let narrow = mode != 0 && p <= 1 << 16;
    let deferred = mode == 2 && narrow;
    let fp32 = narrow.then(|| Barrett32::new(p));
    let ord = opts.order;
    let mut basis: Vec<Poly> = Vec::new();
    let mut alive: Vec<bool> = Vec::new();
    let mut pairs: Vec<Pair> = Vec::new();
    let mut pairs_above_bound = 0usize;
    // Pairs over the degree bound, kept so Gebauer–Möller can retire them:
    // `pairs_above_bound` counts the ones still standing.
    let mut above: Vec<Pair> = Vec::new();
    let gm = gebauer_moeller();
    let mut packed = (matches!(ord, Ordering::Grevlex)
        && n_vars <= 15
        && opts.max_degree < 128
        && input.iter().flatten().all(|(e, _)| total_degree(e) < 128)
        && packed_enabled())
    .then(|| Packed::new(n_vars));
    let mut lm_packed: Option<Vec<u128>> = packed.as_ref().map(|_| Vec::new());
    // `learned` is (solving_degree_max, max_cols_to_solution, steps_to_solution).
    let report = |basis: &[Poly],
                  alive: &[bool],
                  inconsistent,
                  dr,
                  sd,
                  steps,
                  mr,
                  mc,
                  to,
                  pab,
                  learned: (u32, usize, usize),
                  bs: usize| {
        let b: Vec<Poly> = if inconsistent {
            vec![vec![(vec![0; n_vars], 1)]]
        } else {
            basis
                .iter()
                .zip(alive)
                .filter(|(_, &a)| a)
                .map(|(f, _)| f.clone())
                .collect()
        };
        F4Report {
            basis: b,
            inconsistent,
            degree_reached: dr,
            solving_degree: sd,
            solving_degree_max: learned.0,
            max_cols_to_solution: learned.1,
            steps_to_solution: learned.2,
            steps,
            max_rows: mr,
            max_cols: mc,
            ms: (t0.elapsed() + carried.get()).as_secs_f64() * 1e3,
            timed_out: to,
            pairs_above_bound: pab,
            staircase_at_stop: None,
            field_ops: ops_carry.get() + (field_ops_now() - ops0),
            blocked_steps: bs,
            trace_mismatch: false,
            hilbert: HilbertStop::Off,
            hilbert_check_ops: 0,
            resumed_ms: carried.get().as_secs_f64() * 1e3,
        }
    };

    // add a polynomial to the basis: pairs under the Gebauer–Möller
    // criteria (or the product criterion alone, `F4_FP_GM=0`) and the
    // degree bound; drop basis elements it makes redundant.  With `lmp`
    // (the packed leading monomials, kept in step with `basis`) every test
    // runs on packed monomials.
    #[allow(clippy::too_many_arguments)]
    fn add(
        f: Poly,
        basis: &mut Vec<Poly>,
        alive: &mut Vec<bool>,
        pairs: &mut Vec<Pair>,
        above: &mut Vec<Pair>,
        max_degree: u32,
        pairs_above_bound: &mut usize,
        gm: bool,
        lmp: Option<&mut Vec<u128>>,
    ) {
        let k = basis.len();
        let n = f[0].0.len();
        let new: Vec<(usize, Vec<u32>, u128)> = match lmp {
            Some(lmp) => {
                let h = pack(&f[0].0);
                let lms: &[u128] = lmp;
                let new = gm_update(
                    &h,
                    |i| lms[i],
                    pairs,
                    above,
                    |pr| pr.lcm_p,
                    alive,
                    k,
                    gm,
                    |a, b| packed_divides(*a, *b),
                    |a, b| packed_lcm(*a, *b),
                    |a, b| packed_coprime(*a, *b),
                );
                lmp.push(h);
                new.into_iter().map(|(i, l)| (i, unpack(l, n), l)).collect()
            }
            None => {
                let h = f[0].0.clone();
                let new = gm_update(
                    &h,
                    |i| basis[i][0].0.clone(),
                    pairs,
                    above,
                    |pr| pr.lcm.clone(),
                    alive,
                    k,
                    gm,
                    |a, b| divides(a, b),
                    |a, b| lcm(a, b),
                    |a, b| a.iter().zip(b).all(|(x, y)| *x == 0 || *y == 0),
                );
                new.into_iter().map(|(i, l)| (i, l, 0)).collect()
            }
        };
        for (i, l, lp) in new {
            let d = total_degree(&l);
            let pair = Pair {
                i,
                j: k,
                lcm: l,
                lcm_p: lp,
                degree: d,
            };
            if d > max_degree {
                above.push(pair);
            } else {
                pairs.push(pair);
            }
        }
        *pairs_above_bound = above.len();
        basis.push(f);
        alive.push(true);
    }

    for f in input {
        let f = normalise(f, p, ord);
        if f.is_empty() {
            continue;
        }
        if total_degree(&f[0].0) == 0 {
            return report(&basis, &alive, true, 0, 0, 0, 0, 0, false, 0, (0, 0, 0), 0);
        }
        add(
            f,
            &mut basis,
            &mut alive,
            &mut pairs,
            &mut above,
            opts.max_degree,
            &mut pairs_above_bound,
            gm,
            lm_packed.as_mut(),
        );
    }
    let input_lms_now: Vec<Vec<u32>> = basis.iter().map(|f| f[0].0.clone()).collect();
    let mut input_mismatch = false;
    match &mut trace {
        TraceMode::Record(t) => t.input_lms = input_lms_now,
        TraceMode::Replay(t) => {
            // other input leading monomials: not this trace's shape, a full run
            if t.input_lms != input_lms_now {
                input_mismatch = true;
            }
        }
        TraceMode::Off => {}
    }
    let mut replay_step = 0usize;
    let mut mismatch = false;
    if input_mismatch {
        mismatch = true;
        trace = TraceMode::Off;
    }
    // interreduce the input (tails included) so the first matrix is small
    let mut steps = 0usize;
    let mut max_rows = 0usize;
    let mut max_cols = 0usize;
    let mut degree_reached = 0u32;
    let mut solving_degree = 0u32;
    // (solving_degree_max, max_cols_to_solution, steps_to_solution)
    let mut learned = (0u32, 0usize, 0usize);
    let mut staircase_at_stop: Option<usize> = None;
    let mut hilbert_violated = false;
    let mut stair = StairTracker::new(n_vars);
    RUN_LOG.with(|l| *l.borrow_mut() = RunLog::default());

    let ck_fp = opts
        .checkpoint
        .as_ref()
        .map(|_| ckpt::fingerprint(input, n_vars, p, opts, mode, gm, packed.is_some()));
    let mut resumed_opts: Option<F4Options> = None;
    let mut resume_rref: Option<ckpt::RrefLoaded> = None;
    if let (Some((path, _)), Some(fp)) = (&opts.checkpoint, ck_fp) {
        if let Some(l) = ckpt::load_best(path, Some(fp)) {
            eprintln!(
                "f4: resumed {} at step {}, {:.1} s of the budget already spent, {} basis elements, {} pairs",
                path.display(),
                l.steps,
                l.elapsed.as_secs_f64(),
                l.basis.len(),
                l.pairs.len()
            );
            basis = l.basis;
            alive = l.alive;
            pairs = l.pairs;
            above = l.above;
            pairs_above_bound = l.pairs_above_bound;
            steps = l.steps;
            max_rows = l.max_rows;
            max_cols = l.max_cols;
            degree_reached = l.degree_reached;
            solving_degree = l.solving_degree;
            learned = l.learned;
            blocked_steps = l.blocked_steps;
            lm_packed = packed
                .as_ref()
                .map(|_| basis.iter().map(|f| pack(&f[0].0)).collect());
            carried.set(l.elapsed);
            ops_carry.set(l.field_ops);
            let mut spent = l.elapsed;
            // An in-step snapshot of the step that follows this checkpoint: only
            // the wide arithmetic without weights writes one, and only that
            // path reads it. The resumed step skips the reducer pass and the
            // finished part of the reduction, so its counts continue from the
            // snapshot's.
            if !deferred && opts.weights.is_none() {
                if let Some(r) = ckpt::load_rref(&ckpt::rref_path(path), fp, l.steps, l.elapsed) {
                    eprintln!(
                        "f4: in-step snapshot found: phase {}, column {}, {} pivots, {:.1} s spent, {} rows",
                        r.phase,
                        r.c,
                        r.pivots.len(),
                        r.elapsed.as_secs_f64(),
                        r.rows.len()
                    );
                    carried.set(r.elapsed);
                    ops_carry.set(r.ops);
                    spent = r.elapsed;
                    resume_rref = Some(r);
                }
            }
            // the deadline counts the time already spent
            let mut o = opts.clone();
            o.deadline = opts
                .deadline
                .map(|d| d.checked_sub(spent).unwrap_or_else(Instant::now));
            resumed_opts = Some(o);
        }
    }
    let opts = resumed_opts.as_ref().unwrap_or(opts);
    let mut last_ckpt = Instant::now();

    loop {
        if let (Some(h), false) = (&opts.hilbert_expect, hilbert_violated) {
            while let Some(d) = pairs.iter().map(|pr| pr.degree).min() {
                let lms: Vec<&[u32]> = if stair_scan() {
                    basis
                        .iter()
                        .zip(&alive)
                        .filter(|(_, &a)| a)
                        .map(|(f, _)| f[0].0.as_slice())
                        .collect()
                } else {
                    Vec::new()
                };
                let want = |k: u32| h.get(k as usize).copied().unwrap_or(0);
                let mut met = true;
                for k in 0..=d {
                    let unc = if stair_scan() {
                        uncovered_in_degree(&lms, n_vars, k)
                    } else {
                        stair.count(&basis, &alive, k)
                    };
                    if unc < want(k) {
                        // fewer standard monomials than the complete
                        // intersection allows: the certificate was wrong
                        hilbert_violated = true;
                        RUN_LOG.with(|l| l.borrow_mut().violated = true);
                        met = false;
                        break;
                    }
                    met &= unc == want(k);
                }
                if !met {
                    break;
                }
                let before = pairs.len();
                if (d as usize..h.len()).all(|k| h[k] == 0) {
                    pairs.clear();
                } else {
                    pairs.retain(|pr| pr.degree != d);
                }
                RUN_LOG.with(|l| l.borrow_mut().dropped_pairs += before - pairs.len());
            }
        }
        if let TraceMode::Replay(t) = &trace {
            if replay_step >= t.steps.len() {
                // the recorded run's end: the recorded staircase, or the
                // pairs still pending are processed as a full run
                let lms: Vec<&[u32]> = basis
                    .iter()
                    .zip(&alive)
                    .filter(|(_, &a)| a)
                    .map(|(f, _)| f[0].0.as_slice())
                    .collect();
                match (
                    t.stopped,
                    opts.stop_staircase
                        .and_then(|b| staircase_at_most(&lms, n_vars, b)),
                ) {
                    (true, Some(count)) => {
                        staircase_at_stop = Some(count);
                        break;
                    }
                    _ => {
                        mismatch = t.stopped;
                        trace = TraceMode::Off;
                        if pairs.is_empty() {
                            break;
                        }
                    }
                }
            }
        } else if pairs.is_empty() {
            break;
        }
        if debug {
            eprintln!("f4: {} pairs pending, basis {}", pairs.len(), basis.len());
        }
        // the loop-top values a checkpoint of this step's top records
        let top_el = t0.elapsed() + carried.get();
        let top_ops = ops_carry.get() + (field_ops_now() - ops0);
        let mut wrote_top = false;
        if let (Some((path, every)), Some(fp)) = (&opts.checkpoint, ck_fp) {
            let stop = opts.checkpoint_stop_after.is_some_and(|k| steps >= k);
            if stop || last_ckpt.elapsed() >= *every {
                let tw = Instant::now();
                let snap = ckpt::Snap {
                    fingerprint: fp,
                    n_vars,
                    elapsed: top_el,
                    field_ops: top_ops,
                    basis: &basis,
                    alive: &alive,
                    pairs: &pairs,
                    above: &above,
                    pairs_above_bound,
                    steps,
                    max_rows,
                    max_cols,
                    degree_reached,
                    solving_degree,
                    learned,
                    blocked_steps,
                };
                match ckpt::save(path, &snap) {
                    Ok(bytes) => {
                        wrote_top = true;
                        // a snapshot of an earlier step is stale now
                        std::fs::remove_file(ckpt::rref_path(path)).ok();
                        eprintln!(
                            "f4: checkpoint {} at step {steps}: {bytes} bytes in {:.1} s",
                            path.display(),
                            tw.elapsed().as_secs_f64()
                        )
                    }
                    Err(e) => eprintln!("f4: CHECKPOINT WRITE FAILED {}: {e}", path.display()),
                }
                last_ckpt = Instant::now();
                if stop {
                    return report(
                        &basis,
                        &alive,
                        false,
                        degree_reached,
                        solving_degree,
                        steps,
                        max_rows,
                        max_cols,
                        true,
                        pairs_above_bound,
                        learned,
                        blocked_steps,
                    );
                }
            }
        }
        if let Some(d) = opts.deadline {
            if Instant::now() > d {
                return report(
                    &basis,
                    &alive,
                    false,
                    degree_reached,
                    solving_degree,
                    steps,
                    max_rows,
                    max_cols,
                    true,
                    pairs_above_bound,
                    learned,
                    blocked_steps,
                );
            }
        }
        let ops_step0 = opts.capture_run_log.then(field_ops_now);
        let tp0 = Instant::now();
        // The step-top state an in-step snapshot needs: the pending pairs before
        // this step takes its own out, and the counters before it changes them.
        let top: Option<StepTop> = (opts.checkpoint.is_some()
            && opts.checkpoint_in_step.is_some()
            && opts.max_pairs_per_step.is_none())
        .then(|| StepTop {
            pairs: pairs.clone(),
            steps,
            max_rows,
            max_cols,
            degree_reached,
            solving_degree,
            learned,
            blocked_steps,
            elapsed: top_el,
            ops: top_ops,
        });
        let resume_here = resume_rref.take();
        let mut top_written = wrote_top;
        let mut last_snap = Instant::now();
        let mut snaps = 0usize;
        // symbolic preprocessing
        let mut rows_poly: Vec<(usize, Vec<u32>)> = Vec::new(); // (basis index, multiplier)
        let mut seen: FxSet<(usize, Vec<u32>)> = FxSet::default();
        let d;
        let mut selected_len = 0usize;
        if let TraceMode::Replay(t) = &trace {
            // the recorded useful rows of this step stand in for the pairs
            let step = &t.steps[replay_step];
            replay_step += 1;
            d = step.degree;
            degree_reached = degree_reached.max(d);
            // the pairs stay as `add` accumulates them: a replay that diverges
            // (or ends off the recorded staircase) finishes as a full run
            // over all of them
            for (idx, m) in &step.rows {
                if *idx >= basis.len() {
                    let mut r = report(
                        &basis,
                        &alive,
                        false,
                        degree_reached,
                        solving_degree,
                        steps,
                        max_rows,
                        max_cols,
                        false,
                        pairs_above_bound,
                        learned,
                        blocked_steps,
                    );
                    r.trace_mismatch = true;
                    return r;
                }
                if seen.insert((*idx, m.clone())) {
                    rows_poly.push((*idx, m.clone()));
                }
            }
        } else {
            d = pairs.iter().map(|pr| pr.degree).min().unwrap();
            degree_reached = degree_reached.max(d);
            let (selected, rest): (Vec<Pair>, Vec<Pair>) =
                pairs.drain(..).partition(|pr| pr.degree == d);
            pairs = rest;
            let mut selected = selected;
            if let Some(c) = opts.max_pairs_per_step {
                if selected.len() > c {
                    selected.sort_by(|a, b| cmp_monomial(&a.lcm, &b.lcm, ord));
                    pairs.extend(selected.split_off(c));
                }
            }
            selected_len = selected.len();
            for pr in &selected {
                for &idx in &[pr.i, pr.j] {
                    let m: Vec<u32> = pr
                        .lcm
                        .iter()
                        .zip(&basis[idx][0].0)
                        .map(|(a, b)| a - b)
                        .collect();
                    if seen.insert((idx, m.clone())) {
                        rows_poly.push((idx, m));
                    }
                }
            }
        }
        let n_pair_rows = rows_poly.len();
        let (cols, sparse, tp1): (Vec<Vec<u32>>, Vec<Vec<(u32, u64)>>, Instant) = if let Some(pk) =
            packed.as_mut()
        {
            pk.extend(&basis);
            let pk = &*pk;
            let (extra, cols_packed, row_cols) =
                if symbolic_parallel() && rayon::current_num_threads() > 1 && rows_poly.len() >= 64
                {
                    pk.symbolic_par(&rows_poly, &alive)
                } else {
                    pk.symbolic(&rows_poly, &alive)
                };
            let tp1 = Instant::now();
            let idxs: Vec<usize> = rows_poly
                .iter()
                .map(|(i, _)| *i)
                .chain(extra.iter().map(|&(i, _)| i))
                .collect();
            rows_poly.extend(extra.iter().map(|&(i, m)| (i, unpack(m, n_vars))));
            // Small steps (most of them, and all of `solve`'s specialised
            // runs) are cheaper on one thread than handed to the pool: the
            // hand-off wakes a worker and waits for it, which costs more than
            // building a few short rows.
            let to_sparse = |(rc, &idx): (Vec<u32>, &usize)| -> Vec<(u32, u64)> {
                rc.into_iter()
                    .zip(&basis[idx])
                    .map(|(c, (_, v))| (c, *v))
                    .collect()
            };
            let sparse: Vec<Vec<(u32, u64)>> = if row_cols.len() * cols_packed.len() > PAR_WORK {
                row_cols.into_par_iter().zip(&idxs).map(to_sparse).collect()
            } else {
                row_cols.into_iter().zip(&idxs).map(to_sparse).collect()
            };
            let cols = cols_packed.iter().map(|&m| unpack(m, n_vars)).collect();
            (cols, sparse, tp1)
        } else {
            let mut monomials: FxSet<Vec<u32>> = FxSet::default();
            let mut done: FxSet<Vec<u32>> = FxSet::default();
            let mut queue: Vec<Vec<u32>> = Vec::new();
            // One scratch monomial; a heap copy only for a monomial not seen.
            let mut em: Vec<u32> = vec![0; n_vars];
            for (idx, m) in &rows_poly {
                for (e, _) in &basis[*idx] {
                    for (k, x) in em.iter_mut().enumerate() {
                        *x = e[k] + m[k];
                    }
                    if !monomials.contains(em.as_slice()) {
                        monomials.insert(em.clone());
                        queue.push(em.clone());
                    }
                }
                let lm: Vec<u32> = basis[*idx][0].0.iter().zip(m).map(|(x, y)| x + y).collect();
                done.insert(lm);
            }
            while let Some(mono) = queue.pop() {
                if done.contains(&mono) {
                    continue;
                }
                done.insert(mono.clone());
                // one reducer: the alive basis element with the largest leading
                // monomial dividing it (any would do)
                if let Some(idx) =
                    (0..basis.len()).find(|&i| alive[i] && divides(&basis[i][0].0, &mono))
                {
                    let m: Vec<u32> = mono
                        .iter()
                        .zip(&basis[idx][0].0)
                        .map(|(a, b)| a - b)
                        .collect();
                    if seen.insert((idx, m.clone())) {
                        for (e, _) in &basis[idx] {
                            for (k, x) in em.iter_mut().enumerate() {
                                *x = e[k] + m[k];
                            }
                            if !monomials.contains(em.as_slice()) {
                                monomials.insert(em.clone());
                                queue.push(em.clone());
                            }
                        }
                        rows_poly.push((idx, m));
                    }
                }
            }
            let tp1 = Instant::now();
            // matrix: columns in decreasing monomial order, rows sparse
            let mut cols: Vec<Vec<u32>> = monomials.into_iter().collect();
            cols.sort_unstable_by(|a, b| cmp_monomial(b, a, ord));
            let col_of: FxMap<&[u32], u32> = cols
                .iter()
                .enumerate()
                .map(|(i, m)| (m.as_slice(), i as u32))
                .collect();
            let n_cols = cols.len();
            // `(column, coefficient)` in increasing column order, so the
            // leading term first; every row is monic
            let to_sparse = |(idx, m): &(usize, Vec<u32>)| -> Vec<(u32, u64)> {
                let mut em = vec![0u32; n_vars];
                basis[*idx]
                    .iter()
                    .map(|(e, c)| {
                        for (k, x) in em.iter_mut().enumerate() {
                            *x = e[k] + m[k];
                        }
                        (col_of[em.as_slice()], *c)
                    })
                    .collect()
            };
            // small steps (most of them, and all of `solve`'s specialised
            // runs) are cheaper on one thread than dispatched
            let parallel = rows_poly.len() * n_cols > PAR_WORK;
            let sparse: Vec<Vec<(u32, u64)>> = if parallel {
                rows_poly.par_iter().map(to_sparse).collect()
            } else {
                rows_poly.iter().map(to_sparse).collect()
            };
            (cols, sparse, tp1)
        };
        let n_cols = cols.len();
        let parallel = rows_poly.len() * n_cols > PAR_WORK;
        let tp2 = Instant::now();
        let mut input_lm = vec![false; n_cols];
        for r in &sparse {
            input_lm[r[0].0 as usize] = true;
        }
        max_rows = max_rows.max(sparse.len());
        max_cols = max_cols.max(n_cols);
        notify_progress(F4Progress {
            event: "matrix",
            variables: n_vars,
            step: steps,
            degree: d,
            rows: sparse.len(),
            columns: n_cols,
            productive_degree: learned.0,
            field_multiplications: field_ops_now() - ops0,
            elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
        });
        steps += 1;
        // bitop/v1 (`jv_bitops`, off unless a study turns it on): symbolic
        // preprocessing and the matrix build, charged per entry of this
        // step's matrix (the monomial product, two words; the monomial-set
        // probe and the column-index probe), per column (the done-set probe,
        // the reducer search over the basis at its bound, the column sort),
        // and the pair rows' scans over the reducer columns
        if super::jv_bitops::enabled() {
            let entries: u64 = sparse.iter().map(|r| r.len() as u64).sum();
            let nc = n_cols as u64;
            super::jv_bitops::probes(2 * entries + nc, 2);
            super::jv_bitops::words(
                2 * entries + super::jv_bitops::sort_cost(n_cols) + nc * basis.len() as u64,
            );
            let scans: u64 = sparse[..n_pair_rows]
                .iter()
                .map(|r| r.first().map_or(0, |e| nc - e.0 as u64))
                .sum();
            super::jv_bitops::words(scans);
        }
        // The reducer rows of symbolic preprocessing have distinct leading
        // columns, none of them an lcm: they are already an echelon form.
        // Reduce each pair row by them alone, column by column, then take
        // the reduced echelon form of what is left, which lives on the
        // other columns only.  Its rows are exactly the rows of the whole
        // matrix's reduced echelon form whose pivot is not a reducer's
        // leading monomial — the only ones that can be new.
        let (pair_rows, reducers) = sparse.split_at(n_pair_rows);
        const NONE: u32 = u32::MAX;
        let mut reducer_at = vec![NONE; n_cols];
        for (k, r) in reducers.iter().enumerate() {
            reducer_at[r[0].0 as usize] = k as u32;
        }
        let free_cols: Vec<usize> = (0..n_cols).filter(|&c| reducer_at[c] == NONE).collect();
        let deadline = opts.deadline;
        // Block-aware reduction: with every row in one weight class the
        // reduced pair rows split by class over the free columns.
        let block_classes: Option<Vec<u8>> = opts.weights.as_ref().and_then(|w| {
            let col_w: Vec<u8> = cols.iter().map(|m| z3_weight(m, w)).collect();
            let homogeneous = sparse.iter().all(|r| {
                let cw = col_w[r[0].0 as usize];
                r.iter().all(|&(c, _)| col_w[c as usize] == cw)
            });
            homogeneous.then(|| free_cols.iter().map(|&c| col_w[c]).collect())
        });
        if block_classes.is_some() {
            blocked_steps += 1;
        }
        // which pair rows survived the reducers, and (when recording) their
        // reduced rows before the echelon form, to find the contributors of
        // every new pivot
        let mut useful: Vec<bool> = Vec::new();
        let recording = matches!(trace, TraceMode::Record(_));
        let mut rec_rows: Vec<Vec<u64>> = Vec::new();
        let pivots: Option<(Vec<Vec<u64>>, Vec<usize>)> = if deferred {
            let fp32 = fp32.expect("deferred mode has narrow arithmetic");
            let reduce_pair_row = |r: &Vec<(u32, u64)>| -> Option<Vec<u32>> {
                if deadline.is_some_and(|d| Instant::now() > d) {
                    return None;
                }
                let mut acc = vec![0u64; n_cols];
                for &(c, v) in r {
                    acc[c as usize] = v;
                }
                let mut pending = 0usize;
                for c in r[0].0 as usize..n_cols {
                    if reducer_at[c] == NONE {
                        continue;
                    }
                    let f = fp.reduce(acc[c]);
                    if f == 0 {
                        continue;
                    }
                    acc[c] = 0;
                    let neg = p - f;
                    let reducer = &reducers[reducer_at[c] as usize][1..];
                    for &(cc, v) in reducer {
                        acc[cc as usize] += neg * v;
                    }
                    count_ops(reducer.len());
                    pending += 1;
                    if pending == 256 {
                        normalize_lazy(&mut acc, fp);
                        pending = 0;
                    }
                }
                Some(
                    free_cols
                        .iter()
                        .map(|&c| fp.reduce(acc[c]) as u32)
                        .collect(),
                )
            };
            // Faugère–Lachartre: reduce the reducer block once instead of
            // re-running its cascade for every pair row.  With the
            // reducers `A | B` (`A` on their own leading columns, `B` on the
            // free columns) brought to `B' = A⁻¹B`, each pair row `C | D`
            // becomes `D − C·B'`: one dense update over the free columns per
            // original entry at a reducer column, and no cascade, because
            // `B'` has no entries on reducer columns.  Rows are monic, so
            // `A` has a unit diagonal.  `B'` is built rightmost lead first
            // in blocks: a block's rows are reduced in parallel against the
            // blocks before it, then finished in order against each other.
            // Tiny steps (most of `solve`'s specialised runs) keep the
            // cascade: building `B'` there costs more than it saves.
            let fl = fl_enabled()
                && n_cols >= FL_MIN_COLS
                && reducers.len().saturating_mul(free_cols.len()) <= FL_MAX_ENTRIES;
            let reduced: Option<Vec<Vec<u32>>> = if fl {
                reduce_pairs_fl(
                    pair_rows,
                    reducers,
                    &reducer_at,
                    &free_cols,
                    n_cols,
                    fp,
                    deadline,
                )
            } else if parallel {
                pair_rows.par_iter().map(reduce_pair_row).collect()
            } else {
                pair_rows.iter().map(reduce_pair_row).collect()
            };
            reduced.and_then(|mut rows| {
                useful = rows.iter().map(|r| r.iter().any(|&v| v != 0)).collect();
                rows.retain(|r| r.iter().any(|&v| v != 0));
                if recording {
                    rec_rows = rows
                        .iter()
                        .map(|r| r.iter().map(|&v| v as u64).collect())
                        .collect();
                }
                let piv = match &block_classes {
                    Some(cls) => {
                        rref_blocked(&mut rows, cls, |b| rref32_deferred(b, fp32, fp, deadline))
                    }
                    None => rref32_deferred(&mut rows, fp32, fp, deadline),
                };
                piv.map(|piv| {
                    let wide = rows
                        .into_iter()
                        .map(|row| row.into_iter().map(u64::from).collect())
                        .collect();
                    (wide, piv)
                })
            })
        } else if let Some(fp32) = fp32 {
            let reduce_pair_row = |r: &Vec<(u32, u64)>| -> Option<Vec<u32>> {
                if deadline.is_some_and(|d| Instant::now() > d) {
                    return None;
                }
                let mut acc = vec![0u32; n_cols];
                for &(c, v) in r {
                    acc[c as usize] = v as u32;
                }
                for c in r[0].0 as usize..n_cols {
                    let f = acc[c];
                    if f == 0 || reducer_at[c] == NONE {
                        continue;
                    }
                    acc[c] = 0;
                    let neg = fp32.p - f;
                    let reducer = &reducers[reducer_at[c] as usize][1..];
                    for &(cc, v) in reducer {
                        let x = &mut acc[cc as usize];
                        *x = fp32.reduce(*x + neg * v as u32);
                    }
                    count_ops(reducer.len());
                }
                Some(free_cols.iter().map(|&c| acc[c]).collect())
            };
            let reduced: Option<Vec<Vec<u32>>> = if parallel {
                pair_rows.par_iter().map(reduce_pair_row).collect()
            } else {
                pair_rows.iter().map(reduce_pair_row).collect()
            };
            reduced.and_then(|mut rows| {
                useful = rows.iter().map(|r| r.iter().any(|&v| v != 0)).collect();
                rows.retain(|r| r.iter().any(|&v| v != 0));
                if recording {
                    rec_rows = rows
                        .iter()
                        .map(|r| r.iter().map(|&v| v as u64).collect())
                        .collect();
                }
                let piv = match &block_classes {
                    Some(cls) => rref_blocked(&mut rows, cls, |b| rref32(b, fp32, deadline)),
                    None => rref32(&mut rows, fp32, deadline),
                };
                piv.map(|piv| {
                    let wide = rows
                        .into_iter()
                        .map(|row| row.into_iter().map(u64::from).collect())
                        .collect();
                    (wide, piv)
                })
            })
        } else {
            let reduce_pair_row = |r: &Vec<(u32, u64)>| -> Option<Vec<u64>> {
                if deadline.is_some_and(|d| Instant::now() > d) {
                    return None;
                }
                let mut acc = vec![0u64; n_cols];
                for &(c, v) in r {
                    acc[c as usize] = v;
                }
                for c in r[0].0 as usize..n_cols {
                    let f = acc[c];
                    if f == 0 || reducer_at[c] == NONE {
                        continue;
                    }
                    acc[c] = 0;
                    let neg = p - f;
                    let reducer = &reducers[reducer_at[c] as usize][1..];
                    for &(cc, v) in reducer {
                        let x = &mut acc[cc as usize];
                        *x = fp.reduce(*x + neg * v);
                    }
                    count_ops(reducer.len());
                }
                Some(free_cols.iter().map(|&c| acc[c]).collect())
            };
            // an in-step snapshot stands in for the reducer pass and the finished
            // part of the reduction
            let reduced: Option<Vec<Vec<u64>>> = if resume_here.is_some() {
                Some(Vec::new())
            } else if parallel {
                pair_rows.par_iter().map(reduce_pair_row).collect()
            } else {
                pair_rows.iter().map(reduce_pair_row).collect()
            };
            let mut snapshot_hook = |v: &RrefView| -> bool {
                let (Some((path, _)), Some(top), Some(every), Some(fp)) =
                    (&opts.checkpoint, &top, opts.checkpoint_in_step, ck_fp)
                else {
                    return false;
                };
                if last_snap.elapsed() < every {
                    return false;
                }
                let tw = Instant::now();
                if !top_written {
                    let snap = ckpt::Snap {
                        fingerprint: fp,
                        n_vars,
                        elapsed: top.elapsed,
                        field_ops: top.ops,
                        basis: &basis,
                        alive: &alive,
                        pairs: &top.pairs,
                        above: &above,
                        pairs_above_bound,
                        steps: top.steps,
                        max_rows: top.max_rows,
                        max_cols: top.max_cols,
                        degree_reached: top.degree_reached,
                        solving_degree: top.solving_degree,
                        learned: top.learned,
                        blocked_steps: top.blocked_steps,
                    };
                    match ckpt::save(path, &snap) {
                        Ok(bytes) => {
                            top_written = true;
                            eprintln!(
                                "f4: checkpoint {} at the top of step {}, written inside it: {bytes} bytes in {:.1} s",
                                path.display(),
                                top.steps,
                                tw.elapsed().as_secs_f64()
                            );
                        }
                        Err(e) => {
                            eprintln!("f4: CHECKPOINT WRITE FAILED {}: {e}", path.display());
                            last_snap = Instant::now();
                            return false;
                        }
                    }
                }
                let tr = Instant::now();
                let rs = ckpt::RrefSnap {
                    fingerprint: fp,
                    top_steps: top.steps,
                    top_elapsed: top.elapsed,
                    ops: ops_carry.get() + (field_ops_now() - ops0),
                    elapsed: t0.elapsed() + carried.get(),
                    phase: v.phase,
                    c: v.c,
                    pr: v.pr,
                    i: v.i,
                    pivots: v.pivots,
                    rows: v.rows,
                };
                match ckpt::save_rref(&ckpt::rref_path(path), &rs) {
                    Ok(bytes) => eprintln!(
                        "f4: in-step snapshot of step {} (phase {}, column {}, {} pivots): {bytes} bytes in {:.1} s",
                        top.steps + 1,
                        v.phase,
                        v.c,
                        v.pivots.len(),
                        tr.elapsed().as_secs_f64()
                    ),
                    Err(e) => eprintln!("f4: IN-STEP SNAPSHOT FAILED: {e}"),
                }
                last_snap = Instant::now();
                snaps += 1;
                opts.checkpoint_stop_in_rref.is_some_and(|k| snaps >= k)
            };
            reduced.and_then(|mut rows| {
                let mut resume_arg = None;
                if let Some(rs) = resume_here {
                    rows = rs.rows;
                    resume_arg = Some((rs.phase, rs.c, rs.pr, rs.i, rs.pivots));
                } else {
                    useful = rows.iter().map(|r| r.iter().any(|&v| v != 0)).collect();
                    rows.retain(|r| r.iter().any(|&v| v != 0));
                    if recording {
                        rec_rows = rows.iter().map(|r| r.to_vec()).collect();
                    }
                }
                let piv = match &block_classes {
                    Some(cls) => rref_blocked(&mut rows, cls, |b| rref(b, fp, deadline)),
                    None => {
                        let hook: Option<&mut dyn FnMut(&RrefView) -> bool> =
                            if top.is_some() && opts.checkpoint.is_some() {
                                Some(&mut snapshot_hook)
                            } else {
                                None
                            };
                        rref_ckpt(&mut rows, fp, deadline, resume_arg, hook)
                    }
                };
                piv.map(|piv| (rows, piv))
            })
        };
        if debug {
            eprintln!(
                "f4 step {steps}: degree {d}, {} pairs, {} rows × {n_cols} cols, {} pivots, basis {}",
                selected_len,
                sparse.len(),
                reducers.len() + pivots.as_ref().map_or(0, |(_, pv)| pv.len()),
                basis.len()
            );
        }
        let Some((rows, pivots)) = pivots else {
            return report(
                &basis,
                &alive,
                false,
                degree_reached,
                solving_degree,
                steps,
                max_rows,
                max_cols,
                true,
                pairs_above_bound,
                learned,
                blocked_steps,
            );
        };
        let tp3 = Instant::now();
        // new polynomials: rows whose pivot column is not an input leading monomial
        let mut new_polys: Vec<Poly> = Vec::new();
        for (row, &c) in rows.iter().zip(&pivots) {
            if input_lm[free_cols[c]] {
                continue;
            }
            let poly: Poly = row
                .iter()
                .enumerate()
                .filter(|(_, &v)| v != 0)
                .map(|(j, &v)| (cols[free_cols[j]].clone(), v))
                .collect();
            if total_degree(&poly[0].0) == 0 {
                return report(
                    &basis,
                    &alive,
                    true,
                    degree_reached,
                    d,
                    steps,
                    max_rows,
                    max_cols,
                    false,
                    pairs_above_bound,
                    (learned.0.max(d), max_cols, steps),
                    blocked_steps,
                );
            }
            new_polys.push(poly);
        }
        if !new_polys.is_empty() {
            solving_degree = d;
            learned = (learned.0.max(d), max_cols, steps);
        }
        let learned_now = !new_polys.is_empty();
        let learned_new = new_polys.len();
        match &mut trace {
            TraceMode::Record(t) => {
                // the pair rows that the new pivots are combinations of
                let retained: Vec<usize> = (0..n_pair_rows).filter(|&i| useful[i]).collect();
                let mut needed = vec![false; retained.len()];
                for (col, contributors) in tracked_rref_contributors(&rec_rows, p) {
                    if !input_lm[free_cols[col]] {
                        for i in contributors {
                            needed[i] = true;
                        }
                    }
                }
                t.steps.push(TraceStep {
                    degree: d,
                    rows: retained
                        .iter()
                        .zip(&needed)
                        .filter(|(_, &nd)| nd)
                        .map(|(&i, _)| rows_poly[i].clone())
                        .collect(),
                    new_lms: new_polys.iter().map(|f| f[0].0.clone()).collect(),
                });
            }
            TraceMode::Replay(t) => {
                let expected = &t.steps[replay_step - 1].new_lms;
                let found: Vec<&Vec<u32>> = new_polys.iter().map(|f| &f[0].0).collect();
                if found.len() != expected.len() || found.iter().zip(expected).any(|(a, b)| *a != b)
                {
                    // the shapes diverged: keep what this step found (it is
                    // in the ideal) and finish as a full run over the pairs
                    // the basis has accumulated
                    mismatch = true;
                    trace = TraceMode::Off;
                }
            }
            TraceMode::Off => {}
        }
        for f in new_polys {
            add(
                f,
                &mut basis,
                &mut alive,
                &mut pairs,
                &mut above,
                opts.max_degree,
                &mut pairs_above_bound,
                gm,
                lm_packed.as_mut(),
            );
        }
        if let Some(ops_step0) = ops_step0 {
            let lms: Vec<&[u32]> = basis
                .iter()
                .zip(&alive)
                .filter(|(_, &a)| a)
                .map(|(f, _)| f[0].0.as_slice())
                .collect();
            let sc = staircase_at_most(&lms, n_vars, usize::MAX).unwrap_or(0);
            RUN_LOG.with(|l| {
                l.borrow_mut().steps.push(StepStat {
                    degree: d,
                    pairs: selected_len,
                    rows: n_pair_rows,
                    all_rows: sparse.len(),
                    ops: field_ops_now() - ops_step0,
                    new_polys: learned_new,
                    staircase: sc,
                })
            });
        }
        if debug {
            eprintln!(
                "f4 step {steps} ms: symbolic {:.2} matrix {:.2} reduce {:.2} update {:.2}",
                (tp1 - tp0).as_secs_f64() * 1e3,
                (tp2 - tp1).as_secs_f64() * 1e3,
                (tp3 - tp2).as_secs_f64() * 1e3,
                tp3.elapsed().as_secs_f64() * 1e3
            );
        }
        if let (Some(bound), true) = (opts.stop_staircase, learned_now && !pairs.is_empty()) {
            let lms: Vec<&[u32]> = basis
                .iter()
                .zip(&alive)
                .filter(|(_, &a)| a)
                .map(|(f, _)| f[0].0.as_slice())
                .collect();
            if let Some(count) = staircase_at_most(&lms, n_vars, bound) {
                staircase_at_stop = Some(count);
                if let TraceMode::Record(t) = &mut trace {
                    t.stopped = true;
                }
                break;
            }
        }
    }
    // final interreduction of the alive basis (tails)
    let mut reduced: Vec<Poly> = basis
        .iter()
        .zip(&alive)
        .filter(|(_, &a)| a)
        .map(|(f, _)| f.clone())
        .collect();
    let ti = Instant::now();
    reduced = interreduce_linear(&reduced, n_vars, p, ord)
        .unwrap_or_else(|| interreduce(&reduced, p, ord));
    if debug {
        eprintln!("f4 interreduce ms: {:.2}", ti.elapsed().as_secs_f64() * 1e3);
    }
    F4Report {
        basis: reduced,
        inconsistent: false,
        degree_reached,
        solving_degree,
        solving_degree_max: learned.0,
        max_cols_to_solution: learned.1,
        steps_to_solution: learned.2,
        steps,
        max_rows,
        max_cols,
        ms: (t0.elapsed() + carried.get()).as_secs_f64() * 1e3,
        timed_out: false,
        pairs_above_bound,
        staircase_at_stop,
        field_ops: ops_carry.get() + (field_ops_now() - ops0),
        blocked_steps,
        trace_mismatch: mismatch,
        hilbert: if opts.hilbert_expect.is_some() {
            if hilbert_violated {
                HilbertStop::Violated
            } else {
                HilbertStop::Applied
            }
        } else {
            HilbertStop::Off
        },
        hilbert_check_ops: 0,
        resumed_ms: carried.get().as_secs_f64() * 1e3,
    }
}

/// Whether the final interreduction runs as one matrix step
/// ([`interreduce_linear`]); `F4_FP_LINEAR_INTERREDUCE=0` keeps the
/// polynomial-by-polynomial [`interreduce`] as a same-binary control.
fn linear_interreduce_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("F4_FP_LINEAR_INTERREDUCE").as_deref() != Ok("0"))
}

/// [`interreduce`] as one F4 step: the minimal basis and, from symbolic
/// preprocessing, a reducer for every non-leading monomial some leading
/// monomial divides are already an echelon form (every row monic, leading
/// columns distinct), so the Faugère–Lachartre reduction of the basis rows
/// by the reducers followed by the reduced echelon form of what remains
/// is the reduced Gröbner basis.  On `quad_n9_p65521` the polynomial
/// interreduction was 43 % of the whole run.  `None` outside the narrow
/// deferred arithmetic (`p ≤ 2¹⁶`), where the caller interreduces as before.
fn interreduce_linear(basis: &[Poly], n_vars: usize, p: u64, ord: Ordering) -> Option<Vec<Poly>> {
    if !linear_interreduce_enabled() || p > 1 << 16 || arithmetic_mode() != 2 || basis.is_empty() {
        return None;
    }
    // the same minimal subset as `interreduce`
    let keep: Vec<&Poly> = basis
        .iter()
        .enumerate()
        .filter(|&(i, f)| {
            !basis
                .iter()
                .enumerate()
                .any(|(j, g)| j != i && divides(&g[0].0, &f[0].0) && (g[0].0 != f[0].0 || j < i))
        })
        .map(|(_, f)| f)
        .collect();
    let mut rows_poly: Vec<(usize, Vec<u32>)> =
        (0..keep.len()).map(|i| (i, vec![0u32; n_vars])).collect();
    let mut seen: FxSet<(usize, Vec<u32>)> = rows_poly.iter().cloned().collect();
    let mut monomials: FxSet<Vec<u32>> = FxSet::default();
    let mut done: FxSet<Vec<u32>> = keep.iter().map(|f| f[0].0.clone()).collect();
    let mut queue: Vec<Vec<u32>> = Vec::new();
    for f in &keep {
        for (e, _) in f.iter() {
            if monomials.insert(e.clone()) {
                queue.push(e.clone());
            }
        }
    }
    let mut em: Vec<u32> = vec![0; n_vars];
    while let Some(mono) = queue.pop() {
        if !done.insert(mono.clone()) {
            continue;
        }
        if let Some(idx) = (0..keep.len()).find(|&i| divides(&keep[i][0].0, &mono)) {
            let m: Vec<u32> = mono
                .iter()
                .zip(&keep[idx][0].0)
                .map(|(a, b)| a - b)
                .collect();
            if seen.insert((idx, m.clone())) {
                for (e, _) in keep[idx].iter() {
                    for (k, x) in em.iter_mut().enumerate() {
                        *x = e[k] + m[k];
                    }
                    if !monomials.contains(em.as_slice()) {
                        monomials.insert(em.clone());
                        queue.push(em.clone());
                    }
                }
                rows_poly.push((idx, m));
            }
        }
    }
    let mut cols: Vec<Vec<u32>> = monomials.into_iter().collect();
    cols.sort_unstable_by(|a, b| cmp_monomial(b, a, ord));
    let col_of: FxMap<&[u32], u32> = cols
        .iter()
        .enumerate()
        .map(|(i, m)| (m.as_slice(), i as u32))
        .collect();
    let n_cols = cols.len();
    let sparse: Vec<Vec<(u32, u64)>> = rows_poly
        .iter()
        .map(|(idx, m)| {
            let mut em = vec![0u32; n_vars];
            keep[*idx]
                .iter()
                .map(|(e, c)| {
                    for (k, x) in em.iter_mut().enumerate() {
                        *x = e[k] + m[k];
                    }
                    (col_of[em.as_slice()], *c)
                })
                .collect()
        })
        .collect();
    let (pair_rows, reducers) = sparse.split_at(keep.len());
    const NONE: u32 = u32::MAX;
    let mut reducer_at = vec![NONE; n_cols];
    for (k, r) in reducers.iter().enumerate() {
        reducer_at[r[0].0 as usize] = k as u32;
    }
    let free_cols: Vec<usize> = (0..n_cols).filter(|&c| reducer_at[c] == NONE).collect();
    if reducers.len().saturating_mul(free_cols.len()) > FL_MAX_ENTRIES {
        return None;
    }
    let fp = Barrett::new(p);
    let mut rows = reduce_pairs_fl(
        pair_rows,
        reducers,
        &reducer_at,
        &free_cols,
        n_cols,
        fp,
        None,
    )?;
    let pivots = rref32_deferred(&mut rows, Barrett32::new(p), fp, None)?;
    if pivots.len() != keep.len() {
        return None; // not full rank: cannot happen for a minimal basis
    }
    let mut out: Vec<Poly> = rows
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, &v)| v != 0)
                .map(|(j, &v)| (cols[free_cols[j]].clone(), v as u64))
                .collect()
        })
        .collect();
    out.sort_by(|a, b| cmp_monomial(&a[0].0, &b[0].0, ord));
    Some(out)
}

/// A monomial ordered by `cmp_monomial` under a fixed ordering, for the
/// work heap of [`reduce`].
struct Keyed<'a>(Vec<u32>, &'a Ordering);

impl PartialEq for Keyed<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Keyed<'_> {}
impl PartialOrd for Keyed<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Keyed<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        cmp_monomial(&self.0, &other.0, *self.1)
    }
}

/// Reduce `f` by `basis` (full reduction of every term).
pub fn reduce(f: &Poly, basis: &[Poly], p: u64, ord: Ordering) -> Poly {
    reduce_skipping_vecs(f, basis, usize::MAX, p, ord)
}

/// [`reduce`] by every element of `basis` except `basis[skip]`, on exponent
/// vectors.
fn reduce_skipping_vecs(f: &Poly, basis: &[Poly], skip: usize, p: u64, ord: Ordering) -> Poly {
    // coefficients of the pending terms, and a max-heap of their
    // monomials; a reduction only adds monomials below the one it
    // removes, so each monomial leaves the heap once
    let mut work: FxMap<Vec<u32>, u64> = FxMap::default();
    let mut heap: std::collections::BinaryHeap<Keyed> = std::collections::BinaryHeap::new();
    for (e, c) in f {
        let v = work.entry(e.clone()).or_insert_with(|| {
            heap.push(Keyed(e.clone(), &ord));
            0
        });
        *v = (*v + c % p) % p;
    }
    let mut out: Vec<(Vec<u32>, u64)> = Vec::new();
    while let Some(Keyed(mono, _)) = heap.pop() {
        let Some(c) = work.remove(&mono) else {
            continue;
        };
        if c == 0 {
            continue;
        }
        let g = basis
            .iter()
            .enumerate()
            .find(|&(j, g)| j != skip && divides(&g[0].0, &mono));
        if let Some((_, g)) = g {
            // `g` is monic: `mono − c·m·g` cancels the leading term (already
            // removed from `work`) and adds the shifted tail
            let neg = p - c;
            for (e, gc) in &g[1..] {
                let em: Vec<u32> = e
                    .iter()
                    .zip(&mono)
                    .zip(&g[0].0)
                    .map(|((x, a), b)| x + a - b)
                    .collect();
                let v = work.entry(em).or_insert_with_key(|k| {
                    heap.push(Keyed(k.clone(), &ord));
                    0
                });
                *v = (*v + mulmod(neg, *gc, p)) % p;
                super::jv_bitops::row_ops(1);
                super::jv_bitops::probes(1, 2);
            }
        } else {
            out.push((mono, c));
        }
    }
    // popped in decreasing order: already leading-first
    out
}

// ── Reducing a set on packed monomials ─────────────────────────────
//
// `interreduce` and `autoreduce` reduce every element of a set by the others
// again and again; on exponent vectors each reduction allocates a vector per
// term, hashes it and compares it element by element.  With at most 16
// unknowns and every exponent at most 127 a monomial is one `u128` (a byte
// per exponent, as in `Packed`) and the same walk is a few word operations.
// The terms popped, the divisors chosen and the coefficients that come out
// are those of `reduce_skipping_vecs`; whatever does not pack is handed back
// to it.

/// The most unknowns a monomial of this section holds.
const SET_PACK_VARS: usize = 16;

/// The largest exponent it holds.  The top bit of every byte stays clear,
/// which is what lets `packed_divides` test all the exponents at once and
/// `set_mul` see an exponent leave the range.
const SET_PACK_EXP: u32 = 127;

/// A leading monomial that divides nothing: its byte 0 is 128, which no
/// packed monomial reaches.  It stands for a set element whose *leading*
/// monomial does not pack, which divides no monomial that does.
const SET_NEVER: u128 = 0x80;

fn set_pack(e: &[u32]) -> Option<u128> {
    if e.len() > SET_PACK_VARS {
        return None;
    }
    let mut x = 0u128;
    for (k, &v) in e.iter().enumerate() {
        if v > SET_PACK_EXP {
            return None;
        }
        x |= u128::from(v) << (8 * k);
    }
    Some(x)
}

/// The monomials of `f` packed, if each has `n_vars` exponents and all pack.
fn set_pack_poly(f: &Poly, n_vars: usize) -> Option<Vec<u128>> {
    if f.iter().any(|(e, _)| e.len() != n_vars) {
        return None;
    }
    f.iter().map(|(e, _)| set_pack(e)).collect()
}

/// The packed leading monomial of `f`, or [`SET_NEVER`] when it does not
/// pack (or `f` has no terms).
///
/// It is kept apart from [`set_pack_poly`] on purpose.  A polynomial can
/// have a lead that packs and a tail that does not (under lex a tail can
/// carry a bigger exponent than the lead; any order can run past degree 127
/// in the tail), and then it still divides the monomials its lead divides,
/// as it does for the exponent-vector code.  The caller finds it as the
/// divisor it is and, having no packed terms to multiply, gives the
/// reduction up to the exponent-vector code; passing it by, as if its lead
/// did not pack, would change what is reduced.
fn set_lead_word(f: &Poly, n_vars: usize) -> u128 {
    match f.first() {
        Some((e, _)) if e.len() == n_vars => set_pack(e).unwrap_or(SET_NEVER),
        _ => SET_NEVER,
    }
}

/// `a · b` as monomials, or `None` when an exponent would pass 127.
#[inline(always)]
fn set_mul(a: u128, b: u128) -> Option<u128> {
    // every byte is at most 127 + 127, so no carry leaves its byte
    let s = a + b;
    (s & PACKED_GUARD == 0).then_some(s)
}

/// The total degree: the sum of the 16 bytes.
#[inline(always)]
fn set_degree(x: u128) -> u32 {
    const LO: u64 = 0x00ff_00ff_00ff_00ff;
    // 16-bit lanes of at most 2·127, then of at most 4·127
    let pairs = |w: u64| (w & LO) + ((w >> 8) & LO);
    let t = pairs(x as u64) + pairs((x >> 64) as u64);
    // the top lane of `t · (1 + 2¹⁶ + 2³² + 2⁴⁸)` is the sum of the four,
    // at most 16·127
    (t.wrapping_mul(0x0001_0001_0001_0001) >> 48) as u32
}

/// A key that sorts ascending where [`cmp_monomial`] sorts descending.
/// Graded orders compare the degree first.  Grevlex then compares from the
/// last variable and the smaller exponent wins, which is the numeric order
/// of the packed word (variable 15 is its top byte); lex and grlex compare
/// from the first variable and the larger exponent wins, the numeric order
/// of the byte-reversed word, inverted.
#[inline(always)]
fn set_order_key(ord: Ordering, x: u128) -> (u32, u128) {
    match ord {
        Ordering::Grevlex => (!set_degree(x), x),
        Ordering::Grlex => (!set_degree(x), !x.swap_bytes()),
        Ordering::Lex => (0, !x.swap_bytes()),
    }
}

/// The monomial a [`set_order_key`] came from.
#[inline(always)]
fn set_from_order_key(ord: Ordering, key: (u32, u128)) -> u128 {
    match ord {
        Ordering::Grevlex => key.1,
        _ => (!key.1).swap_bytes(),
    }
}

/// A list of polynomials with their monomials packed, kept beside it for
/// the many reductions of [`interreduce`] and [`autoreduce`].
struct PackedSet {
    n_vars: usize,
    /// Per polynomial, its monomials in its order, or `None` when one does
    /// not pack.
    terms: Vec<Option<Vec<u128>>>,
    /// Their leading monomials ([`set_lead_word`]): [`SET_NEVER`] where the
    /// lead does not pack, but not where only a tail monomial does not.
    lms: Vec<u128>,
}

impl PackedSet {
    /// `None` when the polynomials do not all have the same number of
    /// unknowns (the exponent-vector code compares what the shorter has).
    fn new(polys: &[Poly]) -> Option<Self> {
        // the number of unknowns is read off the first term there is: a
        // lone empty polynomial has no unknowns to count
        let n_vars = polys
            .iter()
            .find_map(|f| f.first())
            .map_or(0, |(e, _)| e.len());
        if polys.iter().flatten().any(|(e, _)| e.len() != n_vars) {
            return None;
        }
        let mut s = PackedSet {
            n_vars,
            terms: Vec::new(),
            lms: Vec::new(),
        };
        for f in polys {
            s.push(f);
        }
        Some(s)
    }

    fn push(&mut self, f: &Poly) {
        self.lms.push(set_lead_word(f, self.n_vars));
        self.terms.push(set_pack_poly(f, self.n_vars));
    }

    fn replace(&mut self, i: usize, f: &Poly) {
        self.lms[i] = set_lead_word(f, self.n_vars);
        self.terms[i] = set_pack_poly(f, self.n_vars);
    }

    fn remove(&mut self, i: usize) {
        self.terms.remove(i);
        self.lms.remove(i);
    }
}

/// The index of the first of `lms`, other than `skip`, that divides `x`.
fn first_divisor(lms: &[u128], skip: usize, x: u128) -> Option<usize> {
    let cut = skip.min(lms.len());
    let find = |part: &[u128]| part.iter().position(|&l| packed_divides(l, x));
    find(&lms[..cut]).or_else(|| {
        let rest = lms.get(cut + 1..)?;
        find(rest).map(|j| cut + 1 + j)
    })
}

/// [`reduce_skipping_vecs`] on packed monomials, the polynomials `set`
/// being packed in `ps`.  Every monomial it forms, in the heap and in the
/// work map, is a packed word: the same terms are popped in the same
/// order, and the same coefficients come out.  `None` when `f` or a product
/// does not pack, or when the divisor chosen for a monomial has a tail that
/// does not (its lead packs, or it would divide nothing here), and the
/// caller reduces on exponent vectors.  A set element whose lead does not
/// pack is passed by, which is exact: it divides no packed monomial.
fn reduce_skipping_packed(
    f: &Poly,
    set: &[Poly],
    ps: &PackedSet,
    skip: usize,
    p: u64,
    ord: Ordering,
) -> Option<Poly> {
    // below 2³² and for reduced coefficients, one Barrett step is the
    // remainder of `v + neg · gc`, which fits a `u64`
    let fp = (2..1 << 32).contains(&p).then(|| Barrett::new(p));
    let mut work: PkMap<u128, u64> = PkMap::default();
    // a min-heap of order keys: the greatest monomial first
    let mut heap: std::collections::BinaryHeap<std::cmp::Reverse<(u32, u128)>> =
        std::collections::BinaryHeap::new();
    for (x, (_, c)) in set_pack_poly(f, ps.n_vars)?.into_iter().zip(f) {
        let v = work.entry(x).or_insert_with(|| {
            heap.push(std::cmp::Reverse(set_order_key(ord, x)));
            0
        });
        *v = (*v + c % p) % p;
    }
    let mut out: Poly = Vec::new();
    while let Some(std::cmp::Reverse(key)) = heap.pop() {
        let x = set_from_order_key(ord, key);
        let Some(c) = work.remove(&x) else {
            continue;
        };
        if c == 0 {
            continue;
        }
        let Some(j) = first_divisor(&ps.lms, skip, x) else {
            out.push((unpack(x, ps.n_vars), c));
            continue;
        };
        // `set[j]` is monic: `x − c·m·set[j]` cancels the leading term
        // (already removed from `work`) and adds the shifted tail
        let g = &set[j];
        // `None` here is the element whose lead packs and whose tail does
        // not, which the vector code reduces by and is not ours to skip
        let tg = ps.terms[j].as_deref()?;
        let m = x - tg[0];
        let neg = p - c;
        for (&e, &(_, gc)) in tg[1..].iter().zip(&g[1..]) {
            let em = set_mul(e, m)?;
            let v = work.entry(em).or_insert_with(|| {
                heap.push(std::cmp::Reverse(set_order_key(ord, em)));
                0
            });
            *v = match fp {
                Some(fp) if gc < p => fp.reduce(*v + neg * gc),
                _ => (*v + mulmod(neg, gc, p)) % p,
            };
            super::jv_bitops::row_ops(1);
            super::jv_bitops::probes(1, 2);
        }
    }
    // popped in decreasing order: already leading-first
    Some(out)
}

/// [`reduce_skipping_vecs`] of `set[i]` by the rest of `set`, on packed
/// monomials when `ps` (the packed form of `set`) is given and they hold.
fn reduce_skipping(set: &[Poly], ps: Option<&PackedSet>, i: usize, p: u64, ord: Ordering) -> Poly {
    ps.and_then(|ps| reduce_skipping_packed(&set[i], set, ps, i, p, ord))
        .unwrap_or_else(|| reduce_skipping_vecs(&set[i], set, i, p, ord))
}

/// Scale a reduction's output to be monic.  [`reduce_skipping`] returns
/// distinct monomials in decreasing order with non-zero coefficients below
/// `p`, which is all of [`normalise`] that remains to be done.
fn make_monic(mut f: Poly, p: u64) -> Poly {
    scale_monic(&mut f, p);
    f
}

/// Interreduce a basis: drop elements whose leading monomial another
/// divides, then reduce every tail by the others.
pub fn interreduce(basis: &[Poly], p: u64, ord: Ordering) -> Vec<Poly> {
    interreduce_with(basis, p, ord, true)
}

/// [`interreduce`], on packed monomials or (`packed` false) exponent vectors
/// only, the control the tests compare the packed path with.
fn interreduce_with(basis: &[Poly], p: u64, ord: Ordering, packed: bool) -> Vec<Poly> {
    let mut keep: Vec<Poly> = Vec::new();
    for (i, f) in basis.iter().enumerate() {
        let redundant = basis
            .iter()
            .enumerate()
            .any(|(j, g)| j != i && divides(&g[0].0, &f[0].0) && (g[0].0 != f[0].0 || j < i));
        if !redundant {
            keep.push(f.clone());
        }
    }
    let mut out = keep;
    let mut ps = if packed { PackedSet::new(&out) } else { None };
    for i in 0..out.len() {
        let r = reduce_skipping(&out, ps.as_ref(), i, p, ord);
        if !r.is_empty() {
            out[i] = make_monic(r, p);
            if let Some(ps) = ps.as_mut() {
                ps.replace(i, &out[i]);
            }
        }
    }
    out.sort_by(|a, b| cmp_monomial(&a[0].0, &b[0].0, ord));
    out
}

/// Autoreduce a *generating set* (not a Gröbner basis): reduce every
/// element by the others until nothing changes, dropping the ones that
/// reduce to zero.  Two generators with the same leading monomial are
/// not redundant — their difference is a new generator with a smaller
/// one — which is what distinguishes this from [`interreduce`].
pub fn autoreduce(polys: &[Poly], p: u64, ord: Ordering) -> Vec<Poly> {
    autoreduce_with(polys, p, ord, true)
}

/// [`autoreduce`], on packed monomials or (`packed` false) exponent vectors
/// only.
fn autoreduce_with(polys: &[Poly], p: u64, ord: Ordering, packed: bool) -> Vec<Poly> {
    let mut set: Vec<Poly> = polys
        .iter()
        .map(|f| normalise(f, p, ord))
        .filter(|f| !f.is_empty())
        .collect();
    let mut ps = if packed { PackedSet::new(&set) } else { None };
    loop {
        let mut changed = false;
        let mut i = 0;
        while i < set.len() {
            let r = reduce_skipping(&set, ps.as_ref(), i, p, ord);
            let r = if r.is_empty() { r } else { make_monic(r, p) };
            if r != set[i] {
                changed = true;
                if r.is_empty() {
                    set.remove(i);
                    if let Some(ps) = ps.as_mut() {
                        ps.remove(i);
                    }
                    continue;
                }
                set[i] = r;
                if let Some(ps) = ps.as_mut() {
                    ps.replace(i, &set[i]);
                }
            }
            i += 1;
        }
        if !changed {
            break;
        }
    }
    set.sort_by(|a, b| cmp_monomial(&b[0].0, &a[0].0, ord));
    set
}

// ── Solving ────────────────────────────────────────────────────────

/// Observations from the opt-in streaming/domain solver. These are stage
/// diagnostics, not complete operation accounting or a performance claim.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct VisitReport {
    /// Every solution in the requested Cartesian domain has been visited.
    pub complete: bool,
    /// The visitor requested termination after an equation-checked witness.
    pub stopped_early: bool,
    pub timed_out: bool,
    /// The fallible trace sink rejected an event; no later callback ran.
    pub trace_stopped: bool,
    /// At most 512 UTF-8 bytes of the first sink error.
    pub trace_error: Option<String>,
    pub solutions_visited: usize,
    pub candidates_rejected: usize,
    pub solving_degree: u32,
    pub max_rows: usize,
    pub max_cols: usize,
    pub f4_runs: usize,
    pub field_ops: u64,
    pub domain_evaluations: u64,
    pub factored_univariates: usize,
    /// Counted coefficient multiplications in the reused root finder only.
    pub root_field_multiplications: u64,
    pub full_field_branches: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitEnd {
    Complete,
    Stopped,
    TimedOut,
    TraceStopped,
}

/// Borrowed events for one depth-first traversal. `assignments` contains only
/// fixed original variables; `active_variables` maps local exponent positions
/// to original variables. Node IDs are monotonically assigned within the call.
/// Native traces retain useful rows, not zero rows or private pivots. They are
/// materialized per F4 call, so sink limits do not bound native matrix memory.
#[derive(Debug, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VisitEvent<'a> {
    System {
        node_id: usize,
        parent_id: Option<usize>,
        active_variables: &'a [usize],
        assignments: &'a [(usize, u64)],
        polynomials: &'a [Poly],
    },
    NativeResult {
        node_id: usize,
        report: &'a F4Report,
        trace: &'a F4Trace,
    },
    Branch {
        node_id: usize,
        original_variable: usize,
        value: u64,
        source: &'static str,
    },
    Candidate {
        node_id: usize,
        assignment: &'a [u64],
        residual_satisfied: bool,
        original_satisfied: Option<bool>,
    },
}

struct VisitState<'a, F, S> {
    original: &'a [Poly],
    domains: &'a [Option<Vec<u64>>],
    p: u64,
    opts: &'a F4Options,
    assignment: Vec<u64>,
    visitor: F,
    sink: Option<S>,
    next_node: usize,
    report: VisitReport,
}

impl<F, S> VisitState<'_, F, S>
where
    F: FnMut(&[u64]) -> bool,
    S: FnMut(VisitEvent<'_>) -> Result<(), String>,
{
    fn emit(&mut self, event: VisitEvent<'_>) -> bool {
        if self.report.trace_stopped {
            return false;
        }
        if let Some(sink) = &mut self.sink {
            if let Err(mut error) = sink(event) {
                let mut end = error.len().min(512);
                while !error.is_char_boundary(end) {
                    end -= 1;
                }
                error.truncate(end);
                self.report.trace_stopped = true;
                self.report.trace_error = Some(error);
                return false;
            }
        }
        true
    }
    fn expired(&self) -> bool {
        self.opts.deadline.is_some_and(|d| Instant::now() >= d)
    }

    fn walk(&mut self, input: &[Poly], active: &[usize], parent_id: Option<usize>) -> VisitEnd {
        if self.report.trace_stopped {
            return VisitEnd::TraceStopped;
        }
        if self.expired() {
            return VisitEnd::TimedOut;
        }
        let node_id = self.next_node;
        self.next_node += 1;
        if self.sink.is_some() {
            let assignments: Vec<_> = self
                .assignment
                .iter()
                .copied()
                .enumerate()
                .filter(|(i, _)| !active.contains(i))
                .collect();
            if !self.emit(VisitEvent::System {
                node_id,
                parent_id,
                active_variables: active,
                assignments: &assignments,
                polynomials: input,
            }) {
                return VisitEnd::TraceStopped;
            }
        }
        // Sink persistence may itself consume the remaining deadline. It is
        // cooperative (not interruptible), but must not admit later work.
        if self.sink.is_some() && self.expired() {
            return VisitEnd::TimedOut;
        }
        if active.is_empty() {
            // A degree-truncated basis can describe a strict superset of
            // the original variety. Never expose those spurious roots.
            let residual_satisfied = input.iter().all(|g| eval(g, &[], self.p) == 0);
            let original_satisfied = residual_satisfied.then(|| {
                self.original
                    .iter()
                    .all(|g| eval(g, &self.assignment, self.p) == 0)
            });
            if self.sink.is_some() {
                let assignment = self.assignment.clone();
                if !self.emit(VisitEvent::Candidate {
                    node_id,
                    assignment: &assignment,
                    residual_satisfied,
                    original_satisfied,
                }) {
                    return VisitEnd::TraceStopped;
                }
            }
            if self.sink.is_some() && self.expired() {
                return VisitEnd::TimedOut;
            }
            if !residual_satisfied {
                return VisitEnd::Complete;
            }
            if original_satisfied == Some(false) {
                self.report.candidates_rejected += 1;
                return VisitEnd::Complete;
            }
            self.report.solutions_visited += 1;
            return if (self.visitor)(&self.assignment) {
                VisitEnd::Stopped
            } else {
                VisitEnd::Complete
            };
        }
        if active
            .iter()
            .any(|&i| self.domains[i].as_ref().is_some_and(Vec::is_empty))
        {
            return VisitEnd::Complete;
        }
        let (r, trace) = if self.sink.is_some() {
            let (r, trace) = f4_record(input, active.len(), self.p, self.opts);
            (r, Some(trace))
        } else {
            (f4(input, active.len(), self.p, self.opts), None)
        };
        if self.report.f4_runs == 0 {
            self.report.solving_degree = r.solving_degree;
        }
        self.report.f4_runs += 1;
        self.report.max_rows = self.report.max_rows.max(r.max_rows);
        self.report.max_cols = self.report.max_cols.max(r.max_cols);
        self.report.field_ops += r.field_ops;
        if let Some(trace) = &trace {
            if !self.emit(VisitEvent::NativeResult {
                node_id,
                report: &r,
                trace,
            }) {
                return VisitEnd::TraceStopped;
            }
        }
        if r.timed_out || self.expired() {
            return VisitEnd::TimedOut;
        }
        if r.inconsistent {
            return VisitEnd::Complete;
        }
        let mut best: Option<(usize, Vec<u64>)> = None;
        for g in &r.basis {
            let vars: HashSet<_> = g
                .iter()
                .flat_map(|(e, _)| {
                    e.iter()
                        .enumerate()
                        .filter(|(_, k)| **k != 0)
                        .map(|(i, _)| i)
                })
                .collect();
            if vars.len() != 1 {
                continue;
            }
            let var = *vars.iter().next().unwrap();
            let roots = if let Some(domain) = &self.domains[active[var]] {
                let mut roots = Vec::new();
                for &x in domain {
                    if self.expired() {
                        return VisitEnd::TimedOut;
                    }
                    self.report.domain_evaluations += 1;
                    let value = g.iter().fold(0, |v, (e, c)| {
                        (v + mulmod(*c, powmod(x, u64::from(e[var]), self.p), self.p)) % self.p
                    });
                    if value == 0 {
                        roots.push(x);
                    }
                }
                roots
            } else {
                let degree = g.iter().map(|(e, _)| e[var]).max().unwrap_or(0) as usize;
                let mut coefficients = vec![0; degree + 1];
                for (e, c) in g {
                    coefficients[e[var] as usize] = *c;
                }
                self.report.factored_univariates += 1;
                let Some(roots) = super::glv_invariant_base::poly_roots_fp_counted_until(
                    &coefficients,
                    self.p,
                    0x4634_524f_4f54,
                    &mut self.report.root_field_multiplications,
                    self.opts.deadline,
                ) else {
                    return VisitEnd::TimedOut;
                };
                roots
            };
            if best.as_ref().is_none_or(|(_, old)| roots.len() < old.len()) {
                best = Some((var, roots));
            }
            if best.as_ref().is_some_and(|(_, roots)| roots.is_empty()) {
                return VisitEnd::Complete;
            }
        }
        // Without a univariate, branch on the smallest finite domain when
        // one is known. Full-field fallback is lazy and remains explicit.
        let has_univariate = best.is_some();
        let (var, values): (usize, Box<dyn Iterator<Item = u64>>) = if let Some((i, roots)) = best {
            (i, Box::new(roots.into_iter()))
        } else if let Some((i, domain)) = active
            .iter()
            .enumerate()
            .filter_map(|(i, &original)| self.domains[original].as_ref().map(|d| (i, d)))
            .min_by_key(|(_, d)| d.len())
        {
            (i, Box::new(domain.clone().into_iter()))
        } else {
            (0, Box::new(0..self.p))
        };
        let full_field = !has_univariate && self.domains[active[var]].is_none();
        let mut remaining = active.to_vec();
        let original = remaining.remove(var);
        for value in values {
            if self.expired() {
                return VisitEnd::TimedOut;
            }
            if full_field {
                self.report.full_field_branches += 1;
            }
            if !self.emit(VisitEvent::Branch {
                node_id,
                original_variable: original,
                value,
                source: if has_univariate {
                    "univariate_roots"
                } else if full_field {
                    "full_field"
                } else {
                    "finite_domain"
                },
            }) {
                return VisitEnd::TraceStopped;
            }
            if self.sink.is_some() && self.expired() {
                return VisitEnd::TimedOut;
            }
            self.assignment[original] = value;
            let sub: Vec<_> = r
                .basis
                .iter()
                .map(|g| {
                    normalise_owned(specialise(g, var, value, self.p), self.p, self.opts.order)
                })
                .filter(|g| !g.is_empty())
                .collect();
            let end = self.walk(&sub, &remaining, Some(node_id));
            if end != VisitEnd::Complete {
                return end;
            }
        }
        VisitEnd::Complete
    }
}

/// Visit exact solutions in the Cartesian product of `domains` (`None`
/// means all of F_p), stopping when the visitor returns true. Domain
/// restrictions define the requested problem; completeness is relative to
/// them. Callers claiming unrestricted completeness must justify their domains.
/// Auxiliary variables must not inherit a coordinate membership domain.
///
/// Unlike [`solve`], this opt-in path uses rational-factor extraction for
/// unrestricted univariates. The legacy exhaustive scan remains a control.
///
/// Returns `Err` before traversal, callbacks or native checkpoint I/O if any
/// of `checkpoint`, `checkpoint_stop_after`, `checkpoint_in_step` or
/// `checkpoint_stop_in_rref` is set, including hooks without a checkpoint
/// path. A native checkpoint describes one polynomial system, not this
/// recursive traversal: specialized systems cannot reuse its path. Direct
/// native F4 checkpoint support is unaffected.
pub fn solve_visit(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    domains: &[Option<Vec<u64>>],
    visitor: impl FnMut(&[u64]) -> bool,
) -> Result<VisitReport, String> {
    solve_visit_impl(
        input,
        n_vars,
        p,
        opts,
        domains,
        visitor,
        None::<fn(VisitEvent<'_>) -> Result<(), String>>,
    )
}

/// Traced variant of [`solve_visit`]. Sink failure is sticky and returns an
/// incomplete report with `trace_stopped`, distinct from positive visitor stop
/// or timeout. Input errors remain `Err`. Events borrow native data only during
/// the callback. Native row traces are materialized once per recursive F4 call;
/// external time/memory limits remain necessary. Hilbert stopping is rejected
/// because the native recording path does not support that strategy.
/// All four checkpoint options are also rejected with `Err` before traversal,
/// either callback or native checkpoint I/O, as in [`solve_visit`]. There is
/// no recursive traversal or trace-sink resume support.
pub fn solve_visit_traced(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    domains: &[Option<Vec<u64>>],
    visitor: impl FnMut(&[u64]) -> bool,
    sink: impl FnMut(VisitEvent<'_>) -> Result<(), String>,
) -> Result<VisitReport, String> {
    if opts.stop_hilbert.is_some() {
        return Err("traced visitor does not support Hilbert stopping".into());
    }
    solve_visit_impl(input, n_vars, p, opts, domains, visitor, Some(sink))
}

fn solve_visit_impl<F, S>(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    domains: &[Option<Vec<u64>>],
    visitor: F,
    sink: Option<S>,
) -> Result<VisitReport, String>
where
    F: FnMut(&[u64]) -> bool,
    S: FnMut(VisitEvent<'_>) -> Result<(), String>,
{
    if opts.checkpoint.is_some()
        || opts.checkpoint_stop_after.is_some()
        || opts.checkpoint_in_step.is_some()
        || opts.checkpoint_stop_in_rref.is_some()
    {
        return Err("recursive visitor does not support checkpoint options".into());
    }
    if p >= 1u64 << 32
        || !super::residual_walk::is_prime_u64(p)
        || domains.len() != n_vars
        || domains
            .iter()
            .flatten()
            .any(|d| d.iter().any(|&x| x >= p) || d.iter().collect::<HashSet<_>>().len() != d.len())
        || input
            .iter()
            .flatten()
            .any(|(e, c)| e.len() != n_vars || *c >= p)
    {
        return Err("invalid prime, domains or polynomial dimensions".into());
    }
    let mut state = VisitState {
        original: input,
        domains,
        p,
        opts,
        assignment: vec![0; n_vars],
        visitor,
        sink,
        next_node: 0,
        report: VisitReport::default(),
    };
    let end = state.walk(input, &(0..n_vars).collect::<Vec<_>>(), None);
    state.report.complete = end == VisitEnd::Complete;
    state.report.stopped_early = end == VisitEnd::Stopped;
    state.report.timed_out = end == VisitEnd::TimedOut;
    Ok(state.report)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Verdict {
    Inconsistent,
    Solutions(Vec<Vec<u64>>),
    /// The degree bound (or the deadline) was reached before the system
    /// resolved.
    Undetermined,
}

#[derive(Clone, Debug)]
pub struct SolveReport {
    pub verdict: Verdict,
    /// Solving degree of the top-level run.
    pub solving_degree: u32,
    pub degree_reached: u32,
    pub max_rows: usize,
    pub max_cols: usize,
    pub f4_runs: usize,
    pub ms: f64,
    pub timed_out: bool,
    /// `F_p` multiplications spent in row reduction, summed over the F4
    /// runs of the substitution tree.
    pub field_ops: u64,
    /// Block-reduced steps of the top-level run.
    pub blocked_steps: usize,
    /// Candidate points the substitution tree produced that do not satisfy
    /// the input, and were dropped.  A truncated run (pairs above the
    /// degree bound, or a staircase stop) returns a basis of a sub-ideal,
    /// whose variety can be strictly larger than the input's; the final
    /// evaluation against the input keeps the verdict exact.  Non-zero only
    /// when some run of the tree was truncated.
    pub candidates_rejected: usize,
}

/// Roots in `F_p` of a univariate polynomial given as `(degree, coeff)`.
fn roots_univariate(coeffs: &[(u32, u64)], p: u64) -> Vec<u64> {
    (0..p)
        .filter(|&x| {
            let mut v = 0u64;
            for (k, c) in coeffs {
                v = (v + mulmod(*c, powmod(x, *k as u64, p), p)) % p;
            }
            v == 0
        })
        .collect()
}

/// A forced assignment from a pure nonconstant linear univariate equation.
/// Combine coefficients so duplicate or zero terms cannot invent a pivot.
fn linear_assignment(g: &Poly, p: u64) -> Option<(usize, u64)> {
    let mut variable = None;
    let (mut a, mut b) = (0u64, 0u64);
    for (powers, coefficient) in g {
        let coefficient = coefficient % p;
        if coefficient == 0 {
            continue;
        }
        let mut support = powers.iter().enumerate().filter(|(_, power)| **power != 0);
        match support.next() {
            None => b = ((b as u128 + coefficient as u128) % p as u128) as u64,
            Some((i, 1)) if support.next().is_none() => {
                if variable.is_some_and(|previous| previous != i) {
                    return None;
                }
                variable = Some(i);
                a = ((a as u128 + coefficient as u128) % p as u128) as u64;
            }
            _ => return None,
        }
    }
    (a != 0).then(|| (variable.unwrap(), mulmod((p - b) % p, inv_mod_u64(a, p), p)))
}

fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

/// Substitute `x_i = a` and drop variable `i`.
fn specialise(f: &Poly, i: usize, a: u64, p: u64) -> Vec<(Vec<u32>, u64)> {
    f.iter()
        .map(|(e, c)| {
            let mut e2 = e.clone();
            let k = e2.remove(i);
            (e2, mulmod(*c, powmod(a, k as u64, p), p))
        })
        .collect()
}

/// Solve a zero-dimensional system over `F_p` by degree-bounded F4 and
/// substitution.  Solutions are full assignments `(x_0, …, x_{n−1})`.
pub fn solve(input: &[Poly], n_vars: usize, p: u64, opts: &F4Options) -> SolveReport {
    let linear_roots = std::env::var("F4_FP_LINEAR_ROOTS").is_ok_and(|v| v == "1");
    solve_with_linear_roots(input, n_vars, p, opts, linear_roots)
}

fn solve_with_linear_roots(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    linear_roots: bool,
) -> SolveReport {
    let t0 = Instant::now();
    let ops0 = field_ops_now();
    let mut runs = 0usize;
    let mut max_rows = 0;
    let mut max_cols = 0;
    let mut top: Option<(u32, u32)> = None;
    let mut timed_out = false;
    let mut blocked = 0usize;
    let verdict = solve_rec(
        input,
        n_vars,
        p,
        opts,
        linear_roots,
        &mut runs,
        &mut max_rows,
        &mut max_cols,
        &mut top,
        &mut timed_out,
        &mut blocked,
    );
    let (solving_degree, degree_reached) = top.unwrap_or((0, 0));
    // Every run of the substitution tree may be a truncation (pairs above
    // the degree bound dropped, or a staircase stop), and a truncated basis
    // generates a sub-ideal of the input: its variety is a superset of the
    // input's, never a subset, and the same holds for every specialised
    // basis further down.  The tree therefore returns a superset of the
    // solutions, and the input itself decides which candidates are real.
    // A refutation needs no check: `1` in a truncated basis is `1` in the
    // ideal.
    let mut candidates_rejected = 0usize;
    let verdict = match verdict {
        Verdict::Solutions(sols) => {
            let kept: Vec<Vec<u64>> = sols
                .into_iter()
                .filter(|s| {
                    let ok = input.iter().all(|f| eval(f, s, p) == 0);
                    if !ok {
                        candidates_rejected += 1;
                    }
                    ok
                })
                .collect();
            if kept.is_empty() {
                Verdict::Inconsistent
            } else {
                Verdict::Solutions(kept)
            }
        }
        other => other,
    };
    SolveReport {
        verdict,
        solving_degree,
        degree_reached,
        max_rows,
        max_cols,
        f4_runs: runs,
        ms: t0.elapsed().as_secs_f64() * 1e3,
        timed_out,
        field_ops: field_ops_now() - ops0,
        blocked_steps: blocked,
        candidates_rejected,
    }
}

#[allow(clippy::too_many_arguments)]
fn solve_rec(
    input: &[Poly],
    n_vars: usize,
    p: u64,
    opts: &F4Options,
    linear_roots: bool,
    runs: &mut usize,
    max_rows: &mut usize,
    max_cols: &mut usize,
    top: &mut Option<(u32, u32)>,
    timed_out: &mut bool,
    blocked: &mut usize,
) -> Verdict {
    if n_vars == 0 {
        // constants only: consistent iff all zero
        let bad = input.iter().any(|f| f.iter().any(|(_, c)| *c % p != 0));
        return if bad {
            Verdict::Inconsistent
        } else {
            Verdict::Solutions(vec![vec![]])
        };
    }
    let r = f4(input, n_vars, p, opts);
    *runs += 1;
    *max_rows = (*max_rows).max(r.max_rows);
    *max_cols = (*max_cols).max(r.max_cols);
    if top.is_none() {
        *top = Some((r.solving_degree, r.degree_reached));
        *blocked = r.blocked_steps;
    }
    if r.timed_out {
        *timed_out = true;
        return Verdict::Undetermined;
    }
    if r.inconsistent {
        return Verdict::Inconsistent;
    }
    // a univariate element?  prefer the variable with the fewest roots
    // A forced linear value is valid even for a truncated basis: every
    // basis polynomial lies in the input ideal. Other constraints are
    // still checked by recursion and by solve's final exact input filter.
    let mut best = if linear_roots {
        r.basis
            .iter()
            .find_map(|g| linear_assignment(g, p))
            .map(|(i, x)| (i, vec![x]))
    } else {
        None
    };
    if best.is_none() {
        for g in &r.basis {
            let vars: HashSet<usize> = g
                .iter()
                .flat_map(|(e, _)| e.iter().enumerate().filter(|(_, &k)| k > 0).map(|(i, _)| i))
                .collect();
            if vars.len() == 1 {
                let i = *vars.iter().next().unwrap();
                let coeffs: Vec<(u32, u64)> = g.iter().map(|(e, c)| (e[i], *c)).collect();
                let roots = roots_univariate(&coeffs, p);
                if best.as_ref().is_none_or(|(_, rs)| roots.len() < rs.len()) {
                    best = Some((i, roots));
                }
            }
        }
    }
    let (var, values): (usize, Vec<u64>) = match best {
        Some(b) => b,
        None => {
            // Without a univariate element every value of the first
            // variable is tried.  With a truncation (`pairs_above_bound`)
            // the basis is that of a sub-ideal, so this and the root
            // enumeration above return a superset of the solutions, which
            // `solve` filters against the input.
            (0, (0..p).collect())
        }
    };
    let mut sols: Vec<Vec<u64>> = Vec::new();
    for a in values {
        if let Some(d) = opts.deadline {
            if Instant::now() > d {
                *timed_out = true;
                return Verdict::Undetermined;
            }
        }
        let sub: Vec<Poly> = r
            .basis
            .iter()
            .map(|g| normalise_owned(specialise(g, var, a, p), p, opts.order))
            .filter(|g| !g.is_empty())
            .collect();
        match solve_rec(
            &sub,
            n_vars - 1,
            p,
            opts,
            linear_roots,
            runs,
            max_rows,
            max_cols,
            top,
            timed_out,
            blocked,
        ) {
            Verdict::Inconsistent => {}
            Verdict::Undetermined => return Verdict::Undetermined,
            Verdict::Solutions(ss) => {
                for mut s in ss {
                    s.insert(var, a);
                    sols.push(s);
                }
            }
        }
    }
    if sols.is_empty() {
        Verdict::Inconsistent
    } else {
        Verdict::Solutions(sols)
    }
}

/// Evaluate a polynomial at a point.
pub fn eval(f: &Poly, x: &[u64], p: u64) -> u64 {
    let mut v = 0u64;
    for (e, c) in f {
        let mut t = *c;
        for (i, &k) in e.iter().enumerate() {
            if k > 0 {
                t = mulmod(t, powmod(x[i], k as u64, p), p);
            }
        }
        v = (v + t) % p;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn analytic_linear_roots_match_exhaustive_field_roots() {
        for p in [2, 3, 5, 7, 59] {
            for a in 1..p {
                for b in 0..p {
                    let g = vec![(vec![0, 1], a), (vec![0, 0], b)];
                    let (var, root) = linear_assignment(&g, p).unwrap();
                    assert_eq!(var, 1);
                    assert_eq!(roots_univariate(&[(1, a), (0, b)], p), vec![root]);
                }
            }
        }
        assert_eq!(
            linear_assignment(&vec![(vec![1], 1), (vec![1], 4)], 5),
            None
        );
        assert_eq!(
            linear_assignment(&vec![(vec![1, 0], 1), (vec![0, 1], 1)], 5),
            None
        );
        assert_eq!(linear_assignment(&vec![(vec![2], 1)], 5), None);
        assert_eq!(linear_assignment(&vec![(vec![0], 1)], 5), None);
    }

    #[test]
    fn linear_root_policy_preserves_recursive_and_inconsistent_answers() {
        for p in [3, 5, 7] {
            for c in 0..p {
                let input = vec![
                    vec![(vec![2, 0], 1), (vec![0, 0], c)],
                    vec![(vec![0, 1], 1), (vec![1, 0], p - 1)],
                ];
                let opts = F4Options::new(Ordering::Grevlex, 8);
                let baseline = solve_with_linear_roots(&input, 2, p, &opts, false);
                let candidate = solve_with_linear_roots(&input, 2, p, &opts, true);
                match (baseline.verdict, candidate.verdict) {
                    (Verdict::Inconsistent, Verdict::Inconsistent) => {}
                    (Verdict::Solutions(mut a), Verdict::Solutions(mut b)) => {
                        a.sort();
                        b.sort();
                        assert_eq!(a, b);
                    }
                    (a, b) => panic!("linear policy changed verdict: {a:?} vs {b:?}"),
                }
            }
        }
    }


    fn recursive_visitors_reject_all_checkpoint_options_before_callbacks_or_file_effects() {
        let root = std::env::temp_dir().join(format!(
            "f4-visitor-checkpoint-rejection-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&root).unwrap();
        let existing = root.join("existing.checkpoint");
        let absent_parent = root.join("absent");
        let absent = absent_parent.join("new.checkpoint");
        let sentinel = b"not a native checkpoint; must remain untouched";
        std::fs::write(&existing, sentinel).unwrap();
        // The empty system would immediately visit a solution; the quadratic
        // would invoke native F4. Reject before either traversal can begin.
        for (input, n_vars, domains) in [
            (vec![], 0, vec![]),
            (vec![vec![(vec![2], 1), (vec![0], 6)]], 1, vec![None]),
        ] {
            for path in [&existing, &absent] {
                // Each option alone, each hook with a checkpoint path, and all
                // options together. Orphan hooks must not silently be ignored.
                for case in 0..8 {
                    let mut opts = F4Options::new(Ordering::Grevlex, 4);
                    if case == 0 || case >= 4 {
                        opts.checkpoint = Some((path.clone(), Duration::ZERO));
                    }
                    if matches!(case, 1 | 4 | 7) {
                        opts.checkpoint_stop_after = Some(0);
                    }
                    if matches!(case, 2 | 5 | 7) {
                        opts.checkpoint_in_step = Some(Duration::ZERO);
                    }
                    if matches!(case, 3 | 6 | 7) {
                        opts.checkpoint_stop_in_rref = Some(0);
                    }
                    let plain = solve_visit(&input, n_vars, 7, &opts, &domains, |_| {
                        panic!("checkpoint options reached the solution visitor")
                    });
                    let traced = solve_visit_traced(
                        &input,
                        n_vars,
                        7,
                        &opts,
                        &domains,
                        |_| panic!("checkpoint options reached the traced solution visitor"),
                        |_| panic!("checkpoint options reached the trace sink"),
                    );
                    for result in [plain, traced] {
                        assert_eq!(
                            result.unwrap_err(),
                            "recursive visitor does not support checkpoint options",
                            "case {case}, path {path:?}, n_vars {n_vars}"
                        );
                    }
                    assert_eq!(std::fs::read(&existing).unwrap(), sentinel);
                    assert!(!absent_parent.exists());
                    assert_eq!(std::fs::read_dir(&root).unwrap().count(), 1);
                }
            }
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn traced_visit_matches_plain_with_unrestricted_auxiliary_and_serializable_events() {
        // x in {4}, y unrestricted, y^2=x over F7. In particular the
        // valid auxiliary y=2 must not inherit the singleton leaf domain {4}.
        let input = vec![vec![(vec![0, 2], 1), (vec![1, 0], 6)]];
        let domains = [Some(vec![4]), None];
        for order in [Ordering::Lex, Ordering::Grevlex] {
            let opts = F4Options::new(order, 4);
            let mut plain = Vec::new();
            let a = solve_visit(&input, 2, 7, &opts, &domains, |s| {
                plain.push(s.to_vec());
                false
            })
            .unwrap();
            let mut traced = Vec::new();
            let mut systems = 0;
            let mut native = 0;
            let mut candidates = 0;
            let b = solve_visit_traced(
                &input,
                2,
                7,
                &opts,
                &domains,
                |s| {
                    traced.push(s.to_vec());
                    false
                },
                |event| {
                    // The callback can durably encode without owning the native data.
                    assert!(serde_json::to_vec(&event).is_ok());
                    match event {
                        VisitEvent::System {
                            node_id,
                            active_variables,
                            assignments,
                            ..
                        } => {
                            assert_eq!(node_id, systems);
                            systems += 1;
                            assert_eq!(active_variables.len() + assignments.len(), 2);
                            for (original, _) in assignments {
                                assert!(!active_variables.contains(original));
                            }
                        }
                        VisitEvent::NativeResult { report, trace, .. } => {
                            native += 1;
                            assert!(!report.timed_out);
                            assert!(!trace.input_lms.is_empty());
                        }
                        VisitEvent::Candidate {
                            residual_satisfied,
                            original_satisfied,
                            ..
                        } => {
                            candidates += 1;
                            assert!(residual_satisfied);
                            assert_eq!(original_satisfied, Some(true));
                        }
                        _ => {}
                    }
                    Ok(())
                },
            )
            .unwrap();
            plain.sort();
            traced.sort();
            assert_eq!(plain, vec![vec![4, 2], vec![4, 5]]);
            assert_eq!(traced, plain);
            assert!(a.complete && b.complete && !b.trace_stopped);
            assert_eq!(native, b.f4_runs);
            assert_eq!(candidates, b.solutions_visited);
            assert!(b.factored_univariates > 0);
            assert_eq!(b.full_field_branches, 0);
        }
    }

    #[test]
    fn traced_visit_sink_failure_is_sticky_and_distinct_from_other_stops() {
        let input = vec![vec![(vec![2], 1), (vec![0], 6)]];
        let opts = F4Options::new(Ordering::Grevlex, 4);
        let mut calls = 0;
        let before = solve_visit_traced(
            &input,
            1,
            7,
            &opts,
            &[None],
            |_| panic!("sink rejected before any witness"),
            |_| {
                calls += 1;
                Err("é".repeat(400))
            },
        )
        .unwrap();
        assert_eq!(calls, 1);
        assert_eq!(before.f4_runs, 0);
        assert!(
            before.trace_stopped && !before.complete && !before.stopped_early && !before.timed_out
        );
        assert_eq!(before.trace_error.unwrap().len(), 512);

        let mut sequence = Vec::new();
        let during = solve_visit_traced(
            &input,
            1,
            7,
            &opts,
            &[None],
            |_| panic!("sink rejects before visiting candidate"),
            |event| {
                match event {
                    VisitEvent::System { .. } => sequence.push("system"),
                    VisitEvent::NativeResult { .. } => sequence.push("native"),
                    VisitEvent::Branch { .. } => {
                        sequence.push("branch");
                        return Err("event limit".into());
                    }
                    VisitEvent::Candidate { .. } => panic!("after sink failure"),
                }
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(sequence, ["system", "native", "branch"]);
        assert_eq!(during.f4_runs, 1);
        assert_eq!(during.solutions_visited, 0);
        assert!(
            during.trace_stopped && !during.complete && !during.stopped_early && !during.timed_out
        );
        assert_eq!(during.trace_error.as_deref(), Some("event limit"));

        let positive =
            solve_visit_traced(&input, 1, 7, &opts, &[None], |_| true, |_| Ok(())).unwrap();
        assert!(positive.stopped_early && !positive.complete && !positive.trace_stopped);
        let expired = F4Options::new(Ordering::Grevlex, 4).with_budget(Duration::ZERO);
        let timeout = solve_visit_traced(
            &input,
            1,
            7,
            &expired,
            &[None],
            |_| panic!("expired visitor"),
            |_| panic!("expired sink"),
        )
        .unwrap();
        assert!(timeout.timed_out && !timeout.complete && !timeout.trace_stopped);
    }

    #[test]
    fn traced_visit_checks_truncated_candidates_and_stops_lazy_full_field_branching() {
        let input = vec![
            vec![(vec![2, 0], 1), (vec![0, 1], 6)],
            vec![(vec![0, 2], 1), (vec![1, 0], 6)],
        ];
        let opts = F4Options::new(Ordering::Grevlex, 1);
        let mut roots = Vec::new();
        let report = solve_visit_traced(
            &input,
            2,
            7,
            &opts,
            &[None, None],
            |s| {
                assert!(input.iter().all(|g| eval(g, s, 7) == 0));
                roots.push(s.to_vec());
                false
            },
            |_| Ok(()),
        )
        .unwrap();
        let expected: Vec<_> = (0..7)
            .flat_map(|x| (0..7).map(move |y| vec![x, y]))
            .filter(|s| input.iter().all(|g| eval(g, s, 7) == 0))
            .collect();
        roots.sort();
        assert_eq!(roots, expected);
        assert!(report.complete);

        // An unconstrained variable would have billions of values. Reject the
        // first branch: no range collection, child F4 or candidate is allowed.
        let p = 4_294_967_291;
        let mut branches = 0;
        let lazy = solve_visit_traced(
            &[],
            1,
            p,
            &opts,
            &[None],
            |_| panic!("branch rejected"),
            |event| {
                if let VisitEvent::Branch { value, source, .. } = event {
                    branches += 1;
                    assert_eq!(value, 0);
                    assert_eq!(source, "full_field");
                    return Err("branch cap".into());
                }
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(branches, 1);
        assert!(lazy.trace_stopped && !lazy.complete);
        assert_eq!(lazy.f4_runs, 1);
        assert_eq!(lazy.solutions_visited, 0);
    }

    #[test]
    fn streaming_domains_follow_original_variables_and_match_the_legacy_solver() {
        let p = 7;
        // x^2=1, y=x*z, z^2=1. Variable elimination may remove z
        // before y; the support tag still belongs to the original x.
        let input = vec![
            vec![(vec![2, 0, 0], 1), (vec![0, 0, 0], p - 1)],
            vec![(vec![0, 1, 0], 1), (vec![1, 0, 1], p - 1)],
            vec![(vec![0, 0, 2], 1), (vec![0, 0, 0], p - 1)],
        ];
        for order in [Ordering::Lex, Ordering::Grevlex] {
            let opts = F4Options::new(order, 6);
            let Verdict::Solutions(mut want) = solve(&input, 3, p, &opts).verdict else {
                panic!("control must resolve");
            };
            want.sort();
            let mut got = Vec::new();
            let report = solve_visit(&input, 3, p, &opts, &[Some(vec![1, 6]), None, None], |s| {
                got.push(s.to_vec());
                false
            })
            .unwrap();
            got.sort();
            assert_eq!(got, want);
            assert!(report.complete && !report.stopped_early && !report.timed_out);
            assert!(report.domain_evaluations > 0);
            assert!(report.factored_univariates > 0);
            assert_eq!(report.full_field_branches, 0);
        }
    }

    #[test]
    fn streaming_verifies_truncated_roots_and_distinguishes_stop_timeout_and_exhaustion() {
        // A deliberately truncated F4 can leave candidate supersets. The
        // visitor still sees only exact roots of the original equations.
        let input = vec![
            vec![(vec![2, 0], 1), (vec![0, 1], 6)],
            vec![(vec![0, 2], 1), (vec![1, 0], 6)],
        ];
        let opts = F4Options::new(Ordering::Grevlex, 1);
        let mut count = 0;
        let report = solve_visit(&input, 2, 7, &opts, &[None, None], |s| {
            assert!(input.iter().all(|g| eval(g, s, 7) == 0));
            count += 1;
            count == 2
        })
        .unwrap();
        assert_eq!(count, 2);
        assert!(report.stopped_early && !report.complete && !report.timed_out);
        let expired = F4Options::new(Ordering::Grevlex, 4).with_budget(Duration::ZERO);
        let timeout =
            solve_visit(&input, 2, 7, &expired, &[None, None], |_| panic!("expired")).unwrap();
        assert!(timeout.timed_out && !timeout.complete && !timeout.stopped_early);
        let empty = solve_visit(&input, 2, 7, &opts, &[Some(vec![]), None], |_| {
            panic!("empty domain")
        })
        .unwrap();
        assert!(empty.complete && !empty.stopped_early && !empty.timed_out);
        assert_eq!(empty.solutions_visited, 0);
        assert!(solve_visit(&input, 2, 7, &opts, &[Some(vec![1, 1]), None], |_| false).is_err());
    }

    #[test]
    fn unrestricted_rational_factors_handle_repeated_roots_and_characteristic_two() {
        for p in [2, 5, 7, 13] {
            // x^p-x is squarefree and vanishes at the whole field; its
            // square checks that factor extraction does not repeat roots.
            let f = vec![
                (vec![2 * p as u32], 1),
                (vec![p as u32 + 1], (2 * p - 2) % p),
                (vec![2], 1),
            ];
            let input = vec![f4_fp_normalise_for_test(&f, p)];
            let mut got = Vec::new();
            let report = solve_visit(
                &input,
                1,
                p,
                &F4Options::new(Ordering::Grevlex, 30),
                &[None],
                |s| {
                    got.push(s[0]);
                    false
                },
            )
            .unwrap();
            got.sort();
            assert_eq!(got, (0..p).collect::<Vec<_>>());
            assert!(report.complete);
        }
    }

    fn f4_fp_normalise_for_test(f: &Poly, p: u64) -> Poly {
        normalise(f, p, Ordering::Grevlex)
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn avx2_lazy_accumulation_matches_scalar_at_boundaries() {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }
        let fp = Barrett::new(65521);
        for width in [0, 1, 3, 4, 5, 7, 8, 15, 16, 17, 33, 257, 4097] {
            let pivot: Vec<u32> = (0..width)
                .map(|i| ((i * 7919 + 137) % 65521) as u32)
                .collect();
            for from in [0, width.min(1), width / 3, width] {
                let initial: Vec<u64> = (0..width)
                    .map(|i| ((i * 1_000_003 + 17) as u64) % 65521)
                    .collect();
                let mut scalar = initial.clone();
                let mut vector = initial;
                for update in 0..256 {
                    let factor = ((update * 4093 + 1) % 65521) as u64;
                    add_lazy_scalar(&mut scalar, &pivot, from, factor);
                    // SAFETY: the runtime AVX2 feature was checked above.
                    unsafe { add_lazy_avx2(&mut vector, &pivot, from, factor) };
                    assert_eq!(vector, scalar, "width={width} from={from} update={update}");
                }
                normalize_lazy(&mut scalar, fp);
                normalize_lazy(&mut vector, fp);
                assert_eq!(vector, scalar, "normalization width={width} from={from}");
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn avx512_lazy_accumulation_matches_scalar_at_boundaries() {
        if !std::arch::is_x86_feature_detected!("avx512f")
            || !std::arch::is_x86_feature_detected!("avx2")
        {
            return;
        }
        let fp = Barrett::new(65521);
        for width in [0, 1, 7, 8, 9, 15, 16, 17, 33, 257, 4097] {
            let pivot: Vec<u32> = (0..width)
                .map(|i| ((i * 7919 + 137) % 65521) as u32)
                .collect();
            for from in [0, width.min(1), width / 3, width] {
                let initial: Vec<u64> = (0..width)
                    .map(|i| ((i * 1_000_003 + 17) as u64) % 65521)
                    .collect();
                let mut scalar = initial.clone();
                let mut vector = initial;
                for update in 0..256 {
                    let factor = ((update * 4093 + 1) % 65521) as u64;
                    add_lazy_scalar(&mut scalar, &pivot, from, factor);
                    // SAFETY: the runtime AVX-512F and AVX2 features were checked above.
                    unsafe { add_lazy_avx512(&mut vector, &pivot, from, factor) };
                    assert_eq!(vector, scalar, "width={width} from={from} update={update}");
                }
                normalize_lazy(&mut scalar, fp);
                normalize_lazy(&mut vector, fp);
                assert_eq!(vector, scalar, "normalization width={width} from={from}");
            }
        }
    }

    pub(super) fn poly(terms: &[(&[u32], u64)]) -> Poly {
        terms.iter().map(|(e, c)| (e.to_vec(), *c)).collect()
    }

    #[test]
    fn circle_and_line_over_f7_match_buchberger() {
        // x² + y² − 1, x − y over F_7: y² = 4 → y = ±2
        let p = 7;
        let f = poly(&[(&[2, 0], 1), (&[0, 2], 1), (&[0, 0], 6)]);
        let g = poly(&[(&[1, 0], 1), (&[0, 1], 6)]);
        let r = f4(
            &[f.clone(), g.clone()],
            2,
            p,
            &F4Options::new(Ordering::Grevlex, 8),
        );
        assert!(!r.inconsistent);
        for b in &r.basis {
            for x in 0..p {
                for y in 0..p {
                    if eval(&f, &[x, y], p) == 0 && eval(&g, &[x, y], p) == 0 {
                        assert_eq!(eval(b, &[x, y], p), 0);
                    }
                }
            }
        }
        let s = solve(&[f, g], 2, p, &F4Options::new(Ordering::Grevlex, 8));
        let mut sols = match s.verdict {
            Verdict::Solutions(v) => v,
            other => panic!("{other:?}"),
        };
        sols.sort();
        assert_eq!(sols, vec![vec![2, 2], vec![5, 5]]);
    }

    #[test]
    fn solving_degree_max_sees_the_highest_productive_step() {
        // x⁸ − 1 and x² + x + 1 over F_7 have no common root (x⁸ = 1 and
        // x³ = 1 force x² = 1).  The only pair has degree 8 and falls to a
        // linear element; the pair after it, of degree 2, yields 1.  The last
        // productive step is at degree 2, but the run learned something at
        // degree 8 first.
        let p = 7;
        let f = poly(&[(&[8], 1), (&[0], 6)]);
        let g = poly(&[(&[2], 1), (&[1], 1), (&[0], 1)]);
        let r = f4(&[f, g], 1, p, &F4Options::new(Ordering::Grevlex, 16));
        assert!(r.inconsistent);
        assert_eq!(r.solving_degree, 2);
        assert_eq!(r.solving_degree_max, 8);
        assert!(r.max_cols_to_solution <= r.max_cols);
        assert!(r.steps_to_solution <= r.steps);
    }

    #[test]
    fn staircase_counts_standard_monomials() {
        // (x², y²) leaves 1, x, y, xy; adding xy leaves three; no pure power
        // of y leaves infinitely many.
        let x2: &[u32] = &[2, 0];
        let y2: &[u32] = &[0, 2];
        let xy: &[u32] = &[1, 1];
        assert_eq!(staircase_at_most(&[x2, y2], 2, 10), Some(4));
        assert_eq!(staircase_at_most(&[x2, y2, xy], 2, 10), Some(3));
        assert_eq!(staircase_at_most(&[x2, y2], 2, 3), None);
        assert_eq!(staircase_at_most(&[x2, xy], 2, 100), None);
    }

    // A tiny deterministic generator for the Hilbert-stop tests.
    fn lcg(s: &mut u64) -> u64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s >> 33
    }

    /// `n` random polynomials of degree 2 in `n` variables over `F_p`.
    fn random_quadrics(n: usize, p: u64, seed: u64) -> Vec<Poly> {
        let mut s = seed;
        let mut monos: Vec<Vec<u32>> = vec![vec![0; n]];
        for i in 0..n {
            let mut e = vec![0; n];
            e[i] = 1;
            monos.push(e);
            for j in i..n {
                let mut e = vec![0; n];
                e[i] += 1;
                e[j] += 1;
                monos.push(e);
            }
        }
        (0..n)
            .map(|_| {
                let t: Vec<(Vec<u32>, u64)> =
                    monos.iter().map(|m| (m.clone(), lcg(&mut s) % p)).collect();
                normalise(&t, p, Ordering::Grevlex)
            })
            .collect()
    }

    fn sorted(mut b: Vec<Poly>) -> Vec<Poly> {
        b.sort();
        b
    }

    #[test]
    fn hilbert_function_of_a_complete_intersection() {
        assert_eq!(
            complete_intersection_hilbert(&[2; 6]),
            vec![1, 6, 15, 20, 15, 6, 1]
        );
        assert_eq!(complete_intersection_hilbert(&[2, 3]), vec![1, 2, 2, 1]);
        assert_eq!(
            complete_intersection_hilbert(&[3, 3, 2])
                .iter()
                .sum::<usize>(),
            18
        );
    }

    #[test]
    fn run_log_is_opt_in() {
        let sys = random_quadrics(3, 101, 19);
        let _ = take_run_log();
        let _ = f4(&sys, 3, 101, &F4Options::new(Ordering::Grevlex, 16));
        assert!(take_run_log().steps.is_empty());

        let _ = f4(
            &sys,
            3,
            101,
            &F4Options::new(Ordering::Grevlex, 16).capturing_run_log(),
        );
        assert!(!take_run_log().steps.is_empty());
    }

    #[test]
    fn hilbert_stop_gives_the_same_reduced_basis_as_the_plain_run() {
        // Reports use a process-wide counter by default, so other parallel
        // tests can contaminate an operation-cost comparison. Keep every solve
        // on one worker and compare its existing thread-local operation counts.
        // The basis, applicability and savings assertions remain unchanged.
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap()
            .install(|| {
                let p = 101;
                let (mut applied, mut cheaper) = (0, 0);
                for n in [3usize, 4, 5, 6] {
                    for seed in 0..6u64 {
                        let sys = random_quadrics(n, p, 1000 * n as u64 + seed);
                        let counted = |options: &F4Options| {
                            FIELD_OPS_THREAD.with(|counter| {
                                let before = counter.get();
                                let report = f4(&sys, n, p, options);
                                (report, counter.get() - before)
                            })
                        };
                        let (plain, plain_ops) = counted(&F4Options::new(Ordering::Grevlex, 24));
                        for chunk in [1usize, 3, usize::MAX] {
                            let (h, hilbert_ops) = counted(
                                &F4Options::new(Ordering::Grevlex, 24).stopping_by_hilbert(chunk),
                            );
                            assert_eq!(
                                sorted(h.basis.clone()),
                                sorted(plain.basis.clone()),
                                "n = {n}, seed {seed}, chunk {chunk}, outcome {:?}",
                                h.hilbert
                            );
                            if h.hilbert == HilbertStop::Applied {
                                applied += 1;
                                assert!(
                                    h.hilbert_check_ops > 0 && h.hilbert_check_ops <= h.field_ops
                                );
                                if chunk == 1 && hilbert_ops < plain_ops {
                                    cheaper += 1;
                                }
                            }
                        }
                    }
                }
                // random quadrics are regular with overwhelming probability
                assert!(applied >= 60, "{applied}");
                assert!(cheaper >= 20, "{cheaper}");
            });
    }

    #[test]
    fn hilbert_stop_falls_back_when_the_leading_forms_are_not_regular() {
        let p = 101;
        // two conics with the same leading form xy: a solution at infinity
        let f1: Poly = normalise(&[(vec![1, 1], 1), (vec![0, 0], 1)], p, Ordering::Grevlex);
        let f2: Poly = normalise(
            &[(vec![1, 1], 1), (vec![1, 0], 1), (vec![0, 0], 2)],
            p,
            Ordering::Grevlex,
        );
        let sys = vec![f1, f2];
        let plain = f4(&sys, 2, p, &F4Options::new(Ordering::Grevlex, 12));
        let h = f4(
            &sys,
            2,
            p,
            &F4Options::new(Ordering::Grevlex, 12).stopping_by_hilbert(1),
        );
        assert_eq!(h.hilbert, HilbertStop::CheckFailed);
        assert_eq!(sorted(h.basis), sorted(plain.basis));
        // a non-square system is never certified
        let mut three = random_quadrics(2, p, 5);
        three.push(random_quadrics(2, p, 6).remove(0));
        let plain = f4(&three, 2, p, &F4Options::new(Ordering::Grevlex, 12));
        let h = f4(
            &three,
            2,
            p,
            &F4Options::new(Ordering::Grevlex, 12).stopping_by_hilbert(1),
        );
        assert_eq!(h.hilbert, HilbertStop::CheckFailed);
        assert_eq!(sorted(h.basis), sorted(plain.basis));
    }

    #[test]
    fn stopping_below_a_staircase_keeps_every_solution() {
        // Two Kummer towers over F_17 (y0² = y1, y1² = 1 and the same for
        // z), linked by y0 + z0 = c.  The towers alone leave 16 standard
        // monomials, so zero-dimensionality is there from the start and must
        // not stop the run; a bound of 2 may.  When it does, every root of
        // the system must still be a zero of every element of the basis, and
        // the staircase it stopped at bounds the number of roots.
        let p = 17;
        // variables: y0, y1, z0, z1
        let tower = |a: usize, b: usize| {
            let mut sq = vec![0u32; 4];
            sq[a] = 2;
            let mut nx = vec![0u32; 4];
            nx[b] = 1;
            vec![(sq, 1), (nx, p - 1)]
        };
        let last = |a: usize| {
            let mut sq = vec![0u32; 4];
            sq[a] = 2;
            vec![(sq, 1), (vec![0; 4], p - 1)]
        };
        for c in 0..p {
            let link = vec![
                (vec![1, 0, 0, 0], 1),
                (vec![0, 0, 1, 0], 1),
                (vec![0; 4], (p - c) % p),
            ];
            let sys: Vec<Poly> = vec![tower(0, 1), last(1), tower(2, 3), last(3), link];
            let mut roots = Vec::new();
            for y0 in 0..p {
                for z0 in 0..p {
                    let x = [y0, y0 * y0 % p, z0, z0 * z0 % p];
                    if sys.iter().all(|f| eval(f, &x, p) == 0) {
                        roots.push(x);
                    }
                }
            }
            let r = f4(
                &sys,
                4,
                p,
                &F4Options::new(Ordering::Grevlex, 12).stopping_below(2),
            );
            if r.inconsistent {
                assert!(roots.is_empty(), "c = {c}: refuted a system with roots");
                continue;
            }
            if let Some(s) = r.staircase_at_stop {
                assert!(
                    s <= 2 && s >= roots.len(),
                    "c = {c}: stopped at {s} with {} roots",
                    roots.len()
                );
            }
            for x in &roots {
                for b in &r.basis {
                    assert_eq!(eval(b, x, p), 0, "c = {c}: a basis element misses a root");
                }
            }
            // A bound of zero can only be met by 1, which is a refutation, so
            // it never stops a run early.
            let full = f4(
                &sys,
                4,
                p,
                &F4Options::new(Ordering::Grevlex, 12).stopping_below(0),
            );
            assert_eq!(full.staircase_at_stop, None);
            assert_eq!(full.inconsistent, roots.is_empty(), "c = {c}");
        }
    }

    #[test]
    fn autoreduce_keeps_generators_with_equal_leading_monomials() {
        // x² + y and x² + 2y generate (x², y): interreduce would wrongly
        // drop one of them, autoreduce must keep two generators
        let p = 31;
        let f = poly(&[(&[2, 0], 1), (&[0, 1], 1)]);
        let g = poly(&[(&[2, 0], 1), (&[0, 1], 2)]);
        let a = autoreduce(&[f.clone(), g.clone()], p, Ordering::Grevlex);
        assert_eq!(a.len(), 2, "{a:?}");
        assert!(a.iter().any(|h| h == &poly(&[(&[2, 0], 1)])), "{a:?}");
        assert!(a.iter().any(|h| h == &poly(&[(&[0, 1], 1)])), "{a:?}");
        // multiples of a generator are dropped
        let m = poly(&[(&[3, 1], 1), (&[1, 2], 1)]); // x·y·(x² + y)
        let a = autoreduce(&[f.clone(), m], p, Ordering::Grevlex);
        assert_eq!(a.len(), 1, "{a:?}");
    }

    #[test]
    fn inconsistent_system_is_refuted() {
        let p = 31;
        let f = poly(&[(&[1, 0], 1), (&[0, 0], 1)]); // x + 1
        let g = poly(&[(&[1, 0], 1), (&[0, 0], 2)]); // x + 2
        let r = f4(&[f, g], 2, p, &F4Options::new(Ordering::Grevlex, 4));
        assert!(r.inconsistent);
    }

    #[test]
    fn degree_bound_truncates_and_reports_it() {
        // x³ − y, y³ − x, x·y − 1 over F_101 has finitely many solutions;
        // a bound of 2 cannot form the degree-3 pairs.
        let p = 101;
        let f = poly(&[(&[3, 0], 1), (&[0, 1], p - 1)]);
        let g = poly(&[(&[0, 3], 1), (&[1, 0], p - 1)]);
        let h = poly(&[(&[1, 1], 1), (&[0, 0], p - 1)]);
        let r = f4(
            &[f.clone(), g.clone(), h.clone()],
            2,
            p,
            &F4Options::new(Ordering::Grevlex, 2),
        );
        assert!(r.pairs_above_bound > 0);
        let s = solve(
            &[f.clone(), g.clone(), h.clone()],
            2,
            p,
            &F4Options::new(Ordering::Grevlex, 8),
        );
        let sols = match s.verdict {
            Verdict::Solutions(v) => v,
            other => panic!("{other:?}"),
        };
        for x in 0..p {
            for y in 0..p {
                let on = [&f, &g, &h].iter().all(|q| eval(q, &[x, y], p) == 0);
                assert_eq!(on, sols.contains(&vec![x, y]), "({x}, {y})");
            }
        }
    }

    #[test]
    fn random_zero_dimensional_systems_agree_with_brute_force() {
        // three random quadrics in three unknowns over F_13, plus the
        // field equations' role played by brute force
        let p = 13;
        let mut seed = 0x1234_5678u64;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % p
        };
        let monos: Vec<Vec<u32>> = {
            let mut v = Vec::new();
            for a in 0..=2 {
                for b in 0..=2 - a {
                    for c in 0..=2 - a - b {
                        v.push(vec![a, b, c]);
                    }
                }
            }
            v
        };
        for _ in 0..6 {
            let sys: Vec<Poly> = (0..3)
                .map(|_| monos.iter().map(|m| (m.clone(), rnd())).collect())
                .collect();
            let s = solve(&sys, 3, p, &F4Options::new(Ordering::Grevlex, 10));
            let mut expected: Vec<Vec<u64>> = Vec::new();
            for x in 0..p {
                for y in 0..p {
                    for z in 0..p {
                        if sys.iter().all(|q| eval(q, &[x, y, z], p) == 0) {
                            expected.push(vec![x, y, z]);
                        }
                    }
                }
            }
            let mut got = match s.verdict {
                Verdict::Solutions(v) => v,
                Verdict::Inconsistent => vec![],
                Verdict::Undetermined => panic!("undetermined"),
            };
            got.sort();
            expected.sort();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn barrett_matches_the_hardware_remainder() {
        let mut x = 0x0123_4567_89ab_cdefu64;
        for p in [2u64, 3, 31, 65_521, 2_147_483_647, 4_294_967_291] {
            let fp = Barrett::new(p);
            for probe in [0, 1, p - 1, p, p + 1, p * p - 1, u64::MAX, u64::MAX - 1] {
                assert_eq!(fp.reduce(probe), probe % p, "{probe} mod {p}");
            }
            for _ in 0..10_000 {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                assert_eq!(fp.reduce(x), x % p, "{x} mod {p}");
            }
        }
    }

    /// The blocked `D − C·B'` equals the product computed entry by entry,
    /// across tile boundaries, late-starting `B'` rows and several blocks.
    #[test]
    fn blocked_pair_reduction_matches_direct_product() {
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for p in [31u64, 65_521] {
            let fp = Barrett::new(p);
            for (n_red, n_free, n_pairs) in [(3, 5, 2), (40, 300, 70), (90, 600, 130)] {
                // reducers lead columns 0..n_red; the free columns follow
                let n_cols = n_red + n_free;
                let reducer_at: Vec<u32> = (0..n_cols)
                    .map(|c| if c < n_red { c as u32 } else { u32::MAX })
                    .collect();
                let free_index: Vec<u32> = (0..n_cols)
                    .map(|c| {
                        if c < n_red {
                            u32::MAX
                        } else {
                            (c - n_red) as u32
                        }
                    })
                    .collect();
                let bstart: Vec<usize> = (0..n_red).map(|_| next() as usize % n_free).collect();
                let bprime: Vec<Vec<u32>> = bstart
                    .iter()
                    .map(|&b| {
                        (0..n_free)
                            .map(|i| if i < b { 0 } else { (next() % p) as u32 })
                            .collect()
                    })
                    .collect();
                let mut pair_rows: Vec<Vec<(u32, u64)>> = Vec::new();
                for _ in 0..n_pairs {
                    let mut row = Vec::new();
                    for c in 0..n_cols {
                        if next() % 3 == 0 {
                            row.push((c as u32, 1 + next() % (p - 1)));
                        }
                    }
                    pair_rows.push(row);
                }
                let got = reduce_pairs_blocked(
                    &pair_rows,
                    &bprime,
                    &bstart,
                    &reducer_at,
                    &free_index,
                    n_free,
                    fp,
                    None,
                )
                .unwrap();
                for (r, row) in pair_rows.iter().zip(&got) {
                    let mut want = vec![0u64; n_free];
                    for &(c, v) in r {
                        let c = c as usize;
                        if c >= n_red {
                            want[c - n_red] = (want[c - n_red] + v) % p;
                        } else {
                            for (w, &b) in want.iter_mut().zip(&bprime[c]) {
                                *w = (*w + (p - v) * u64::from(b)) % p;
                            }
                        }
                    }
                    let row: Vec<u64> = row.iter().map(|&v| u64::from(v)).collect();
                    assert_eq!(row, want, "p={p} n_red={n_red} n_free={n_free}");
                }
            }
        }
    }

    #[test]
    fn narrow_arithmetic_matches_wide_rref() {
        let mut x = 0x0123_4567_89ab_cdefu64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        for p in [2u64, 3, 29, 31, 101, 65_521] {
            let fp = Barrett::new(p);
            let narrow = Barrett32::new(p);
            let max_update = (p * (p - 1)) as u32;
            for probe in [0, 1, (p - 1) as u32, p as u32, max_update - 1] {
                assert_eq!(narrow.reduce(probe) as u64, fp.reduce(probe as u64));
            }
            for _ in 0..10_000 {
                let probe = (next() % p.saturating_mul(p - 1)) as u32;
                assert_eq!(narrow.reduce(probe) as u64, probe as u64 % p);
            }
            for (rows, cols) in [(1, 1), (7, 9), (20, 15), (25, 40)] {
                let mut wide: Vec<Vec<u64>> = (0..rows)
                    .map(|_| (0..cols).map(|_| next() % p).collect())
                    .collect();
                let mut narrow_rows: Vec<Vec<u32>> = wide
                    .iter()
                    .map(|row| row.iter().map(|&v| v as u32).collect())
                    .collect();
                let mut deferred_rows = narrow_rows.clone();
                let want = rref(&mut wide, fp, None).unwrap();
                let got = rref32(&mut narrow_rows, narrow, None).unwrap();
                let got_deferred = rref32_deferred(&mut deferred_rows, narrow, fp, None).unwrap();
                assert_eq!(got, want, "p={p}, shape={rows}x{cols}");
                assert_eq!(got_deferred, want, "deferred p={p}, shape={rows}x{cols}");
                let widened: Vec<Vec<u64>> = narrow_rows
                    .into_iter()
                    .map(|row| row.into_iter().map(u64::from).collect())
                    .collect();
                assert_eq!(widened, wide, "p={p}, shape={rows}x{cols}");
                let widened_deferred: Vec<Vec<u64>> = deferred_rows
                    .into_iter()
                    .map(|row| row.into_iter().map(u64::from).collect())
                    .collect();
                assert_eq!(
                    widened_deferred, wide,
                    "deferred p={p}, shape={rows}x{cols}"
                );
            }
        }
    }

    #[test]
    fn narrow_f4_matches_wide_basis() {
        let monomials = [
            vec![0, 0, 0],
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![2, 0, 0],
            vec![0, 2, 0],
            vec![0, 0, 2],
            vec![1, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 1],
        ];
        let mut seed = 0x5eed_2026_u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for p in [29u64, 31, 65_521] {
            let sys: Vec<Poly> = (0..3)
                .map(|_| monomials.iter().map(|m| (m.clone(), next() % p)).collect())
                .collect();
            let opts = F4Options::new(Ordering::Grevlex, 6);
            let wide = f4_with_arithmetic(&sys, 3, p, &opts, 0, TraceMode::Off);
            let narrow = f4_with_arithmetic(&sys, 3, p, &opts, 1, TraceMode::Off);
            let deferred = f4_with_arithmetic(&sys, 3, p, &opts, 2, TraceMode::Off);
            assert_eq!(narrow.basis, wide.basis, "p={p}");
            assert_eq!(narrow.steps, wide.steps, "p={p}");
            assert_eq!(
                (narrow.max_rows, narrow.max_cols),
                (wide.max_rows, wide.max_cols)
            );
            assert_eq!(narrow.inconsistent, wide.inconsistent, "p={p}");
            assert_eq!(deferred.basis, wide.basis, "deferred p={p}");
            assert_eq!(deferred.steps, wide.steps, "deferred p={p}");
            assert_eq!(
                (deferred.max_rows, deferred.max_cols),
                (wide.max_rows, wide.max_cols),
                "deferred p={p}"
            );
            assert_eq!(deferred.inconsistent, wide.inconsistent, "deferred p={p}");
        }
    }

    /// `S(f, g)` of two monic polynomials.
    fn spoly(f: &Poly, g: &Poly, p: u64, ord: Ordering) -> Poly {
        let l: Vec<u32> = f[0].0.iter().zip(&g[0].0).map(|(a, b)| *a.max(b)).collect();
        let mut terms: Vec<(Vec<u32>, u64)> = Vec::new();
        for (h, sign) in [(f, 1), (g, p - 1)] {
            for (e, c) in h {
                let em = e
                    .iter()
                    .zip(&l)
                    .zip(&h[0].0)
                    .map(|((x, a), b)| x + a - b)
                    .collect();
                terms.push((em, mulmod(*c, sign, p)));
            }
        }
        let mut map: BTreeMap<Vec<u32>, u64> = BTreeMap::new();
        for (e, c) in terms {
            let v = map.entry(e).or_insert(0);
            *v = (*v + c) % p;
        }
        let mut out: Poly = map.into_iter().filter(|(_, c)| *c != 0).collect();
        out.sort_by(|(a, _), (b, _)| cmp_monomial(b, a, ord));
        out
    }

    #[test]
    fn basis_from_the_parallel_path_is_a_reduced_groebner_basis() {
        // six random quadrics in six unknowns over F_65521: the larger
        // steps exceed `PAR_WORK` and run the row reductions in parallel
        let (n, p, ord) = (6usize, 65_521u64, Ordering::Grevlex);
        let mut seed = 0x0bad_c0de_1234_5678u64;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % p
        };
        let mut monos: Vec<Vec<u32>> = Vec::new();
        for m in 0u32..(3u32.pow(n as u32)) {
            let e: Vec<u32> = (0..n).map(|i| (m / 3u32.pow(i as u32)) % 3).collect();
            if e.iter().sum::<u32>() <= 2 {
                monos.push(e);
            }
        }
        let sys: Vec<Poly> = (0..n)
            .map(|_| monos.iter().map(|m| (m.clone(), rnd())).collect())
            .collect();
        let r = f4(&sys, n, p, &F4Options::new(ord, 20));
        assert!(!r.inconsistent && !r.timed_out);
        assert_eq!(r.pairs_above_bound, 0);
        assert!(r.max_rows * r.max_cols > PAR_WORK, "not the parallel path");
        let g = &r.basis;
        for f in &sys {
            assert!(reduce(&normalise(f, p, ord), g, p, ord).is_empty());
        }
        for i in 0..g.len() {
            for j in i + 1..g.len() {
                let s = spoly(&g[i], &g[j], p, ord);
                assert!(reduce(&s, g, p, ord).is_empty(), "S({i}, {j})");
            }
            // reduced: no term of g[i] is divisible by another leading monomial
            for (e, _) in &g[i] {
                for (j, h) in g.iter().enumerate() {
                    assert!(
                        j == i || !divides(&h[0].0, e),
                        "g[{i}] not reduced by g[{j}]"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod degree_bound_solve_tests {
    use super::tests::poly;
    use super::*;

    /// `{x³ + 1, xy − 1, x + y}` over `F_5` is inconsistent (x + y = 0 and
    /// xy = 1 force x² = −1, so x⁴ = 1 and x³ + 1 = x⁻¹ + 1 ≠ 0 for every
    /// root of x² + 1, i.e. x ∈ {2, 3}).  At degree bound 2 the step finds
    /// x² + 1, retires x³ + 1, and drops their degree-3 pair: the truncated
    /// basis no longer generates the input ideal, so a solver that trusts
    /// it reports solutions of a system that has none.
    #[test]
    fn truncated_run_must_not_certify_solutions() {
        let p = 5;
        let f = poly(&[(&[3, 0], 1), (&[0, 0], 1)]); // x³ + 1
        let g = poly(&[(&[1, 1], 1), (&[0, 0], p - 1)]); // xy − 1
        let h = poly(&[(&[1, 0], 1), (&[0, 1], 1)]); // x + y
        let sys = [f, g, h];
        let r = f4(&sys, 2, p, &F4Options::new(Ordering::Grevlex, 2));
        assert!(
            r.pairs_above_bound > 0,
            "the counterexample needs a dropped pair"
        );
        let s = solve(&sys, 2, p, &F4Options::new(Ordering::Grevlex, 2));
        assert!(
            matches!(s.verdict, Verdict::Inconsistent),
            "{:?}",
            s.verdict
        );
        assert_eq!(
            s.candidates_rejected, 2,
            "both spurious roots x ∈ {{2, 3}} are dropped"
        );
        // With a sufficient bound the verdict is exact.
        let s = solve(&sys, 2, p, &F4Options::new(Ordering::Grevlex, 8));
        assert!(
            matches!(s.verdict, Verdict::Inconsistent),
            "{:?}",
            s.verdict
        );
    }
}

#[cfg(test)]
mod progress_observer_tests {
    use super::*;
    #[test]
    fn observing_and_nested_observers_preserve_the_basis_and_restore_scope() {
        let input = vec![
            normalise(&[(vec![2, 0], 1), (vec![0, 1], 4)], 5, Ordering::Grevlex),
            normalise(&[(vec![1, 1], 1), (vec![0, 0], 4)], 5, Ordering::Grevlex),
        ];
        let opts = F4Options::new(Ordering::Grevlex, 8);
        let reference = f4(&input, 2, 5, &opts);
        let (outer, outer_rx) = std::sync::mpsc::channel();
        let (inner, inner_rx) = std::sync::mpsc::channel();
        let observed = observe_progress(outer, || {
            observe_progress(inner, || {
                let _ = f4(&input, 2, 5, &opts);
            });
            f4(&input, 2, 5, &opts)
        });
        assert_eq!(reference.basis, observed.basis);
        assert_eq!(reference.solving_degree_max, observed.solving_degree_max);
        assert_eq!(reference.degree_reached, observed.degree_reached);
        for events in [
            outer_rx.into_iter().collect::<Vec<_>>(),
            inner_rx.into_iter().collect::<Vec<_>>(),
        ] {
            assert_eq!(events.first().unwrap().event, "begin");
            assert_eq!(events.last().unwrap().event, "complete");
            assert!(events.iter().any(|e| e.event == "matrix"));
        }
        let _ = f4(&input, 2, 5, &opts);
    }
    #[test]
    fn observer_is_restored_after_unwind() {
        let (tx, rx) = std::sync::mpsc::channel();
        let result =
            std::panic::catch_unwind(|| observe_progress(tx, || panic!("observer scope control")));
        assert!(result.is_err());
        assert!(rx.recv().is_err());
        let _ = f4(
            &[normalise(&[(vec![1], 1)], 3, Ordering::Grevlex)],
            1,
            3,
            &F4Options::new(Ordering::Grevlex, 2),
        );
    }
}

/// `interreduce`, `autoreduce` and `normalise` on packed monomials and sorted
/// terms, against the exponent-vector and ordered-map code they replace.
#[cfg(test)]
mod packed_set_tests {
    use super::tests::poly;
    use super::*;
    use std::collections::BTreeMap;
    use std::hash::Hasher;

    /// A small deterministic generator.
    struct Xs(u64);

    impl Xs {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    /// Every exponent vector of total degree at most `d` in `n` variables.
    fn monomials_up_to(n: usize, d: u32) -> Vec<Vec<u32>> {
        fn go(k: usize, left: u32, cur: &mut Vec<u32>, out: &mut Vec<Vec<u32>>) {
            if k == cur.len() {
                out.push(cur.clone());
                return;
            }
            for e in 0..=left {
                cur[k] = e;
                go(k + 1, left - e, cur, out);
            }
            cur[k] = 0;
        }
        let mut out = Vec::new();
        go(0, d, &mut vec![0; n], &mut out);
        out
    }

    /// `m` dense random polynomials of degree at most `d` in `n` variables.
    fn random_dense(n: usize, m: usize, d: u32, p: u64, rng: &mut Xs) -> Vec<Poly> {
        let monos = monomials_up_to(n, d);
        (0..m)
            .map(|_| {
                let f: Vec<_> = monos.iter().map(|e| (e.clone(), rng.below(p))).collect();
                normalise(&f, p, Ordering::Grevlex)
            })
            .collect()
    }

    #[test]
    fn packed_monomials_behave_like_exponent_vectors() {
        let mut rng = Xs(0xfeed_beef);
        for n in [1usize, 2, 3, 7, 8, 9, 12, 15, 16] {
            let mono = |rng: &mut Xs, top: u64| -> Vec<u32> {
                (0..n).map(|_| rng.below(top) as u32).collect()
            };
            for _ in 0..400 {
                // small exponents, so that equal and dividing pairs occur
                let (a, b) = (mono(&mut rng, 4), mono(&mut rng, 4));
                let (pa, pb) = (set_pack(&a).unwrap(), set_pack(&b).unwrap());
                assert_eq!(unpack(pa, n), a);
                assert_eq!(set_degree(pa), total_degree(&a));
                assert_eq!(packed_divides(pa, pb), divides(&a, &b), "{a:?} | {b:?}");
                assert!(!packed_divides(SET_NEVER, pa));
                let prod: Vec<u32> = a.iter().zip(&b).map(|(x, y)| x + y).collect();
                assert_eq!(set_mul(pa, pb), Some(set_pack(&prod).unwrap()));
                if divides(&a, &b) {
                    let q: Vec<u32> = b.iter().zip(&a).map(|(x, y)| x - y).collect();
                    assert_eq!(pb - pa, set_pack(&q).unwrap());
                }
                for ord in [Ordering::Lex, Ordering::Grlex, Ordering::Grevlex] {
                    // the key sorts ascending where cmp_monomial sorts descending
                    assert_eq!(
                        set_order_key(ord, pa).cmp(&set_order_key(ord, pb)),
                        cmp_monomial(&b, &a, ord),
                        "{ord:?}: {a:?} vs {b:?}"
                    );
                    assert_eq!(set_from_order_key(ord, set_order_key(ord, pa)), pa);
                }
            }
            // exponents up to 127 pack, with the degree up to 16 · 127
            let top = vec![127u32; n];
            let ptop = set_pack(&top).unwrap();
            assert_eq!(unpack(ptop, n), top);
            assert_eq!(set_degree(ptop), 127 * n as u32);
            assert!(packed_divides(ptop, ptop));
            assert!(!packed_divides(SET_NEVER, ptop));
            // and the first that does not
            let mut over = vec![0u32; n];
            over[n - 1] = 128;
            assert_eq!(set_pack(&over), None);
            assert_eq!(set_mul(ptop, set_pack(&vec![1; n]).unwrap()), None);
            assert_eq!(
                set_mul(
                    set_pack(&vec![64; n]).unwrap(),
                    set_pack(&vec![64; n]).unwrap()
                ),
                None
            );
            let mut small = vec![0u32; n];
            small[0] = 64;
            assert!(set_mul(set_pack(&small).unwrap(), set_pack(&small).unwrap()).is_none());
            small[0] = 63;
            assert!(set_mul(set_pack(&small).unwrap(), set_pack(&small).unwrap()).is_some());
        }
        assert_eq!(set_pack(&[0; 17]), None);
        assert_eq!(set_pack(&[0; 16]), Some(0));
        // the hasher the work map uses spreads neighbouring monomials
        let mut seen = HashSet::new();
        for a in 0..8u32 {
            for b in 0..8u32 {
                for c in 0..8u32 {
                    let mut h = PackedHasher::default();
                    h.write_u128(set_pack(&[a, b, c, 0, 0, 0, 0, 0, 0]).unwrap());
                    seen.insert(h.finish() & 0x3ff);
                }
            }
        }
        // 512 keys into 1024 buckets: a random function leaves about 403
        assert!(
            seen.len() > 340,
            "{} of 512 monomials hash apart",
            seen.len()
        );
    }

    #[test]
    fn first_divisor_skips_only_the_skipped() {
        let lm = |e: &[u32]| set_pack(e).unwrap();
        let lms = [
            lm(&[2, 0]),
            SET_NEVER,
            lm(&[1, 0]),
            lm(&[1, 1]),
            lm(&[0, 0]),
        ];
        let x = lm(&[1, 1]);
        assert_eq!(first_divisor(&lms, usize::MAX, x), Some(2));
        assert_eq!(first_divisor(&lms, 2, x), Some(3));
        assert_eq!(first_divisor(&lms, 3, x), Some(2));
        assert_eq!(first_divisor(&lms, 4, x), Some(2));
        assert_eq!(first_divisor(&lms, 5, x), Some(2));
        assert_eq!(first_divisor(&lms[..3], 2, lm(&[0, 5])), None);
        assert_eq!(first_divisor(&lms, 4, lm(&[0, 5])), None);
        assert_eq!(first_divisor(&[], 0, x), None);
    }

    /// Reduction by a set with its monomials packed against the vector one,
    /// for sets that reduce a good deal (multiples of one another).
    #[test]
    fn packed_reduction_matches_the_vector_reduction() {
        let mut rng = Xs(0x1357_9bdf_2468_ace0);
        for (n, p, ord) in [
            (4usize, 31u64, Ordering::Grevlex),
            (5, 65_521, Ordering::Grevlex),
            (3, 13, Ordering::Grlex),
            (3, 786_433, Ordering::Lex),
            (3, 4_294_967_291, Ordering::Grevlex),
        ] {
            let monos = monomials_up_to(n, 3);
            let gens: Vec<Poly> = (0..5)
                .map(|_| {
                    let k = 2 + rng.below(5) as usize;
                    let f: Vec<_> = (0..k)
                        .map(|_| {
                            (
                                monos[rng.below(monos.len() as u64) as usize].clone(),
                                1 + rng.below(p - 1),
                            )
                        })
                        .collect();
                    normalise(&f, p, ord)
                })
                .filter(|f| !f.is_empty())
                .collect();
            // the generators and their multiples by the monomials of degree <= 1
            let mut set = gens.clone();
            for f in &gens {
                for m in monomials_up_to(n, 1) {
                    let g: Vec<_> = f
                        .iter()
                        .map(|(e, c)| (e.iter().zip(&m).map(|(a, b)| a + b).collect(), *c))
                        .collect();
                    set.push(normalise(&g, p, ord));
                }
            }
            let what = format!("n={n} p={p} {ord:?}");
            assert_eq!(
                autoreduce_with(&set, p, ord, true),
                autoreduce_with(&set, p, ord, false),
                "autoreduce {what}"
            );
            let basis = f4(&gens, n, p, &F4Options::new(Ordering::Grevlex, 8)).basis;
            if !basis.is_empty() && ord == Ordering::Grevlex {
                assert_eq!(
                    interreduce_with(&basis, p, ord, true),
                    interreduce_with(&basis, p, ord, false),
                    "interreduce {what}"
                );
            }
            // one reduction at a time, skipping each element in turn
            let ps = PackedSet::new(&set).unwrap();
            for i in 0..set.len() {
                assert_eq!(
                    reduce_skipping_packed(&set[i], &set, &ps, i, p, ord).unwrap(),
                    reduce_skipping_vecs(&set[i], &set, i, p, ord),
                    "{what}: element {i}"
                );
            }
            // `make_monic` is what `normalise` leaves to do
            for i in 0..set.len() {
                let r = reduce_skipping_vecs(&set[i], &set, i, p, ord);
                if !r.is_empty() {
                    assert_eq!(make_monic(r.clone(), p), normalise(&r, p, ord), "{what}");
                }
            }
        }
    }

    #[test]
    fn reduction_by_what_does_not_pack_falls_back() {
        let p = 31;
        let ord = Ordering::Grevlex;
        // an exponent above 127 in the set, and in the polynomial reduced
        let set = vec![
            poly(&[(&[130, 0], 1), (&[0, 1], 3)]),
            poly(&[(&[0, 2], 1), (&[0, 0], 5)]),
            poly(&[(&[1, 1], 1), (&[0, 0], 1)]),
        ];
        let ps = PackedSet::new(&set).unwrap();
        assert!(ps.terms[0].is_none() && ps.lms[0] == SET_NEVER);
        assert!(ps.terms[1].is_some());
        let f = poly(&[(&[0, 3], 2), (&[1, 1], 4)]);
        let want = reduce_skipping_vecs(&f, &set, usize::MAX, p, ord);
        assert_eq!(
            reduce_skipping_packed(&f, &set, &ps, usize::MAX, p, ord),
            Some(want)
        );
        let big = poly(&[(&[131, 0], 2)]);
        assert_eq!(
            reduce_skipping_packed(&big, &set, &ps, usize::MAX, p, ord),
            None
        );
        // the dispatcher gives the vector answer
        let mut sets = set.clone();
        sets.push(big);
        let ps = PackedSet::new(&sets).unwrap();
        let last = sets.len() - 1;
        assert_eq!(
            reduce_skipping(&sets, Some(&ps), last, p, ord),
            reduce_skipping_vecs(&sets[last], &sets, last, p, ord)
        );
        assert_eq!(
            autoreduce_with(&sets, p, ord, true),
            autoreduce_with(&sets, p, ord, false)
        );
    }

    /// Polynomials in different numbers of unknowns are not packed at all
    /// (the exponent-vector code compares only what the shorter has): the
    /// result is the vector code's.
    #[test]
    fn a_set_of_mixed_widths_is_not_packed() {
        let p = 31;
        let ord = Ordering::Grevlex;
        let set = vec![
            poly(&[(&[1, 1], 1), (&[0, 0], 1)]),
            poly(&[(&[1, 0, 0], 1), (&[0, 0, 1], 3)]),
        ];
        assert!(PackedSet::new(&set).is_none());
        assert_eq!(
            interreduce_with(&set, p, ord, true),
            interreduce_with(&set, p, ord, false)
        );
        assert_eq!(
            autoreduce_with(&set, p, ord, true),
            autoreduce_with(&set, p, ord, false)
        );
    }

    /// A set element whose lead packs and whose tail does not (under lex a
    /// tail can carry a bigger exponent than the lead) still divides
    /// monomials of the others, so the packed reduction must not pass it by.
    #[test]
    fn an_element_with_an_unpackable_tail_still_reduces_the_others() {
        let p = 31;
        let ord = Ordering::Lex;
        // x·y + z²⁰⁰ and x² + x·y·z, whose term x·y·z is divisible by x·y
        let g = poly(&[(&[1, 1, 0], 1), (&[0, 0, 200], 1)]);
        let h = poly(&[(&[2, 0, 0], 1), (&[1, 1, 1], 1)]);
        let set = vec![g, h];
        let want = interreduce_with(&set, p, ord, false);
        assert!(want.contains(&poly(&[(&[2, 0, 0], 1), (&[0, 0, 201], p - 1)])));
        assert_eq!(interreduce_with(&set, p, ord, true), want);
        assert_eq!(interreduce(&set, p, ord), want);
        assert_eq!(
            autoreduce_with(&set, p, ord, true),
            autoreduce_with(&set, p, ord, false)
        );
    }

    /// Under lex the exponents of a run's basis grow far past the degrees of
    /// the input (here to about 110, in an ordinary dense system with a small
    /// degree bound; one degree more takes them to the thousands, and the
    /// run to minutes), and `f4` ends by interreducing it.  That basis is
    /// interreduced already, so the exponent-vector interreduction leaves it
    /// as it is, and so does the packed one.
    #[test]
    fn lex_bases_with_large_exponents_are_interreduced() {
        let p = 536_870_909;
        let ord = Ordering::Lex;
        let sys = random_dense(4, 5, 2, p, &mut Xs(0xd1ff_000a));
        let r = f4(&sys, 4, p, &F4Options::new(ord, 9));
        let top = r.basis.iter().flatten().flat_map(|(e, _)| e).max();
        assert!(top > Some(&64), "the exponents did not grow: {top:?}");
        assert_eq!(interreduce_with(&r.basis, p, ord, false), r.basis);
        assert_eq!(interreduce_with(&r.basis, p, ord, true), r.basis);
    }

    /// An element that packs only in its lead keeps that lead in the packed
    /// set, where one whose lead does not pack has none.
    #[test]
    fn a_lead_that_packs_is_kept_when_the_tail_does_not() {
        let set = vec![
            poly(&[(&[1, 1, 0], 1), (&[0, 0, 200], 1)]),
            poly(&[(&[130, 0, 0], 1), (&[0, 0, 1], 3)]),
            poly(&[(&[2, 0, 0], 1), (&[0, 1, 1], 5)]),
        ];
        let ps = PackedSet::new(&set).unwrap();
        assert!(ps.terms[0].is_none() && ps.lms[0] == set_pack(&[1, 1, 0]).unwrap());
        assert!(ps.terms[1].is_none() && ps.lms[1] == SET_NEVER);
        assert!(ps.terms[2].is_some() && ps.lms[2] == set_pack(&[2, 0, 0]).unwrap());
        // a replacement is looked at again
        let mut ps = ps;
        ps.replace(0, &poly(&[(&[1, 1, 0], 1), (&[0, 0, 2], 1)]));
        assert!(ps.terms[0].is_some() && ps.lms[0] == set_pack(&[1, 1, 0]).unwrap());
        ps.replace(2, &poly(&[(&[200, 0, 0], 1)]));
        assert!(ps.terms[2].is_none() && ps.lms[2] == SET_NEVER);
        ps.remove(1);
        assert_eq!((ps.terms.len(), ps.lms.len()), (2, 2));
    }

    /// The packed reduction gives the vector reduction's answer or gives up,
    /// and gives up exactly when the divisor it would take has a tail that
    /// does not pack.
    #[test]
    fn a_tail_that_does_not_pack_only_matters_to_the_element_that_divides() {
        let p = 31;
        let ord = Ordering::Lex;
        let hole = poly(&[(&[1, 1, 0], 1), (&[0, 0, 200], 1)]); // lead x·y
        let plain = poly(&[(&[1, 0, 0], 1), (&[0, 0, 1], 3)]); // x + 3z
        let g = poly(&[(&[1, 1, 1], 1), (&[0, 1, 0], 2)]); // x·y·z + 2y

        // x·y·z is divided by x + 3z first, so the hole behind it is not taken
        let set = vec![plain.clone(), hole.clone(), g.clone()];
        let ps = PackedSet::new(&set).unwrap();
        let want = reduce_skipping_vecs(&set[2], &set, 2, p, ord);
        assert_eq!(
            reduce_skipping_packed(&set[2], &set, &ps, 2, p, ord),
            Some(want)
        );
        // and by the hole first when it comes first
        let set = vec![hole.clone(), plain, g];
        let ps = PackedSet::new(&set).unwrap();
        assert_eq!(reduce_skipping_packed(&set[2], &set, &ps, 2, p, ord), None);
        // the hole itself has a monomial that does not pack, and is not reduced
        assert_eq!(reduce_skipping_packed(&set[0], &set, &ps, 0, p, ord), None);
        // the dispatcher takes the vector answer
        assert_eq!(
            reduce_skipping(&set, Some(&ps), 2, p, ord),
            reduce_skipping_vecs(&set[2], &set, 2, p, ord)
        );
        assert_ne!(reduce_skipping(&set, Some(&ps), 2, p, ord), set[2]);
    }

    /// Sets whose exponents straddle the packed range, under each order, so
    /// that leads and tails that do and do not pack are mixed: the packed
    /// code agrees with the vector code on every reduction it takes, and
    /// `interreduce` and `autoreduce` are the same with the packed code off.
    #[test]
    fn exponents_straddling_the_packed_range_agree_across_the_paths() {
        let mut rng = Xs(0x5712_adde_0127_0128);
        let n = 3;
        let mut with_a_hole = 0;
        let mut declined = 0;
        let mut taken = 0;
        for (ord, p) in [
            (Ordering::Lex, 31u64),
            (Ordering::Grlex, 31),
            (Ordering::Grevlex, 31),
            (Ordering::Lex, 536_870_909),
        ] {
            for _ in 0..8 {
                let gens: Vec<Poly> = (0..3)
                    .map(|_| {
                        let k = 2 + rng.below(2) as usize;
                        let f: Vec<_> = (0..k)
                            .map(|_| {
                                // small exponents, the first variable the smallest
                                // so that under lex the leads pack and the tails
                                // may not; and often one exponent at the edge of
                                // the packed range or past it (never two: a
                                // reduction would then run through millions of
                                // monomials, in both codes)
                                let mut e: Vec<u32> = (0..n)
                                    .map(|v| rng.below(if v == 0 { 3 } else { 4 }) as u32)
                                    .collect();
                                if rng.below(5) == 0 {
                                    let v = rng.below(n as u64) as usize;
                                    e[v] = [127, 128, 129, 130][rng.below(4) as usize];
                                }
                                (e, 1 + rng.below(p - 1))
                            })
                            .collect();
                        normalise(&f, p, ord)
                    })
                    .filter(|f| !f.is_empty())
                    .collect();
                // the generators and their multiples by each variable
                let mut set = gens.clone();
                for f in &gens {
                    for v in 0..n {
                        let g: Vec<_> = f
                            .iter()
                            .map(|(e, c)| {
                                let mut e = e.clone();
                                e[v] += 1;
                                (e, *c)
                            })
                            .collect();
                        set.push(normalise(&g, p, ord));
                    }
                }
                if set.is_empty() {
                    continue;
                }
                let what = format!("{ord:?} p={p}");
                let ps = PackedSet::new(&set).unwrap();
                with_a_hole += (0..set.len())
                    .filter(|&j| ps.terms[j].is_none() && ps.lms[j] != SET_NEVER)
                    .count();
                for i in 0..set.len() {
                    let want = reduce_skipping_vecs(&set[i], &set, i, p, ord);
                    match reduce_skipping_packed(&set[i], &set, &ps, i, p, ord) {
                        Some(got) => {
                            taken += 1;
                            assert_eq!(got, want, "{what}: element {i}");
                        }
                        None => declined += 1,
                    }
                    assert_eq!(reduce_skipping(&set, Some(&ps), i, p, ord), want);
                }
                // one pass of interreduction over the whole set, and
                // autoreduction (which runs until nothing changes, and on
                // a set with far exponents and all those multiples can run
                // a long time) over the generators alone
                assert_eq!(
                    interreduce_with(&set, p, ord, true),
                    interreduce_with(&set, p, ord, false),
                    "interreduce {what}"
                );
                assert_eq!(
                    autoreduce_with(&gens, p, ord, true),
                    autoreduce_with(&gens, p, ord, false),
                    "autoreduce {what}"
                );
            }
        }
        // the corpus must leave the packed range for the test to say anything
        assert!(with_a_hole > 0, "no lead packed with a tail that did not");
        assert!(
            taken > 0 && declined > 0,
            "{taken} taken, {declined} declined"
        );
    }

    /// Inputs the polynomial type allows and a field does not: an empty
    /// polynomial and a modulus of 1.  The exponent-vector code takes both
    /// without a panic (a lone empty polynomial comes back from `interreduce`
    /// as it is, and modulo 1 every coefficient is zero), so the packed paths
    /// must read no term that is not there and build no Barrett step, which
    /// needs `p >= 2`.
    #[test]
    fn empty_polynomials_and_a_modulus_of_one_are_taken_as_before() {
        let empty: Poly = Vec::new();
        let x = poly(&[(&[1], 1)]);
        let (lone_empty, lone_x) = (std::slice::from_ref(&empty), std::slice::from_ref(&x));
        for ord in [Ordering::Lex, Ordering::Grlex, Ordering::Grevlex] {
            for p in [2u64, 3, 7, 65_521, 4_294_967_291, (1 << 61) - 1] {
                for packed in [false, true] {
                    assert_eq!(
                        interreduce_with(lone_empty, p, ord, packed),
                        vec![empty.clone()],
                        "interreduce of one empty polynomial, p = {p}, {ord:?}"
                    );
                }
                assert_eq!(interreduce(lone_empty, p, ord), vec![empty.clone()]);
                assert!(autoreduce(lone_empty, p, ord).is_empty());
                assert!(reduce(&empty, lone_x, p, ord).is_empty());
            }
            for packed in [false, true] {
                assert_eq!(
                    interreduce_with(lone_x, 1, ord, packed),
                    vec![x.clone()],
                    "interreduce modulo 1, {ord:?}"
                );
                let ps = if packed { PackedSet::new(lone_x) } else { None };
                assert!(reduce_skipping(lone_x, ps.as_ref(), 0, 1, ord).is_empty());
            }
            assert!(autoreduce(lone_x, 1, ord).is_empty());
            assert!(reduce(&x, lone_x, 1, ord).is_empty());
            assert!(normalise(&x, 1, ord).is_empty());
        }
    }

    /// [`normalise`] as it was before it sorted instead of building a map.
    fn normalise_with_a_map(terms: &[(Vec<u32>, u64)], p: u64, ord: Ordering) -> Poly {
        let mut map: BTreeMap<Vec<u32>, u64> = BTreeMap::new();
        for (e, c) in terms {
            let c = c % p;
            if c == 0 {
                continue;
            }
            let v = map.entry(e.clone()).or_insert(0);
            *v = (*v + c) % p;
        }
        let mut out: Poly = map.into_iter().filter(|(_, c)| *c != 0).collect();
        out.sort_by(|(a, _), (b, _)| cmp_monomial(b, a, ord));
        if let Some((_, lc)) = out.first() {
            let mut inv = 1u64;
            let mut base = lc % p;
            let mut e = p - 2;
            while e > 0 {
                if e & 1 == 1 {
                    inv = mulmod(inv, base, p);
                }
                base = mulmod(base, base, p);
                e >>= 1;
            }
            for (_, c) in out.iter_mut() {
                *c = mulmod(*c, inv, p);
            }
        }
        out
    }

    #[test]
    fn normalise_matches_the_map_based_original() {
        let mut rng = Xs(0x6e6f_726d);
        for p in [
            2u64,
            3,
            5,
            31,
            65_521,
            786_433,
            4_294_967_291,
            (1 << 61) - 1,
        ] {
            for ord in [Ordering::Lex, Ordering::Grlex, Ordering::Grevlex] {
                for _ in 0..60 {
                    let n = 1 + rng.below(4) as usize;
                    let k = rng.below(14) as usize;
                    // few distinct monomials, so that terms repeat and cancel;
                    // coefficients beyond p, and multiples of p
                    let terms: Vec<(Vec<u32>, u64)> = (0..k)
                        .map(|_| {
                            let e = (0..n).map(|_| rng.below(3) as u32).collect();
                            let c = match rng.below(4) {
                                0 => 0,
                                1 => p * rng.below(3),
                                _ => rng.below(p.saturating_mul(2).max(2)),
                            };
                            (e, c)
                        })
                        .collect();
                    assert_eq!(
                        normalise(&terms, p, ord),
                        normalise_with_a_map(&terms, p, ord),
                        "p = {p}, {ord:?}, {terms:?}"
                    );
                    assert_eq!(
                        normalise_owned(terms.clone(), p, ord),
                        normalise_with_a_map(&terms, p, ord),
                        "owned: p = {p}, {ord:?}, {terms:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn invmod_inverts() {
        let mut rng = Xs(0x1234_5678_9abc_def1);
        for p in [
            2u64,
            3,
            5,
            31,
            65_521,
            786_433,
            4_294_967_291,
            (1 << 61) - 1,
        ] {
            for a in (1..40u64).chain((0..200).map(|_| 1 + rng.next() % (u64::MAX - 1))) {
                if a % p == 0 {
                    continue;
                }
                assert_eq!(mulmod(a % p, invmod(a, p), p), 1, "{a}^-1 mod {p}");
            }
        }
    }
}
