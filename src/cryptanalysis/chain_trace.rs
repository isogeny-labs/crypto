//! E35 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.40): solver B's pivot
//! schedule, learned once per curve and replayed with a counted check.
//!
//! Additive.  Solver B (`chain-sparsest`) picks, in every column of the
//! chain solve's echelon, the sparsest of the rows whose lead is that
//! column; E33 (§9.37) counted that selection at `≈ 1.05·10⁶` native
//! operations a call.  Any candidate gives the same row space and the same
//! pivot columns, so the choice changes fill, never the result.  Here the
//! choice is learned from `K` targets of a stream of its own (B′'s
//! elimination, which makes B's choices: [`echelon_learn`]) and replayed
//! on every call ([`echelon_replay`]): in each column with more than one
//! candidate, one trace lookup, one test that the traced row sits at the
//! traced position of the column's sorted candidates, and one test that
//! its entry is nonzero; a miss takes A's choice (the lowest-indexed
//! candidate) at no further selection cost.
//!
//! Charged (native operations, one each): the shape comparison per call,
//! the lookup and the tests per contested column; learning's own
//! multiplications, Macaulay entries, B′ selection work and one operation
//! per column per learned sequence for the per-column mode.  The trace's
//! storage (`cols` pairs) is a memory-size counter, listed, not charged.
//!
//! Nothing here changes an existing solver: `macaulay`, `chain`,
//! `chain-sparsest` and E33's counted solvers take their unchanged paths.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Serialize;

use super::gaudry_quartic::{axpy, Echelon, PivotWork, QuarticSolve};
use super::klein_v4::{build_echelon, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND};
use super::quartic_folds::{K4PipelineReport, KleinInstance};
use super::residual_walk::inv_mod;

/// Learning targets per curve and arm (§9.40, fixed in advance).
pub const LEARN_TARGETS: usize = 3;
/// The learning stream: `StdRng::seed_from_u64(seed ^ LEARN_STREAM)`.
pub const LEARN_STREAM: u64 = 0xE357ACE;
const NONE: u32 = u32::MAX;
const FNV0: u64 = 0xCBF2_9CE4_8422_2325;

/// A learned pivot schedule for one matrix shape.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PivotTrace {
    pub rows: usize,
    pub cols: usize,
    /// Per column: the pivot row and its position among the column's
    /// candidates sorted by row index (`(NONE, NONE)`: no pivot).
    pub choice: Vec<(u32, u32)>,
}

impl PivotTrace {
    /// Storage, bytes (a memory-size counter: listed, not charged).
    pub fn bytes(&self) -> u64 {
        (self.choice.len() * 8) as u64
    }
    /// The pivot rows only (Stage 0 compares these with B's).
    pub fn rows_only(&self) -> Vec<u32> {
        self.choice.iter().map(|c| c.0).collect()
    }
}

/// The native work and diagnostics of one replayed echelon.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ReplayWork {
    /// Shape comparisons (one per echelon).
    pub shape_checks: u64,
    /// Trace lookups (one per contested column).
    pub lookups: u64,
    /// Validity tests (position, then the entry's zero status).
    pub tests: u64,
    /// Columns with more than one candidate.
    pub contested: u64,
    /// Contested columns that took A's choice (the trace missed).
    pub misses: u64,
    /// Diagnostic (not charged): misses whose traced row was still a
    /// candidate, at another position.
    pub misses_shifted: u64,
    /// Diagnostic (not charged): the contested columns where the trace
    /// was used, and those that missed.
    pub hit_cols: Vec<u32>,
    pub miss_cols: Vec<u32>,
    /// The shape differed: the whole echelon took A's choices.
    pub shape_fallback: bool,
    /// Entries read by the lead scans (every rule; not selection).
    pub lead_scan: u64,
    /// FNV-1a digest of `(column, pivot row)`, as E33's.
    pub digest: u64,
}

impl ReplayWork {
    /// The selection work charged as native operations.
    pub fn selection(&self) -> u64 {
        self.shape_checks + self.lookups + self.tests
    }
}

fn fnv(h: u64, x: u64) -> u64 {
    let mut h = h;
    for b in x.to_le_bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

thread_local! {
    static LEARNED: RefCell<(Vec<(u32, u32)>, Vec<bool>)> = const { RefCell::new((Vec::new(), Vec::new())) };
    static LAST_REPLAY: RefCell<ReplayWork> = RefCell::new(ReplayWork::default());
}

static CURRENT: RwLock<Option<&'static PivotTrace>> = RwLock::new(None);
static R_ECHELONS: AtomicU64 = AtomicU64::new(0);
static R_CONTESTED: AtomicU64 = AtomicU64::new(0);
static R_MISSES: AtomicU64 = AtomicU64::new(0);
static R_SHAPE: AtomicU64 = AtomicU64::new(0);

/// Install `t` as the trace `chain-sparsest-trace` replays (leaked: a few
/// kilobytes per curve and arm) and reset the replay diagnostics.
pub fn install(t: PivotTrace) -> &'static PivotTrace {
    let r: &'static PivotTrace = Box::leak(Box::new(t));
    *CURRENT.write().expect("trace lock") = Some(r);
    for a in [&R_ECHELONS, &R_CONTESTED, &R_MISSES, &R_SHAPE] {
        a.store(0, Ordering::Relaxed);
    }
    r
}

/// The installed trace.
pub fn current() -> Option<&'static PivotTrace> {
    *CURRENT.read().expect("trace lock")
}

/// The last replayed echelon's work on this thread (Stage A diagnostics).
pub fn last_replay() -> ReplayWork {
    LAST_REPLAY.with(|w| w.borrow().clone())
}

/// Clear [`last_replay`] (Stage A, before each call).
pub fn reset_last_replay() {
    LAST_REPLAY.with(|w| *w.borrow_mut() = ReplayWork::default());
}

pub(crate) fn note_replay(w: &ReplayWork) {
    R_ECHELONS.fetch_add(1, Ordering::Relaxed);
    R_CONTESTED.fetch_add(w.contested, Ordering::Relaxed);
    R_MISSES.fetch_add(w.misses, Ordering::Relaxed);
    R_SHAPE.fetch_add(u64::from(w.shape_fallback), Ordering::Relaxed);
    LAST_REPLAY.with(|l| *l.borrow_mut() = w.clone());
}

fn buckets_of(rows: &[Vec<u16>], lead_scan: &mut u64) -> (Vec<Vec<u32>>, usize, Vec<u32>) {
    let ncols = rows.first().map_or(0, |r| r.len());
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut zero_rows = 0usize;
    let mut zero_ids: Vec<u32> = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        match row.iter().position(|&x| x != 0) {
            Some(c) => {
                *lead_scan += c as u64 + 1;
                buckets[c].push(r as u32);
            }
            None => {
                *lead_scan += ncols as u64;
                zero_rows += 1;
                zero_ids.push(r as u32);
            }
        }
    }
    (buckets, zero_rows, zero_ids)
}

/// Normalise the pivot row `piv` at column `c`; the columns it touches.
fn normalise(piv: &mut [u16], c: usize, p: u32, muls: &mut u64) -> Vec<u32> {
    let ncols = piv.len();
    let inv = inv_mod(piv[c] as u64, p as u64) as u32;
    let mut nz: Vec<u32> = Vec::new();
    for j in c..ncols {
        if piv[j] != 0 {
            piv[j] = ((piv[j] as u32 * inv) % p) as u16;
            nz.push(j as u32);
        }
    }
    *muls += nz.len() as u64;
    nz
}

/// One row update by the normalised pivot (E31/E33's arithmetic exactly).
fn update(row: &mut [u16], piv: &[u16], nz: &[u32], c: usize, p: u32, dense: bool) {
    let f = row[c] as u32;
    if dense {
        axpy(&mut row[c..], &piv[c..], f, p);
    } else {
        let nf = p - f;
        for &j in nz {
            let j = j as usize;
            row[j] = ((row[j] as u32 + nf * piv[j] as u32) % p) as u16;
        }
    }
}

/// **The replay** (solver T): E31 A's elimination, with each contested
/// column's pivot taken from `trace` when the counted check passes.
///
/// `unchecked` is a diagnostic for the positive control of §9.40 only: it
/// takes the traced row without the checks (and aborts on a zero pivot).
pub(crate) fn echelon_replay(
    mut rows: Vec<Vec<u16>>,
    p: u32,
    muls: &mut u64,
    trace: &PivotTrace,
    unchecked: bool,
    work: &mut ReplayWork,
) -> Echelon {
    let ncols = rows.first().map_or(0, |r| r.len());
    work.shape_checks += 1;
    let shape_ok = rows.len() == trace.rows && ncols == trace.cols;
    work.shape_fallback = !shape_ok;
    let (mut buckets, mut zero_rows, mut zero_ids) = buckets_of(&rows, &mut work.lead_scan);
    let mut digest = FNV0;
    let mut pivot_row = vec![NONE; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        b.sort_unstable();
        if b.len() > 1 {
            work.contested += 1;
            if shape_ok {
                work.lookups += 1;
                let (t, pos) = trace.choice[c];
                if unchecked {
                    if (t as usize) < rows.len() {
                        match b.iter().position(|&r| r == t) {
                            Some(i) => b.swap(0, i),
                            None => b.insert(0, t),
                        }
                    }
                } else {
                    work.tests += 1;
                    let at_pos = t != NONE && b.get(pos as usize) == Some(&t);
                    let ok = at_pos && {
                        work.tests += 1;
                        rows[t as usize][c] != 0
                    };
                    if ok {
                        b.swap(0, pos as usize);
                        work.hit_cols.push(c as u32);
                    } else {
                        work.misses += 1;
                        work.miss_cols.push(c as u32);
                        if t != NONE && b.contains(&t) {
                            work.misses_shifted += 1;
                        }
                    }
                }
            }
        }
        let pr = b[0] as usize;
        digest = fnv(fnv(digest, c as u64), pr as u64);
        let mut piv = std::mem::take(&mut rows[pr]);
        assert!(
            piv.get(c).is_some_and(|&x| x != 0),
            "replay: zero pivot at column {c} (unchecked trace)"
        );
        let nz = normalise(&mut piv, c, p, muls);
        let dense = nz.len() * 8 > ncols - c;
        let mut others: Vec<(u32, Vec<u16>, Option<usize>, u64)> = b[1..]
            .iter()
            .map(|&r| (r, std::mem::take(&mut rows[r as usize]), None, 0))
            .collect();
        *muls += (others.len() * nz.len()) as u64;
        let nzr = &nz;
        let pv = &piv;
        others.par_iter_mut().for_each(|(_, row, lead, scan)| {
            update(row, pv, nzr, c, p, dense);
            *lead = row[c + 1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
            *scan = match *lead {
                Some(l) => (l - c) as u64,
                None => (ncols - c - 1) as u64,
            };
        });
        for (r, row, lead, scan) in others {
            work.lead_scan += scan;
            rows[r as usize] = row;
            match lead {
                Some(l) => buckets[l].push(r),
                None => {
                    zero_rows += 1;
                    zero_ids.push(r);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    work.digest = digest;
    zero_ids.sort_unstable();
    Echelon {
        rows,
        pivot_row,
        zero_rows,
        zero_ids,
    }
}

/// **The learner**: E33's B′ (`SparsestIncremental`) elimination with its
/// counts and charges exactly, recording each column's choice with its
/// position among the sorted candidates (taken by [`take_learned`]).
pub(crate) fn echelon_learn(
    mut rows: Vec<Vec<u16>>,
    p: u32,
    muls: &mut u64,
    init_nnz: Vec<u32>,
    work: &mut PivotWork,
) -> Echelon {
    let ncols = rows.first().map_or(0, |r| r.len());
    assert_eq!(init_nnz.len(), rows.len());
    let mut row_nnz = init_nnz;
    let (mut buckets, mut zero_rows, mut zero_ids) = buckets_of(&rows, &mut work.lead_scan);
    let mut digest = FNV0;
    let mut pivot_row = vec![NONE; ncols];
    let mut choice = vec![(NONE, NONE); ncols];
    let mut contested = vec![false; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        b.sort_unstable();
        let mut best = 0usize;
        if b.len() > 1 {
            contested[c] = true;
            work.contested += 1;
            work.compare += b.len() as u64 - 1;
            best = (0..b.len())
                .min_by_key(|&i| (row_nnz[b[i] as usize], i))
                .unwrap_or(0);
            b.swap(0, best);
        }
        let pr = b[0] as usize;
        choice[c] = (pr as u32, best as u32);
        digest = fnv(fnv(digest, c as u64), pr as u64);
        let mut piv = std::mem::take(&mut rows[pr]);
        let nz = normalise(&mut piv, c, p, muls);
        let dense = nz.len() * 8 > ncols - c;
        type Upd = (u32, Vec<u16>, Option<usize>, u32, u64, u64);
        let mut others: Vec<Upd> = b[1..]
            .iter()
            .map(|&r| (r, std::mem::take(&mut rows[r as usize]), None, 0, 0, 0))
            .collect();
        *muls += (others.len() * nz.len()) as u64;
        let nnz_ref = &row_nnz;
        let nzr = &nz;
        let pv = &piv;
        others
            .par_iter_mut()
            .for_each(|(r, row, lead, nnz, scan, upd)| {
                let old_nz = nzr.iter().filter(|&&j| row[j as usize] != 0).count() as u32;
                update(row, pv, nzr, c, p, dense);
                *lead = row[c + 1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
                *scan = match *lead {
                    Some(l) => (l - c) as u64,
                    None => (ncols - c - 1) as u64,
                };
                if lead.is_some() {
                    let new_nz = nzr.iter().filter(|&&j| row[j as usize] != 0).count() as u32;
                    *nnz = nnz_ref[*r as usize] - old_nz + new_nz;
                    *upd = 2 * nzr.len() as u64 + 1;
                }
            });
        for (r, row, lead, nnz, scan, upd) in others {
            work.lead_scan += scan;
            work.update += upd;
            if lead.is_some() {
                row_nnz[r as usize] = nnz;
            }
            rows[r as usize] = row;
            match lead {
                Some(l) => buckets[l].push(r),
                None => {
                    zero_rows += 1;
                    zero_ids.push(r);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    work.digest = digest;
    LEARNED.with(|l| *l.borrow_mut() = (choice, contested));
    zero_ids.sort_unstable();
    Echelon {
        rows,
        pivot_row,
        zero_rows,
        zero_ids,
    }
}

/// The choices recorded by the last [`echelon_learn`] on this thread.
pub(crate) fn take_learned() -> (Vec<(u32, u32)>, Vec<bool>) {
    LEARNED.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// What learning cost, and (end to end) what the replays did.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct TraceLearning {
    /// Learning targets used (`K`) and drawn (some may have `z = ∞`).
    pub targets: u64,
    pub drawn: u64,
    pub generation_muls: u64,
    pub weil_muls: u64,
    pub triangularise_muls: u64,
    pub echelon_muls: u64,
    /// Every learning multiplication (added to the record's `total_muls`).
    pub muls: u64,
    /// Learning's Macaulay entries (added to `macaulay_entries`).
    pub entries: u64,
    /// B′ selection `[initial, comparisons, updates]` of the learning
    /// echelons; the mode's operations are in `[0]` as well.
    pub selection: [u64; 3],
    pub mode_ops: u64,
    /// Everything native: `selection` summed (added to
    /// `pivot_selection_ops`; `entries` is separate).
    pub selection_ops: u64,
    pub lead_scan_reads: u64,
    pub rows: usize,
    pub cols: usize,
    /// Memory-size counter (listed, not charged).
    pub trace_bytes_uncharged: u64,
    /// Learned sequences equal to the mode, of `targets`.
    pub sequences_equal_to_mode: u64,
    /// End to end only: replayed echelons, contested columns, misses and
    /// shape fallbacks, including the tail of the last batch that the
    /// pipeline computes but does not count (diagnostics, not charged).
    pub replay_echelons: u64,
    pub replay_contested: u64,
    pub replay_misses: u64,
    pub replay_shape_fallbacks: u64,
}

/// One learning echelon: B′'s pivot sequence with positions, and its cost.
pub struct Learned {
    pub rows: usize,
    pub cols: usize,
    pub choice: Vec<(u32, u32)>,
    /// Columns with more than one candidate on this system.
    pub contested: Vec<bool>,
    pub st: QuarticSolve,
}

/// B′'s echelon on one system, as the learner runs it (also Stage 0's
/// instrument: B's pivot sequence for a call).  `None` if the system is
/// refused before the echelon (charged by the caller from `st`).
pub fn learn_one(comps: &[HashMap<[u8; 4], u64>], p: u64) -> Result<Learned, QuarticSolve> {
    let mut st = QuarticSolve::default();
    match build_echelon(comps, MACAULAY_BOUND, p, &mut st, true, Pivot::Learn) {
        Ok(_) => {
            let (choice, contested) = take_learned();
            Ok(Learned {
                rows: st.rows,
                cols: st.cols,
                choice,
                contested,
                st,
            })
        }
        Err(_) => Err(st),
    }
}

fn absorb(cost: &mut TraceLearning, l: &Learned) {
    cost.triangularise_muls += l.st.triangularise_muls;
    cost.echelon_muls += l.st.echelon_muls;
    cost.entries += (l.rows * l.cols) as u64;
    cost.selection[0] += l.st.pivot_init_ops;
    cost.selection[1] += l.st.pivot_compare_ops;
    cost.selection[2] += l.st.pivot_update_ops;
    cost.lead_scan_reads += l.st.lead_scan_reads;
}

/// The per-column mode of learned sequences over the first one's shape
/// (ties to the earliest sequence), with its operations charged.
pub fn mode_trace(seqs: &[Learned], cost: &mut TraceLearning) -> Option<PivotTrace> {
    let first = seqs.first()?;
    let (rows, cols) = (first.rows, first.cols);
    let same: Vec<&Learned> = seqs
        .iter()
        .filter(|s| s.rows == rows && s.cols == cols)
        .collect();
    let mut choice = vec![(NONE, NONE); cols];
    for (c, slot) in choice.iter_mut().enumerate() {
        cost.mode_ops += same.len() as u64;
        let mut best = (0usize, (NONE, NONE));
        for s in &same {
            let v = s.choice[c];
            let n = same.iter().filter(|t| t.choice[c] == v).count();
            if n > best.0 {
                best = (n, v);
            }
        }
        *slot = best.1;
    }
    cost.selection[0] += cost.mode_ops;
    cost.targets = seqs.len() as u64;
    cost.sequences_equal_to_mode = same.iter().filter(|s| s.choice == choice).count() as u64;
    cost.rows = rows;
    cost.cols = cols;
    let t = PivotTrace { rows, cols, choice };
    cost.trace_bytes_uncharged = t.bytes();
    cost.selection_ops = cost.selection.iter().sum();
    cost.muls = cost.generation_muls + cost.weil_muls + cost.triangularise_muls + cost.echelon_muls;
    Some(t)
}

/// Learn a trace from given systems (the first `k` that reach the
/// echelon; refused ones are charged and skipped).
pub fn learn_from_systems(
    systems: &[Vec<HashMap<[u8; 4], u64>>],
    p: u64,
    k: usize,
    cost: &mut TraceLearning,
) -> Option<PivotTrace> {
    let mut seqs: Vec<Learned> = Vec::new();
    for comps in systems {
        if seqs.len() == k {
            break;
        }
        match learn_one(comps, p) {
            Ok(l) => {
                absorb(cost, &l);
                seqs.push(l);
            }
            Err(st) => cost.triangularise_muls += st.triangularise_muls,
        }
    }
    mode_trace(&seqs, cost)
}

/// The next learning system of one GLS curve: `R = aG + bQ` from the
/// learning stream, Weil-restricted exactly as a target (generation and
/// Weil charged into `cost`); `None` when `z(R) = ∞`.
fn next_learning_system(
    ki: &KleinInstance,
    pre: &SymmetrisedV4,
    lrng: &mut StdRng,
    cost: &mut TraceLearning,
) -> Option<Vec<HashMap<[u8; 4], u64>>> {
    let inst = ki.inst;
    cost.drawn += 1;
    let a = lrng.gen_range(0..inst.n);
    let b = lrng.gen_range(1..inst.n);
    let c = inst.curve();
    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
    cost.generation_muls += c.f.muls();
    let c = inst.curve();
    let before = c.f.muls();
    let Some(z) = ki.z_of_x(&c.f, &r.x) else {
        cost.weil_muls += c.f.muls() - before;
        return None;
    };
    let mut fp = 0u64;
    let comps = v4_system(pre, &c.f, &z, &mut fp);
    cost.weil_muls += c.f.muls() - before + fp;
    Some(comps)
}

/// Learn a GLS curve's trace from `K = LEARN_TARGETS` learning systems
/// (drawn from `StdRng::seed_from_u64(seed ^ LEARN_STREAM)`, further ones
/// drawn and charged when one is refused).
pub fn learn_gls(
    ki: &KleinInstance,
    pre: &SymmetrisedV4,
    seed: u64,
) -> Result<(PivotTrace, TraceLearning), String> {
    let mut cost = TraceLearning::default();
    let mut lrng = StdRng::seed_from_u64(seed ^ LEARN_STREAM);
    let mut seqs: Vec<Learned> = Vec::new();
    while seqs.len() < LEARN_TARGETS {
        if cost.drawn >= 64 * LEARN_TARGETS as u64 {
            return Err("trace learning: too few learning systems reach the echelon".into());
        }
        let Some(comps) = next_learning_system(ki, pre, &mut lrng, &mut cost) else {
            continue;
        };
        match learn_one(&comps, ki.inst.p) {
            Ok(l) => {
                absorb(&mut cost, &l);
                seqs.push(l);
            }
            Err(st) => cost.triangularise_muls += st.triangularise_muls,
        }
    }
    let t = mode_trace(&seqs, &mut cost).ok_or("trace learning: no sequence")?;
    Ok((t, cost))
}

/// Pipeline hook: learn and install the trace of this curve and arm.
pub fn learn_and_install(
    ki: &KleinInstance,
    pre: &SymmetrisedV4,
    seed: u64,
) -> Result<TraceLearning, String> {
    let (t, cost) = learn_gls(ki, pre, seed)?;
    install(t);
    Ok(cost)
}

/// Pipeline hook: charge learning into the record and attach the replay
/// diagnostics.
pub fn charge(mut rep: K4PipelineReport, mut l: TraceLearning) -> K4PipelineReport {
    l.replay_echelons = R_ECHELONS.load(Ordering::Relaxed);
    l.replay_contested = R_CONTESTED.load(Ordering::Relaxed);
    l.replay_misses = R_MISSES.load(Ordering::Relaxed);
    l.replay_shape_fallbacks = R_SHAPE.load(Ordering::Relaxed);
    rep.total_muls += l.muls;
    rep.macaulay_entries += l.entries;
    if let Some(s) = rep.pivot_selection.as_mut() {
        for (si, li) in s.iter_mut().zip(l.selection) {
            *si += li;
        }
    }
    rep.pivot_selection_ops = rep.pivot_selection.map(|s| s.iter().sum());
    rep.lead_scan_reads = rep.lead_scan_reads.map(|v| v + l.lead_scan_reads);
    rep.ratio = rep.total_muls as f64 / rep.rho_matched_muls;
    rep.trace_learning = Some(l);
    rep
}
