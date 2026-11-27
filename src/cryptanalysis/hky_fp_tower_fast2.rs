//! # Second speed pass on the HKY closure instrument (`--engine hky3`).
//!
//! [`hky_fp_tower_fast`](super::hky_fp_tower_fast) (`hky2`, "v1") is left as
//! it is; this module ("v2") computes the same space `V_{F,c}` with the same
//! columns, the same products and the same symmetry split (it uses v1's
//! `Space`), and changes how the queue is served and how a row is reduced.
//! `research/hky_engine_speed2_20261117/README.md` has the measurement of
//! each change. What is different:
//!
//! 1. **Reduction with a bitmap of touched columns** while a reduction is
//!    sparse. v1 scans the accumulator from the first to the last touched
//!    column, which is `O(columns)` per product even when the product has a
//!    hundred terms; on closures of low rank (many summands) that scan, not
//!    the arithmetic, is the cost. v2 follows a bitmap until the reduction
//!    has done a quarter of the span in multiply-adds, then scans as v1.
//! 2. **Scratch memory per thread, allocated once per closure** (v1
//!    allocates and zeroes `8 n` bytes per rayon job of every batch).
//! 3. **Periodic interreduction** ([`Fast2Options::interreduce`]): the
//!    stored rows are brought back to the reduced echelon form of the
//!    current pivot set (back-substitution, in blocks, in parallel), so that
//!    reducer rows carry no pivot column and a product is reduced in one
//!    sweep with short rows.
//! 4. **Batch size from the pivot yield** ([`Fast2Options::target_pivots`]):
//!    a batch holds as many products as are expected to give that many new
//!    pivots, between `batch_min` and `batch_max`. It does not depend on the
//!    number of threads, so the schedule (hence `muladds`, `rows_reduced`
//!    and every stored row) is the same for every thread count.
//! 5. **A parallel second pass** ([`Fast2Options::par_second`]): the rows of
//!    a batch that survive the reduction against the echelon are inserted
//!    by rounds (distinct leading columns at once, the rest reduced again in
//!    parallel) instead of one by one on one thread.
//!
//! ## Why interreduction keeps the answer
//!
//! Replacing a pivot row `g` by `g' = g − Σ aᵢ rᵢ` (`rᵢ` pivot rows with a
//! later leading column) changes neither the span nor the pivot set. The
//! products still owed for `g` are then formed from `g'`:
//! `x·g = x·g' + Σ aᵢ x·rᵢ`. The columns are sorted by degree, highest
//! first, so `deg rᵢ ≤ deg g < c` and every `x·rᵢ` is owed too; by induction
//! on the leading column (from the last column backwards) the product of
//! any version of any row of degree below `c` lies in the final span, and
//! the final span is closed. With the split the same holds part by part
//! (the projections are linear).
//!
//! ## Determinism
//!
//! For given options the engine does the same reductions in the same order
//! whatever the thread count: batches are cut by rules that do not look at
//! the pool, parallel sections return their results in batch order, and
//! insertion follows that order.

use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;

use rayon::prelude::*;

use crate::cryptanalysis::fp_gemm;
use crate::cryptanalysis::hky_fp_tower::{
    count_monomials, Closure, FallDegrees, HkyLimits, HkyRing, NfPoly,
};
use crate::cryptanalysis::hky_fp_tower_fast::{
    axpy_dense, barrett, check_symmetry, detect_simd, normalise, sparse_bucket, Row, RowS, Simd,
    Space, NONE,
};

/// What [`closure_fast2`] does. The defaults are the recommended instrument.
#[derive(Clone, Debug)]
pub struct Fast2Options {
    /// A candidate involution of the tower variables, verified before use
    /// (as in v1).
    pub symmetry: Option<Vec<usize>>,
    /// Smallest and largest batch (products taken from the queue at once).
    pub batch_min: usize,
    pub batch_max: usize,
    /// New pivots aimed at per batch: the batch is
    /// `target_pivots / yield` clamped to `[batch_min, batch_max]`, `yield`
    /// being the running share of products that gave a pivot.
    pub target_pivots: usize,
    /// Interreduce when the pivots added since the last interreduction
    /// exceed this fraction of the rank at that time (and `interreduce_min`).
    /// 0 turns it off.
    pub interreduce: f64,
    pub interreduce_min: usize,
    /// Rows per parallel block of the back-substitution.
    pub interreduce_block: usize,
    /// After an interreduction, put the queued products back in the bucket
    /// of their source row's current number of terms.
    pub rebucket: bool,
    /// The second pass over a batch by parallel rounds (otherwise one row
    /// at a time on the calling thread, as v1).
    pub par_second: bool,
    /// Follow a bitmap of touched columns while a reduction is sparse.
    pub bitmap: bool,
    pub dense_num: usize,
    pub dense_den: usize,
    /// Test hook of the equality gate's negative controls (see v1); 0 in
    /// every measurement. 1: products by variable 0 are not formed.
    /// 2: the delayed reduction subtracts `p² − 1`. 3: the interreduction
    /// drops the last term of every row it rewrites. 4: a new epoch drops
    /// the last term of every row with two or more tail terms. 5: the sweep
    /// skips its first tile of every part. 6, 7, 8 (with
    /// [`Fast2Options::skip`]; skips PROOF.md does not cover): 6 also skips
    /// a variable whose order equals the cover's, 7 takes covers from leads
    /// of any degree, 8 skips whenever the leading column is covered at all.
    pub fault: u8,
    /// After a closure without 1, check that it is closed
    /// ([`verify_closed`]): every generator and every variable times every
    /// final basis row of degree below `c` reduces to zero. Costs about one
    /// more closure and a second copy of the echelon.
    pub verify_closed: bool,
    /// Queue priority: products of a source row with `nnz` terms and leading
    /// degree `d` wait in bucket `1 + ⌊log₂ nnz⌋ + deg_weight · d`; the lowest
    /// bucket is served first, last in first out. 0 is v1's sparsest-first;
    /// 64 or more is lowest degree first, sparsest first among equals.
    pub deg_weight: usize,
    /// Store row tails in the coordinates of the columns that were not
    /// pivots at the last interreduction (an "epoch") and reduce in an
    /// accumulator of that size ([`Echelon`]). Off: one epoch, every
    /// column, which is v2 as it was. Same reductions either way.
    pub np_epochs: bool,
    /// Phase 0 of a batch at once, by tiles of epoch columns, when its
    /// work is at least `sweep_ratio` multiply-adds per accumulator entry
    /// and the accumulators take at most `sweep_max_words` words
    /// ([`reduce_products`]). Same reductions either way.
    pub sweep: bool,
    pub sweep_ratio: u64,
    pub sweep_tile_bytes: usize,
    pub sweep_max_words: usize,
    /// Skip products proved in advance to lie in the final span
    /// (`research/hky_engine_skip_20261118/PROOF.md`, "lead cover"). Fix an
    /// order κ on the variables whose products are formed. When a product
    /// `x_a · s` (a projection of it, with the symmetry) of a row `s` of
    /// degree `d − 1 ≤ c − 2` is formed and its leading column `ℓ` has
    /// degree `d`, the column `ℓ` is *covered* with `κ(a)`; a queued
    /// product `x_b · r` of a row `r` whose leading column is covered with
    /// some `κ(a) < κ(b)` is not formed. The space is the same; counters
    /// differ. Faults 6 and 7 are negative controls that skip products the
    /// proof does not cover.
    pub skip: bool,
    /// The order κ: 0 ascending variable index, 1 descending.
    pub skip_order: u8,
    /// Witnesses at insertion (PROOF.md §2.1–2.3): when a row of degree
    /// at most `c − 2` is inserted, the top-degree parts of its products by
    /// every variable cover (1) their own leading columns, or (2) the
    /// leading columns of the labelled spans they generate (`Witness`;
    /// more covers, more work). 0: only the covers of formed products.
    pub skip_witness: u8,
    /// Witness vectors are inserted only for variables with κ below this
    /// (a cover κ₀ only lets the variables above κ₀ skip, so large labels
    /// buy little); 0 means no limit. Fewer inserts only cover less.
    pub skip_witness_max: u16,
    /// Append one JSON line per batch and per interreduction to this file.
    pub trace: Option<String>,
    /// Checkpoint/resume and basis export (`hky_closure_io`); off by
    /// default. Never changes the schedule or the result.
    pub io: crate::cryptanalysis::hky_closure_io::IoOptions,
}

impl Default for Fast2Options {
    fn default() -> Self {
        Fast2Options {
            symmetry: None,
            batch_min: 32,
            batch_max: 256,
            target_pivots: 32,
            // 0.65 with lead-cover skip: E2LOCMS45 P2 ≈153s (was ~256s at 0.6),
            // P1 still ≈18–19s; unit tests cover the interreduce equivalence.
            interreduce: 0.65,
            interreduce_min: 256,
            interreduce_block: 256,
            rebucket: true,
            par_second: true,
            bitmap: true,
            dense_num: 1,
            dense_den: 3,
            fault: 0,
            verify_closed: false,
            deg_weight: 1,
            np_epochs: true,
            sweep: true,
            sweep_ratio: 1,
            sweep_tile_bytes: 1 << 20,
            sweep_max_words: 1 << 28,
            // Lead-cover skip is exact (PROOF.md); on E2LOCMS45 P2 it cut wall
            // ~256s → ~153s at ir=0.6/0.65 with unchanged d_F.
            skip: true,
            skip_order: 1,
            skip_witness: 1,
            skip_witness_max: 0,
            trace: None,
            io: Default::default(),
        }
    }
}

/// Timers and counts of one closure.
#[derive(Clone, Debug, Default)]
pub struct Fast2Stats {
    pub symmetry: String,
    pub part_cols: Vec<u64>,
    pub part_rank: Vec<u64>,
    pub setup_s: f64,
    /// Parallel sections: products and reduction against the echelon.
    pub parallel_s: f64,
    /// The second pass (its parallel rounds included) and insertion.
    pub second_s: f64,
    /// Of `second_s`, the part spent on the calling thread alone.
    pub serial_s: f64,
    pub interreduce_s: f64,
    pub batches: u64,
    pub rounds: u64,
    pub interreductions: u64,
    /// Multiply-adds: all, and those of the interreductions and of the
    /// second pass (both included in the total).
    pub muladds: u64,
    pub interreduce_muladds: u64,
    pub second_muladds: u64,
    /// Products reduced, how many reduced to zero, reducer rows used.
    pub products: u64,
    pub zero: u64,
    pub reducers: u64,
    /// Multiply-adds against the rows of the epoch's pivots (phase 0 of the
    /// reduction), and those spent on products that reduced to zero.
    pub core_muladds: u64,
    pub zero_muladds: u64,
    /// Batches whose phase 0 ran as a sweep.
    pub swept_batches: u64,
    /// [`Fast2Options::verify_closed`]: reductions checked, how many were
    /// not zero (0 = closed), seconds.
    pub verify_products: u64,
    pub verify_nonzero: u64,
    pub verify_s: f64,
    /// Bytes of the stored rows at the end (8 per sparse term, 4 per dense
    /// word), and of the sweep's accumulators.
    pub final_row_bytes: u64,
    pub sweep_bytes: u64,
    /// [`Fast2Options::skip`]: product items (row × variable) queued, and
    /// of those, served without being formed; columns covered at the end.
    pub skip_on: bool,
    pub queued_items: u64,
    pub skipped_items: u64,
    pub covered_cols: u64,
    /// Analysis (not checkpointed): items served that the criterion covers
    /// (formed only when the skip is off), and their multiply-adds; and
    /// multiply-adds and products by source class `2 · degree + covered`
    /// (31: generator rows).
    pub coverable_items: u64,
    /// Witness spaces: multiply-adds, seconds, rows at the end.
    pub witness_muladds: u64,
    pub witness_s: f64,
    /// Unused (0); and the rounds of `Witness::insert_batch`.
    pub witness_seq_s: f64,
    /// Covered columns by degree and κ: entry `d · (m + 1) + κ` (m the
    /// number of κ values; entry `d · (m + 1) + m` unused), at the end (or
    /// at a budget stop). And the rank (pivots) by degree.
    pub cover_hist: Vec<u64>,
    pub rank_by_deg: Vec<u64>,
    pub witness_survivors: u64,
    pub witness_rows: u64,
    pub coverable_ops: u64,
    pub ops_by_src: Vec<u64>,
    pub prods_by_src: Vec<u64>,
    pub dense_rows: u64,
    pub sparse_rows: u64,
    pub stored_words: u64,
    /// The largest `stored_words` seen (before an interreduction).
    pub peak_words: u64,
    pub threads: usize,
    pub simd: &'static str,
    /// What `hky_closure_io` did: checkpoints written, the resume source,
    /// the exported basis.
    pub io: crate::cryptanalysis::hky_closure_io::IoReport,
}

/// The echelon of one part.
///
/// **Stored rows are in epoch coordinates** (`research/hky_engine_opt_20261118/`):
/// a row's leading column is a column of the part, but its tail is written
/// in the coordinates of `np_cols`, the columns that were not pivots at the
/// last interreduction (the "epoch"; before the first interreduction every
/// column). Right after an interreduction every stored row has no pivot
/// column in its tail, so a tail lives in the non-pivot columns; a row
/// inserted later is reduced against every pivot, so its tail does too.
/// Hence every tail can be written in `np_cols` coordinates, and reducing a
/// product works in an accumulator of `np_cols.len()` entries (the columns
/// not yet solved, `n − rank` late in a closure) instead of `n`.
/// [`Echelon::row_full`] gives a row back in the part's columns.
///
/// - `RowS::Sparse(v)`: `v[0] = (lead, 1)` with `lead` a column of the part;
///   `v[1..]` are `(j, x)` with `j` an index into `np_cols`, ascending.
/// - `RowS::Dense { lead, v, nnz }`: `v[0] = 1`; `v[1 + i]` is the
///   coefficient of `np_cols[s + i]`, `s = np_start(lead)` (the first epoch
///   column after `lead`), up to the last epoch column.
///
/// Checkpoint state: `pivot`, `rows` and `np_cols` (`of_col` is derived:
/// [`Echelon::with_epoch`]).
pub(super) struct Echelon {
    pub(super) pivot: Vec<u32>,
    pub(super) rows: Vec<RowS>,
    /// The non-pivot columns of the epoch, ascending.
    pub(super) np_cols: Vec<u32>,
    /// Column of the part → its index in `np_cols`, or `NONE` (a pivot
    /// column of the epoch). Derived from `np_cols`.
    pub(super) of_col: Vec<u32>,
}

/// Every piece of state the main loop carries from one batch to the next.
/// A checkpoint is exactly this struct (`hky_closure_io` serialises it);
/// state added to the loop must be added here, or resume is not exact.
/// Each `Echelon` is saved as `pivot` (derived from the rows), `rows` and
/// `np_cols`, and rebuilt with [`Echelon::with_epoch`].
pub(super) struct ClosureState {
    pub(super) ech: Vec<Echelon>,
    pub(super) queue: Vec<Vec<Item>>,
    pub(super) rows_reduced: u64,
    pub(super) contains_one: bool,
    pub(super) yield_q16: u64,
    pub(super) fresh_since_ir: usize,
    pub(super) rank_at_ir: usize,
    /// The counters of `Fast2Stats` (timers included, so they accumulate
    /// across restarts).
    pub(super) stats: Fast2Stats,
    /// Wall seconds of the closure before this process (restarts).
    pub(super) elapsed_before_s: f64,
    /// [`Fast2Options::skip`]: per part and column, the smallest κ that
    /// covers it ([`NO_COVER`]: none). Empty when the skip is off.
    pub(super) cover: Vec<Vec<u16>>,
}

/// A column no formed product covers ([`Fast2Options::skip`]).
pub(super) const NO_COVER: u16 = u16::MAX;

/// PROOF.md, the witness spaces of one part: a semi-echelon basis (distinct
/// leading columns, monic) of homogeneous vectors, each with a label κ,
/// such that for every κ the rows with label ≤ κ are a basis of `E_κ`, the
/// span of the vectors inserted with label ≤ κ. The leading columns of
/// `E_κ` are then exactly those of its rows, so the label of the row at a
/// column is the least κ whose space has that leading column.
pub(super) struct Witness {
    pub(super) pivot: Vec<u32>,
    pub(super) rows: Vec<Row>,
    pub(super) label: Vec<u16>,
}

impl Witness {
    pub(super) fn new(n: usize) -> Self {
        Witness {
            pivot: vec![NONE; n],
            rows: Vec::new(),
            label: Vec::new(),
        }
    }

    /// `v` minus rows of label ≤ `k`, reduced at its lead while the row
    /// there has label ≤ `k` (read-only: many in parallel before
    /// the placement in [`Witness::insert_batch`], which does the rest). The
    /// result stays in E_k.
    /// `acc` is a zeroed scratch of at least the part's columns (left zeroed).
    pub(super) fn prereduce(&self, v: &Row, k: u16, p: u64, acc: &mut [u64]) -> (Row, u64) {
        let mut ops = 0u64;
        let Some(&(first, _)) = v.first() else {
            return (Vec::new(), 0);
        };
        let p2 = p * p;
        let mut hi = 0usize;
        for &(j, x) in v {
            acc[j as usize] = u64::from(x);
            hi = hi.max(j as usize);
        }
        let mut j = first as usize;
        loop {
            while j <= hi && acc[j].is_multiple_of(p) {
                acc[j] = 0;
                j += 1;
            }
            if j > hi {
                return (Vec::new(), ops);
            }
            let x = acc[j] % p;
            let ri = self.pivot[j];
            if ri == NONE || self.label[ri as usize] > k {
                let mut r: Row = Vec::new();
                for (jj, a) in acc.iter_mut().enumerate().take(hi + 1).skip(j) {
                    let y = std::mem::take(a) % p;
                    if y != 0 {
                        r.push((jj as u32, y as u32));
                    }
                }
                return (r, ops);
            }
            let nf = p - x;
            let row = &self.rows[ri as usize];
            for &(c, y) in row {
                let a = &mut acc[c as usize];
                *a += nf * u64::from(y);
                if *a >= p2 {
                    *a -= p2;
                }
                hi = hi.max(c as usize);
            }
            ops += row.len() as u64;
            acc[j] = 0;
        }
    }

    /// Inserts a batch of `(label, vector)` by parallel rounds (as the
    /// engine's second pass): every candidate is reduced in parallel against
    /// the rows of label ≤ its own; for each leading column the survivor
    /// of smallest label (first among equals) is placed — at a free column,
    /// or in place of a row of larger label, which then goes on, minus the
    /// new row, with its own label; the others go to the next round.
    /// Every placement keeps the witness invariant (W) of PROOF.md §2.3: a
    /// row placed at column ℓ with label k is a combination of inserted
    /// vectors of label ≤ k with lead ℓ.
    fn insert_batch(
        &mut self,
        mut cands: Vec<(u16, Row)>,
        p: u64,
        pool: &Pool,
        set: &mut impl FnMut(u32, u16),
    ) -> (u64, u64) {
        let (mut ops, mut rounds) = (0u64, 0u64);
        while !cands.is_empty() {
            rounds += 1;
            let me = &*self;
            let pre: Vec<(u16, Row, u64)> = cands
                .par_iter()
                .with_max_len(4)
                .map(|(k, v)| {
                    let (r, o) = pool.with(|sc| me.prereduce(v, *k, p, &mut sc.acc));
                    (*k, r, o)
                })
                .collect();
            cands = Vec::new();
            // winner per leading column: smallest label, first among equals
            let mut best: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
            let mut surv: Vec<(u16, Row)> = Vec::new();
            for (k, r, o) in pre {
                ops += o;
                if r.is_empty() {
                    continue;
                }
                let i = surv.len();
                let lead = r[0].0;
                surv.push((k, r));
                match best.get(&lead) {
                    Some(&b) if surv[b].0 <= k => {}
                    _ => {
                        best.insert(lead, i);
                    }
                }
            }
            let winners: std::collections::HashSet<usize> = best.values().copied().collect();
            for (i, (k, mut r)) in surv.into_iter().enumerate() {
                if !winners.contains(&i) {
                    cands.push((k, r));
                    continue;
                }
                let lead = r[0].0 as usize;
                normalise(&mut r, p);
                let ri = self.pivot[lead];
                if ri == NONE {
                    self.pivot[lead] = self.rows.len() as u32;
                    self.rows.push(r);
                    self.label.push(k);
                    set(lead as u32, k);
                } else {
                    // held by a larger label (prereduce stopped there)
                    let ri = ri as usize;
                    debug_assert!(self.label[ri] > k);
                    let old = std::mem::replace(&mut self.rows[ri], r);
                    let ol = std::mem::replace(&mut self.label[ri], k);
                    set(lead as u32, k);
                    let mut rest: Row = Vec::new();
                    ops += sub_monic(&old, &self.rows[ri], p, &mut rest);
                    if !rest.is_empty() {
                        cands.push((ol, rest));
                    }
                }
            }
        }
        (ops, rounds)
    }
}

fn cover_hist(cover: &[Vec<u16>], space: &Space, c: u32, kappa: &[u16]) -> Vec<u64> {
    let m = kappa.iter().filter(|&&k| k != NO_COVER).count();
    let mut h = vec![0u64; (c as usize + 1) * (m + 1)];
    for (cv, part) in cover.iter().zip(&space.parts) {
        for (col, &k) in cv.iter().enumerate() {
            if k != NO_COVER {
                h[part.deg[col] as usize * (m + 1) + (k as usize).min(m)] += 1;
            }
        }
    }
    h
}

fn rank_by_deg(ech: &[Echelon], space: &Space, c: u32) -> Vec<u64> {
    let mut r = vec![0u64; c as usize + 1];
    for (e, part) in ech.iter().zip(&space.parts) {
        for (col, &pv) in e.pivot.iter().enumerate() {
            if pv != NONE {
                r[part.deg[col] as usize] += 1;
            }
        }
    }
    r
}

/// `out = a − b` for two rows monic at the same leading column (sparse,
/// ascending, zeros dropped).
fn sub_monic(a: &Row, b: &Row, p: u64, out: &mut Row) -> u64 {
    out.clear();
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        let ca = a.get(i).map_or(u32::MAX, |t| t.0);
        let cb = b.get(j).map_or(u32::MAX, |t| t.0);
        if ca < cb {
            out.push(a[i]);
            i += 1;
        } else if cb < ca {
            out.push((cb, (p - u64::from(b[j].1)) as u32));
            j += 1;
        } else {
            let z = (u64::from(a[i].1) + p - u64::from(b[j].1)) % p;
            if z != 0 {
                out.push((ca, z as u32));
            }
            i += 1;
            j += 1;
        }
    }
    b.len() as u64
}

/// PROOF.md: the product by a variable of order `kb` of a row with leading
/// column `lead` (of `part`) lies in the final span, because `lead` is
/// covered with a smaller order. Fault 6 (negative control) also accepts
/// an equal order, fault 8 any order; the proof covers neither.
fn covered(cover: &[Vec<u16>], part: u8, lead: u32, kb: u16, fault: u8) -> bool {
    let Some(x) = cover.get(part as usize).and_then(|v| v.get(lead as usize)) else {
        return false;
    };
    *x != NO_COVER && (*x < kb || (fault == 6 && *x == kb) || fault == 8)
}

struct Scratch {
    acc: Vec<u64>,
    bm: Vec<u64>,
}

#[derive(Clone, Copy)]
struct Ctx {
    p: u64,
    p2: u64,
    barrett_m: u64,
    simd: Simd,
    gemm_simd: fp_gemm::GemmSimd,
    use_gemm: bool,
    bitmap: bool,
}

/// Counts of one reduction.
#[derive(Clone, Copy, Default)]
struct Red {
    ops: u64,
    nred: u32,
    /// Of `ops`, those of phase 0 (rows of the epoch's pivots).
    ops0: u64,
}

impl Echelon {
    /// An empty echelon of `n` columns (the identity epoch).
    fn new(n: usize) -> Self {
        Self::with_epoch(vec![NONE; n], Vec::new(), (0..n as u32).collect())
    }

    /// An echelon from its checkpointed parts (`of_col` rebuilt).
    pub(super) fn with_epoch(pivot: Vec<u32>, rows: Vec<RowS>, np_cols: Vec<u32>) -> Self {
        let mut of_col = vec![NONE; pivot.len()];
        for (j, &c) in np_cols.iter().enumerate() {
            of_col[c as usize] = j as u32;
        }
        Echelon {
            pivot,
            rows,
            np_cols,
            of_col,
        }
    }

    /// Index in `np_cols` of the first epoch column after column `lead`.
    #[inline]
    fn np_start(&self, lead: u32) -> usize {
        self.np_cols.partition_point(|&c| c <= lead)
    }

    /// The tail of a stored row (after its leading column), in the part's
    /// columns, ascending.
    fn tail_full(&self, row: &RowS) -> Row {
        match row {
            RowS::Sparse(r) => r[1..]
                .iter()
                .map(|&(j, x)| (self.np_cols[j as usize], x))
                .collect(),
            RowS::Dense { lead, v, nnz } => {
                let s = self.np_start(*lead);
                let mut t = Vec::with_capacity(*nnz as usize);
                for (i, &x) in v[1..].iter().enumerate() {
                    if x != 0 {
                        t.push((self.np_cols[s + i], x));
                    }
                }
                t
            }
        }
    }

    /// Stored row `ri` in the part's columns (monic, ascending).
    pub(super) fn row_full(&self, ri: usize) -> Row {
        let row = &self.rows[ri];
        let mut out = Vec::with_capacity(row.nnz());
        out.push((lead_of(row), 1));
        out.extend(self.tail_full(row));
        out
    }

    /// A monic row in the part's columns whose tail has no pivot column of
    /// the epoch, as a sparse stored row.
    fn to_epoch(&self, r: &[(u32, u32)]) -> RowS {
        let mut v: Row = Vec::with_capacity(r.len());
        v.push(r[0]);
        for &(c, x) in &r[1..] {
            let j = self.of_col[c as usize];
            assert!(
                j != NONE,
                "hky_fp_tower_fast2: a tail meets a pivot of the epoch"
            );
            v.push((j, x));
        }
        RowS::Sparse(v)
    }

    /// Storage form of a sparse stored row (dense when
    /// `nnz · den > span · num`, `span` the epoch columns from the lead on).
    fn densify(&self, row: RowS, num: usize, den: usize) -> RowS {
        let RowS::Sparse(r) = row else {
            return row;
        };
        let lead = r[0].0;
        let s = self.np_start(lead);
        let m = self.np_cols.len();
        let span = 1 + m - s;
        if r.len() * den > span * num {
            let mut v = vec![0u32; span];
            v[0] = 1;
            for &(j, x) in &r[1..] {
                v[1 + j as usize - s] = x;
            }
            RowS::Dense {
                lead,
                v,
                nnz: r.len() as u32,
            }
        } else {
            let mut r = r;
            r.shrink_to_fit();
            RowS::Sparse(r)
        }
    }

    /// Starts a new epoch: the epoch columns that have become pivots are
    /// dropped and every row's tail is rewritten in the new coordinates.
    /// Every tail must be free of pivot columns (true right after an
    /// interreduction); otherwise this panics.
    fn new_epoch(&mut self, num: usize, den: usize, fault: u8) {
        let pivot = &self.pivot;
        let old: &[u32] = &self.np_cols;
        let mut remap = vec![NONE; old.len()];
        let mut np_new: Vec<u32> = Vec::with_capacity(old.len());
        for (j, &c) in old.iter().enumerate() {
            if pivot[c as usize] == NONE {
                remap[j] = np_new.len() as u32;
                np_new.push(c);
            }
        }
        let m_new = np_new.len();
        let start_new = |lead: u32| np_new.partition_point(|&c| c <= lead);
        let start_old = |lead: u32| old.partition_point(|&c| c <= lead);
        let rows = std::mem::take(&mut self.rows);
        let rows: Vec<RowS> = rows
            .into_par_iter()
            .with_min_len(64)
            .map(|row| {
                let lead = lead_of(&row);
                let mut t: Row = Vec::with_capacity(row.nnz());
                t.push((lead, 1));
                let mut put = |j: usize, x: u32| {
                    let nj = remap[j];
                    assert!(
                        nj != NONE,
                        "hky_fp_tower_fast2: new epoch: a tail meets a pivot"
                    );
                    t.push((nj, x));
                };
                match &row {
                    RowS::Sparse(r) => {
                        for &(j, x) in &r[1..] {
                            put(j as usize, x);
                        }
                    }
                    RowS::Dense { v, .. } => {
                        let s = start_old(lead);
                        for (i, &x) in v[1..].iter().enumerate() {
                            if x != 0 {
                                put(s + i, x);
                            }
                        }
                    }
                }
                drop(row);
                if fault == 4 && t.len() > 2 {
                    t.pop();
                }
                let s = start_new(lead);
                let span = 1 + m_new - s;
                if t.len() * den > span * num {
                    let mut v = vec![0u32; span];
                    v[0] = 1;
                    for &(j, x) in &t[1..] {
                        v[1 + j as usize - s] = x;
                    }
                    RowS::Dense {
                        lead,
                        v,
                        nnz: t.len() as u32,
                    }
                } else {
                    t.shrink_to_fit();
                    RowS::Sparse(t)
                }
            })
            .collect();
        self.rows = rows;
        self.np_cols = np_new;
        self.of_col.iter_mut().for_each(|x| *x = NONE);
        for (j, &c) in self.np_cols.iter().enumerate() {
            self.of_col[c as usize] = j as u32;
        }
    }

    /// Fused interreduction + new epoch using blocked triangular back-substitution
    /// and exact-integer `f64` AVX-512/AVX2 GEMM (`C -= A * B mod p`).
    /// Mathematically identical to row-by-row `reduce_tail` followed by `new_epoch`.
    fn interreduce_and_epoch_gemm(&mut self, cx: &Ctx, dense: (usize, usize)) -> (u64, u64) {
        let m_old = self.np_cols.len();
        let mut p_new_cols: Vec<u32> = Vec::new();
        let mut p_new_rows: Vec<usize> = Vec::new();
        let mut j_to_p = vec![NONE; m_old];
        let mut np_next: Vec<u32> = Vec::with_capacity(m_old);
        let mut j_to_np = vec![NONE; m_old];

        for (j, &c) in self.np_cols.iter().enumerate() {
            let piv = self.pivot[c as usize];
            if piv != NONE {
                j_to_p[j] = p_new_cols.len() as u32;
                p_new_cols.push(c);
                p_new_rows.push(piv as usize);
            } else {
                j_to_np[j] = np_next.len() as u32;
                np_next.push(c);
            }
        }
        let k_tot = p_new_cols.len();
        let m_next = np_next.len();
        if k_tot == 0 {
            return (0, 0);
        }

        let p = cx.p;
        let bm = cx.barrett_m;
        let simd = cx.gemm_simd;
        let nr = fp_gemm::nr_for(simd);
        let k_safe = fp_gemm::k_safe_f64(p);
        let kc_max = 256usize.min(k_safe);
        let nc_tile = (240usize.div_ceil(nr) * nr).max(nr);
        let (d_num, d_den) = dense;

        let start_next = |lead: u32| np_next.partition_point(|&c| c <= lead);
        let start_old = |lead: u32| self.np_cols.partition_point(|&c| c <= lead);

        let make_row_next = |lead: u32, mut terms: Row| -> RowS {
            let s = start_next(lead);
            let span = 1 + m_next - s;
            if terms.len() * d_den > span * d_num {
                let mut v = vec![0u32; span];
                v[0] = 1;
                for &(nj, x) in &terms[1..] {
                    v[1 + nj as usize - s] = x;
                }
                RowS::Dense {
                    lead,
                    v,
                    nnz: terms.len() as u32,
                }
            } else {
                terms.shrink_to_fit();
                RowS::Sparse(terms)
            }
        };

        let prof = std::env::var_os("HKY3_PROF").is_some();
        let t_ir0 = Instant::now();
        let mut is_new_piv = vec![false; self.rows.len()];
        for &ri in &p_new_rows {
            is_new_piv[ri] = true;
        }

        let mut total_ops = 0u64;
        let mut total_changed = 0u64;

        // Phase A: back-substitute the `k_tot` new pivots in reverse blocks of TB=128
        // into `b_rows[0..k_tot]` (in `NP_next` coordinates).
        let mut b_rows: Vec<RowS> = (0..k_tot).map(|_| RowS::Sparse(Vec::new())).collect();
        const TB: usize = 128;
        let mut b1 = k_tot;
        while b1 > 0 {
            let b0 = b1.saturating_sub(TB);
            let bs = b1 - b0;
            let col0 = start_next(p_new_cols[b0]);
            let w_all = m_next - col0;

            let mut a_diag: Vec<Vec<(usize, u32)>> = vec![Vec::new(); bs];
            let mut a_ext: Vec<Vec<(usize, u32)>> = vec![Vec::new(); bs];
            let mut init_np: Vec<Vec<(usize, u32)>> = vec![Vec::new(); bs];
            let mut any_dense_ext = false;
            let mut block_ops = 0u64;

            for i in 0..bs {
                let ri = p_new_rows[b0 + i];
                let row = &self.rows[ri];
                match row {
                    RowS::Sparse(r) => {
                        for &(j, x) in &r[1..] {
                            let pk = j_to_p[j as usize];
                            if pk != NONE {
                                let pku = pk as usize;
                                if pku < b1 {
                                    a_diag[i].push((pku - b0, x));
                                } else {
                                    if matches!(b_rows[pku], RowS::Dense { .. }) {
                                        any_dense_ext = true;
                                    }
                                    block_ops += b_rows[pku].nnz() as u64;
                                    a_ext[i].push((pku, x));
                                }
                            } else {
                                init_np[i].push((j_to_np[j as usize] as usize - col0, x));
                            }
                        }
                    }
                    RowS::Dense { lead, v, .. } => {
                        let s = start_old(*lead);
                        for (off_i, &x) in v[1..].iter().enumerate() {
                            if x == 0 {
                                continue;
                            }
                            let j = s + off_i;
                            let pk = j_to_p[j];
                            if pk != NONE {
                                let pku = pk as usize;
                                if pku < b1 {
                                    a_diag[i].push((pku - b0, x));
                                } else {
                                    if matches!(b_rows[pku], RowS::Dense { .. }) {
                                        any_dense_ext = true;
                                    }
                                    block_ops += b_rows[pku].nnz() as u64;
                                    a_ext[i].push((pku, x));
                                }
                            } else {
                                init_np[i].push((j_to_np[j] as usize - col0, x));
                            }
                        }
                    }
                }
                if !a_diag[i].is_empty() || !a_ext[i].is_empty() {
                    total_changed += 1;
                }
            }
            total_ops += block_ops;

            if w_all == 0 {
                for i in 0..bs {
                    b_rows[b0 + i] = RowS::Sparse(vec![(p_new_cols[b0 + i], 1)]);
                }
                b1 = b0;
                continue;
            }

            // Fast sparse path when no dense external pivots exist and total block work is tiny.
            if !any_dense_ext && block_ops < (w_all as u64) * 8 {
                let mut acc = vec![0u64; w_all];
                let mut touched: Vec<usize> = Vec::new();
                for i in (0..bs).rev() {
                    touched.clear();
                    for &(col_rel, x) in &init_np[i] {
                        if acc[col_rel] == 0 {
                            touched.push(col_rel);
                        }
                        acc[col_rel] += u64::from(x);
                    }
                    let mut apply_sp = |b_row: &RowS, x: u32| {
                        let f = p - u64::from(x);
                        match b_row {
                            RowS::Sparse(r) => {
                                for &(nj, val) in &r[1..] {
                                    let c = nj as usize - col0;
                                    if acc[c] == 0 {
                                        touched.push(c);
                                    }
                                    let t = acc[c] + f * u64::from(val);
                                    acc[c] = t.min(t.wrapping_sub(cx.p2));
                                }
                            }
                            RowS::Dense { lead, v, .. } => {
                                let s = start_next(*lead) - col0;
                                for (k_off, &val) in v[1..].iter().enumerate() {
                                    if val != 0 {
                                        let c = s + k_off;
                                        if acc[c] == 0 {
                                            touched.push(c);
                                        }
                                        let t = acc[c] + f * u64::from(val);
                                        acc[c] = t.min(t.wrapping_sub(cx.p2));
                                    }
                                }
                            }
                        }
                    };
                    for &(pku, x) in &a_ext[i] {
                        apply_sp(&b_rows[pku], x);
                    }
                    for &(jl, x) in &a_diag[i] {
                        total_ops += b_rows[b0 + jl].nnz() as u64;
                        apply_sp(&b_rows[b0 + jl], x);
                    }
                    touched.sort_unstable();
                    touched.dedup();
                    let lead = p_new_cols[b0 + i];
                    let mut terms: Row = Vec::with_capacity(touched.len() + 1);
                    terms.push((lead, 1));
                    for &c in &touched {
                        let v = barrett(acc[c], p, bm);
                        acc[c] = 0;
                        if v != 0 {
                            terms.push(((col0 + c) as u32, v as u32));
                        }
                    }
                    b_rows[b0 + i] = make_row_next(lead, terms);
                }
                b1 = b0;
                continue;
            }

            // Dense/GEMM path for Phase A block.
            let mut c_blk = vec![0.0f64; bs * w_all];
            for i in 0..bs {
                let row_c = &mut c_blk[i * w_all..(i + 1) * w_all];
                for &(col_rel, x) in &init_np[i] {
                    row_c[col_rel] = x as f64;
                }
            }

            // Split external pivots into sparse (apply directly) and dense (GEMM once!).
            let mut dense_mark = vec![NONE; k_tot - b1];
            let mut dense_ext: Vec<(usize, usize)> = Vec::new(); // (start_next, pku)
            for i in 0..bs {
                let row_c = &mut c_blk[i * w_all..(i + 1) * w_all];
                for &(pku, x) in &a_ext[i] {
                    match &b_rows[pku] {
                        RowS::Sparse(r) => {
                            let f = (p - u64::from(x)) as f64;
                            for &(nj, val) in &r[1..] {
                                row_c[nj as usize - col0] += f * (val as f64);
                            }
                        }
                        RowS::Dense { lead, v, .. } => {
                            if v.len() > 1 {
                                let rel = pku - b1;
                                if dense_mark[rel] == NONE {
                                    dense_mark[rel] = dense_ext.len() as u32;
                                    dense_ext.push((start_next(*lead), pku));
                                }
                            }
                        }
                    }
                }
            }

            if !dense_ext.is_empty() {
                // Sort dense_ext by start_next so each column tile uses a contiguous prefix!
                dense_ext.sort_unstable_by_key(|&(s, _)| s);
                for (new_idx, &(_, pku)) in dense_ext.iter().enumerate() {
                    dense_mark[pku - b1] = new_idx as u32;
                }
                let kd = dense_ext.len();
                let mut a_dense = vec![0.0f64; bs * kd];
                let mut a_nnz = 0usize;
                for i in 0..bs {
                    for &(pku, x) in &a_ext[i] {
                        let dm = dense_mark[pku - b1];
                        if dm != NONE {
                            a_dense[i * kd + dm as usize] = (p - u64::from(x)) as f64;
                            a_nnz += 1;
                        }
                    }
                }
                let use_dense_gemm = bs >= 6 && kd >= 8 && a_nnz * 5 >= bs * kd;
                let a_sparse: Vec<Vec<(u32, f64)>> = if use_dense_gemm {
                    Vec::new()
                } else {
                    (0..bs)
                        .map(|i| {
                            let mut v: Vec<(u32, f64)> = a_ext[i]
                                .iter()
                                .filter_map(|&(pku, x)| {
                                    let dm = dense_mark[pku - b1];
                                    (dm != NONE).then_some((dm, (p - u64::from(x)) as f64))
                                })
                                .collect();
                            v.sort_unstable_by_key(|t| t.0);
                            v
                        })
                        .collect()
                };

                let tiles: Vec<(usize, usize)> = (0..w_all)
                    .step_by(nc_tile)
                    .map(|t0| (t0, (t0 + nc_tile).min(w_all)))
                    .collect();
                let b_ref = &b_rows;
                let dex_ref = &dense_ext;
                let ad_ref = &a_dense;
                let as_ref = &a_sparse;
                let c_tiles: Vec<(usize, usize, Vec<f64>)> = tiles
                    .into_par_iter()
                    .with_max_len(1)
                    .map(|(t0, t1)| {
                        let tw = t1 - t0;
                        let j0 = col0 + t0;
                        let j1 = col0 + t1;
                        let mut ct = vec![0.0f64; bs * tw];
                        for i in 0..bs {
                            ct[i * tw..(i + 1) * tw]
                                .copy_from_slice(&c_blk[i * w_all + t0..i * w_all + t1]);
                        }
                        fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm);
                        let kd_act = dex_ref.partition_point(|&(s, _)| s < j1);
                        let mut k_acc = 0usize;
                        if use_dense_gemm {
                            let mut b_pack = Vec::new();
                            let mut kk0 = 0;
                            while kk0 < kd_act {
                                let kc = (kd_act - kk0).min(kc_max);
                                if k_acc + kc > k_safe {
                                    fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm);
                                    k_acc = 0;
                                }
                                fp_gemm::prepare_b_pack(&mut b_pack, kc, tw, nr);
                                for kk in 0..kc {
                                    let (s, pku) = dex_ref[kk0 + kk];
                                    if let RowS::Dense { v, .. } = &b_ref[pku] {
                                        let a_col = s.max(j0);
                                        let src = &v[1 + a_col - s..1 + j1 - s];
                                        fp_gemm::pack_b_put_u32_row(
                                            simd,
                                            &mut b_pack,
                                            kc,
                                            nr,
                                            kk,
                                            a_col - j0,
                                            src,
                                        );
                                    }
                                }
                                fp_gemm::gemm_f64_with_packed_b(
                                    simd,
                                    bs,
                                    tw,
                                    kc,
                                    &ad_ref[kk0..],
                                    kd,
                                    &b_pack,
                                    &mut ct,
                                    tw,
                                );
                                k_acc += kc;
                                kk0 += kc;
                            }
                        } else {
                            let ldb = tw.div_ceil(8) * 8;
                            let mut b_tile = Vec::new();
                            let mut kk0 = 0;
                            while kk0 < kd_act {
                                let kc = (kd_act - kk0).min(kc_max);
                                let kk1 = kk0 + kc;
                                if k_acc + kc > k_safe {
                                    fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm);
                                    k_acc = 0;
                                }
                                b_tile.clear();
                                b_tile.resize(kc * ldb, 0.0);
                                for kk in 0..kc {
                                    let (s, pku) = dex_ref[kk0 + kk];
                                    if let RowS::Dense { v, .. } = &b_ref[pku] {
                                        let a_col = s.max(j0);
                                        let src = &v[1 + a_col - s..1 + j1 - s];
                                        let dst =
                                            &mut b_tile[kk * ldb + (a_col - j0)..][..src.len()];
                                        fp_gemm::cvt_u32_to_f64(simd, src, dst);
                                    }
                                }
                                let mut sub_a: Vec<(u32, f64)> = Vec::new();
                                for i in 0..bs {
                                    let row_s = &as_ref[i];
                                    let lo = row_s.partition_point(|t| (t.0 as usize) < kk0);
                                    let hi = row_s.partition_point(|t| (t.0 as usize) < kk1);
                                    if lo < hi {
                                        sub_a.clear();
                                        for &(k_idx, f) in &row_s[lo..hi] {
                                            sub_a.push((k_idx - kk0 as u32, f));
                                        }
                                        fp_gemm::spmm_row_f64(
                                            simd,
                                            &mut ct[i * tw..(i + 1) * tw],
                                            tw,
                                            &sub_a,
                                            &b_tile,
                                            ldb,
                                        );
                                    }
                                }
                                k_acc += kc;
                                kk0 = kk1;
                            }
                        }
                        fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm);
                        (t0, tw, ct)
                    })
                    .collect();
                for (t0, tw, ct) in c_tiles {
                    for i in 0..bs {
                        c_blk[i * w_all + t0..i * w_all + t0 + tw]
                            .copy_from_slice(&ct[i * tw..(i + 1) * tw]);
                    }
                }
            } else {
                fp_gemm::reduce_f64_slice_mod_p(&mut c_blk, p, bm);
            }

            // In-block triangular back-substitution (reverse order i = bs-1..0).
            for i in (0..bs).rev() {
                if a_diag[i].is_empty() {
                    continue;
                }
                let si = start_next(p_new_cols[b0 + i]) - col0;
                let (top, bot) = c_blk.split_at_mut((i + 1) * w_all);
                let row_i = &mut top[i * w_all..];
                let mut k_acc = 0usize;
                for &(jl, x) in &a_diag[i] {
                    if k_acc >= k_safe {
                        fp_gemm::reduce_f64_slice_mod_p(&mut row_i[si..], p, bm);
                        k_acc = 0;
                    }
                    let sj = start_next(p_new_cols[b0 + jl]) - col0;
                    let row_j = &bot[(jl - i - 1) * w_all..(jl - i) * w_all];
                    let f = (p - u64::from(x)) as f64;
                    fp_gemm::axpy_f64(simd, &mut row_i[sj..], &row_j[sj..], f);
                    total_ops += (w_all - sj) as u64;
                    k_acc += 1;
                }
                fp_gemm::reduce_f64_slice_mod_p(&mut row_i[si..], p, bm);
            }

            // Convert each solved row of c_blk into compact RowS in NP_next coordinates.
            let built: Vec<RowS> = (0..bs)
                .into_par_iter()
                .map(|i| {
                    let lead = p_new_cols[b0 + i];
                    let si = start_next(lead) - col0;
                    let span = 1 + m_next - (col0 + si);
                    let row_c = &c_blk[i * w_all + si..(i + 1) * w_all];
                    let mut v = vec![0u32; span];
                    v[0] = 1;
                    let nnz = 1 + fp_gemm::reduce_f64_to_u32(simd, row_c, &mut v[1..], p, bm);
                    if (nnz as usize) * d_den > span * d_num {
                        RowS::Dense { lead, v, nnz }
                    } else {
                        let mut terms: Row = Vec::with_capacity(nnz as usize);
                        terms.push((lead, 1));
                        for (off_j, &x) in v[1..].iter().enumerate() {
                            if x != 0 {
                                terms.push(((col0 + si + off_j) as u32, x));
                            }
                        }
                        RowS::Sparse(terms)
                    }
                })
                .collect();
            for (i, r) in built.into_iter().enumerate() {
                b_rows[b0 + i] = r;
            }
            b1 = b0;
        }
        let t_phase_a = t_ir0.elapsed().as_secs_f64();

        // Phase B: Classify all old rows (not in p_new_rows) into untouched, sparse-affected,
        // and dense-affected (rows that touch at least one dense `b_rows` pivot).
        let b_is_dense: Vec<bool> = b_rows
            .iter()
            .map(|r| matches!(r, RowS::Dense { v, .. } if v.len() > 1))
            .collect();
        let mut untouched: Vec<usize> = Vec::new();
        let mut affected_sparse: Vec<usize> = Vec::new();
        let mut affected_dense: Vec<(u32, usize)> = Vec::new();
        for (ri, row) in self.rows.iter().enumerate() {
            if is_new_piv[ri] {
                continue;
            }
            let mut has_p = false;
            let mut hits_dense_b = false;
            match row {
                RowS::Sparse(r) => {
                    for &(j, _) in &r[1..] {
                        let pk = j_to_p[j as usize];
                        if pk != NONE {
                            has_p = true;
                            if b_is_dense[pk as usize] {
                                hits_dense_b = true;
                                break;
                            }
                        }
                    }
                }
                RowS::Dense { lead, v, .. } => {
                    let s = start_old(*lead);
                    for (i, &x) in v[1..].iter().enumerate() {
                        if x != 0 {
                            let pk = j_to_p[s + i];
                            if pk != NONE {
                                has_p = true;
                                if b_is_dense[pk as usize] {
                                    hits_dense_b = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            if !has_p {
                untouched.push(ri);
            } else if hits_dense_b {
                affected_dense.push((lead_of(row), ri));
            } else {
                affected_sparse.push(ri);
            }
        }
        total_changed += (affected_dense.len() + affected_sparse.len()) as u64;

        // 1. Remap untouched rows directly into NP_next coordinates in parallel.
        let untouched_built: Vec<(usize, RowS)> = untouched
            .into_par_iter()
            .with_min_len(64)
            .map(|ri| {
                let row = &self.rows[ri];
                let lead = lead_of(row);
                let mut terms: Row = Vec::with_capacity(row.nnz());
                terms.push((lead, 1));
                match row {
                    RowS::Sparse(r) => {
                        for &(j, x) in &r[1..] {
                            terms.push((j_to_np[j as usize], x));
                        }
                    }
                    RowS::Dense { v, .. } => {
                        let s = start_old(lead);
                        for (i, &x) in v[1..].iter().enumerate() {
                            if x != 0 {
                                terms.push((j_to_np[s + i], x));
                            }
                        }
                    }
                }
                (ri, make_row_next(lead, terms))
            })
            .collect();

        // 2. Reduce rows that only touch sparse `b_rows` in parallel without dense allocation.
        let b_ref = &b_rows;
        let rows_ref = &self.rows;
        let j_to_p_ref = &j_to_p;
        let j_to_np_ref = &j_to_np;
        let sparse_res: Vec<(u64, Vec<(usize, RowS)>)> = affected_sparse
            .par_chunks(128)
            .map(|chunk| {
                let mut acc = vec![0u64; m_next];
                let mut touched: Vec<usize> = Vec::new();
                let mut ops = 0u64;
                let mut out = Vec::with_capacity(chunk.len());
                for &ri in chunk {
                    touched.clear();
                    let row = &rows_ref[ri];
                    let lead = lead_of(row);
                    let mut apply = |j: usize, x: u32| {
                        let pk = j_to_p_ref[j];
                        if pk != NONE {
                            let f = p - u64::from(x);
                            if let RowS::Sparse(br) = &b_ref[pk as usize] {
                                ops += br.len() as u64;
                                for &(nj, val) in &br[1..] {
                                    let c = nj as usize;
                                    if acc[c] == 0 {
                                        touched.push(c);
                                    }
                                    let t = acc[c] + f * u64::from(val);
                                    acc[c] = t.min(t.wrapping_sub(cx.p2));
                                }
                            }
                        } else {
                            let c = j_to_np_ref[j] as usize;
                            if acc[c] == 0 {
                                touched.push(c);
                            }
                            let t = acc[c] + u64::from(x);
                            acc[c] = t.min(t.wrapping_sub(cx.p2));
                        }
                    };
                    match row {
                        RowS::Sparse(r) => {
                            for &(j, x) in &r[1..] {
                                apply(j as usize, x);
                            }
                        }
                        RowS::Dense { v, .. } => {
                            let s = start_old(lead);
                            for (i, &x) in v[1..].iter().enumerate() {
                                if x != 0 {
                                    apply(s + i, x);
                                }
                            }
                        }
                    }
                    touched.sort_unstable();
                    touched.dedup();
                    let mut terms: Row = Vec::with_capacity(touched.len() + 1);
                    terms.push((lead, 1));
                    for &c in &touched {
                        let v = barrett(acc[c], p, bm);
                        acc[c] = 0;
                        if v != 0 {
                            terms.push((c as u32, v as u32));
                        }
                    }
                    out.push((ri, make_row_next(lead, terms)));
                }
                (ops, out)
            })
            .collect();

        // 3. Split rows touching dense `b_rows` into high-fan-in (`dense_a`, >= 32 dense hits)
        //    vs low-fan-in (`sparse_a`), so dense 6x24 GEMM never wastes flops on sparse rows!
        let mut affected_dense_hi: Vec<(u32, usize)> = Vec::new();
        let mut affected_dense_lo: Vec<(u32, usize)> = Vec::new();
        for &(ld, ri) in &affected_dense {
            let hits = match &self.rows[ri] {
                RowS::Sparse(r) => r[1..]
                    .iter()
                    .filter(|&&(j, _)| {
                        let pk = j_to_p[j as usize];
                        pk != NONE && b_is_dense[pk as usize]
                    })
                    .count(),
                RowS::Dense { v, .. } => {
                    let s = start_old(ld);
                    v[1..]
                        .iter()
                        .enumerate()
                        .filter(|&(i, &x)| {
                            if x == 0 {
                                return false;
                            }
                            let pk = j_to_p[s + i];
                            pk != NONE && b_is_dense[pk as usize]
                        })
                        .count()
                }
            };
            if hits >= 32 {
                affected_dense_hi.push((ld, ri));
            } else {
                affected_dense_lo.push((ld, ri));
            }
        }
        affected_dense_hi.par_sort_unstable_by_key(|&(ld, _)| ld);
        affected_dense_lo.par_sort_unstable_by_key(|&(ld, _)| ld);

        let mut updated_affected: Vec<(usize, RowS)> = Vec::with_capacity(affected_dense.len());
        for (bucket_is_hi, bucket) in [(true, &affected_dense_hi), (false, &affected_dense_lo)] {
            let mut idx = 0usize;
            while idx < bucket.len() {
                let min_lead = bucket[idx].0;
                let min_np = start_next(min_lead);
                let span_max = (m_next - min_np).max(1);
                let cap_rows = if bucket_is_hi { 1536 } else { 768 };
                let m_super = ((48_000_000usize / span_max).clamp(192, cap_rows) / 6).max(1) * 6;
                let end = (idx + m_super).min(bucket.len());
                let chunk = &bucket[idx..end];
                idx = end;
                let ms = chunk.len();

                // Precompute dense_b list + sparse_b contributions ONCE for this super-strip.
                let mut dense_mark = vec![NONE; k_tot];
                let mut dense_b: Vec<(usize, usize)> = Vec::new(); // (start_next, pku)
                let mut row_p_dense: Vec<Vec<(usize, u32)>> = Vec::with_capacity(ms);
                let mut row_p_sparse: Vec<Vec<(usize, u32)>> = Vec::with_capacity(ms);
                let mut row_np: Vec<Vec<(u32, u32)>> = Vec::with_capacity(ms);
                let mut strip_ops = 0u64;

                for &(lead, ri) in chunk {
                    let mut rpd = Vec::new();
                    let mut rps = Vec::new();
                    let mut rnp = Vec::new();
                    let mut visit = |j: usize, x: u32| {
                        let pk = j_to_p_ref[j];
                        if pk != NONE {
                            let pku = pk as usize;
                            strip_ops += b_ref[pku].nnz() as u64;
                            if b_is_dense[pku] {
                                if dense_mark[pku] == NONE {
                                    dense_mark[pku] = dense_b.len() as u32;
                                    dense_b.push((start_next(p_new_cols[pku]), pku));
                                }
                                rpd.push((pku, x));
                            } else {
                                rps.push((pku, x));
                            }
                        } else {
                            rnp.push((j_to_np_ref[j], x));
                        }
                    };
                    match &rows_ref[ri] {
                        RowS::Sparse(r) => {
                            for &(j, x) in &r[1..] {
                                visit(j as usize, x);
                            }
                        }
                        RowS::Dense { v, .. } => {
                            let s = start_old(lead);
                            for (i, &x) in v[1..].iter().enumerate() {
                                if x != 0 {
                                    visit(s + i, x);
                                }
                            }
                        }
                    }
                    row_p_dense.push(rpd);
                    row_p_sparse.push(rps);
                    row_np.push(rnp);
                }
                total_ops += strip_ops;

                // Sort dense_b by start_next so tile [j0, j1) uses prefix 0..kd_act!
                dense_b.sort_unstable_by_key(|&(s, _)| s);
                for (new_idx, &(_, pku)) in dense_b.iter().enumerate() {
                    dense_mark[pku] = new_idx as u32;
                }
                let kd = dense_b.len();
                let a_nnz: usize = row_p_dense.iter().map(|r| r.len()).sum();
                let use_dense_gemm = bucket_is_hi && ms >= 6 && kd >= 8 && a_nnz * 3 >= ms * kd;

                let mut a_dense: Vec<f64> = Vec::new();
                let mut a_sparse: Vec<Vec<(u32, f64)>> = Vec::new();
                if use_dense_gemm {
                    a_dense.resize(ms * kd, 0.0);
                    for i in 0..ms {
                        let base = i * kd;
                        for &(pku, x) in &row_p_dense[i] {
                            a_dense[base + dense_mark[pku] as usize] = (p - u64::from(x)) as f64;
                        }
                    }
                } else {
                    a_sparse = (0..ms)
                        .map(|i| {
                            let mut v: Vec<(u32, f64)> = row_p_dense[i]
                                .iter()
                                .map(|&(pku, x)| (dense_mark[pku], (p - u64::from(x)) as f64))
                                .collect();
                            v.sort_unstable_by_key(|t| t.0);
                            v
                        })
                        .collect();
                }

                let row_starts: Vec<usize> =
                    chunk.iter().map(|&(lead, _)| start_next(lead)).collect();
                let mut v_rows: Vec<Vec<u32>> = row_starts
                    .iter()
                    .map(|&s| {
                        let mut v = vec![0u32; 1 + m_next - s];
                        v[0] = 1;
                        v
                    })
                    .collect();

                // Pre-seed v_rows with each row's own NP_next entries + sparse `b_rows` ONCE in parallel!
                v_rows.par_iter_mut().enumerate().for_each(|(i, v)| {
                    let s = row_starts[i];
                    if row_p_sparse[i].is_empty() {
                        for &(nj, x) in &row_np[i] {
                            v[1 + nj as usize - s] = x;
                        }
                    } else {
                        let span_t = m_next - s;
                        let mut acc = vec![0u64; span_t];
                        for &(nj, x) in &row_np[i] {
                            acc[nj as usize - s] = u64::from(x);
                        }
                        for &(pku, x) in &row_p_sparse[i] {
                            let f = p - u64::from(x);
                            if let RowS::Sparse(br) = &b_ref[pku] {
                                for &(nj, val) in &br[1..] {
                                    let idx_c = nj as usize - s;
                                    let t = acc[idx_c] + f * u64::from(val);
                                    acc[idx_c] = t.min(t.wrapping_sub(cx.p2));
                                }
                            }
                        }
                        for (off_j, &val) in acc.iter().enumerate() {
                            if val != 0 {
                                v[1 + off_j] = barrett(val, p, bm) as u32;
                            }
                        }
                    }
                });

                let row_ptrs: Vec<usize> =
                    v_rows.iter_mut().map(|v| v.as_mut_ptr() as usize).collect();
                let tiles: Vec<(usize, usize)> = (min_np..m_next)
                    .step_by(nc_tile)
                    .map(|j0| (j0, (j0 + nc_tile).min(m_next)))
                    .collect();
                let db_ref = &dense_b;
                let ad_ref = &a_dense;
                let as_ref = &a_sparse;

                let tile_nnz: Vec<Vec<u32>> = tiles
                    .into_par_iter()
                    .with_max_len(1)
                    .map(|(j0, j1)| {
                        let w = j1 - j0;
                        let kd_act = db_ref.partition_point(|&(s, _)| s < j1);
                        let mut c_tile = vec![0.0f64; ms * w];
                        for i in 0..ms {
                            let s = row_starts[i];
                            if s >= j1 {
                                continue;
                            }
                            let a = s.max(j0);
                            // SAFETY: disjoint column range [j0, j1) owned by this tile.
                            let src_u32 = unsafe {
                                std::slice::from_raw_parts(
                                    (row_ptrs[i] as *const u32).add(1 + a - s),
                                    j1 - a,
                                )
                            };
                            fp_gemm::cvt_u32_to_f64(
                                simd,
                                src_u32,
                                &mut c_tile[i * w + (a - j0)..i * w + w],
                            );
                        }

                        let mut k_acc = 0usize;
                        if use_dense_gemm {
                            let mut b_pack = Vec::new();
                            let mut kk0 = 0;
                            while kk0 < kd_act {
                                let kc = (kd_act - kk0).min(kc_max);
                                if k_acc + kc > k_safe {
                                    fp_gemm::reduce_f64_slice_mod_p(&mut c_tile, p, bm);
                                    k_acc = 0;
                                }
                                fp_gemm::prepare_b_pack(&mut b_pack, kc, w, nr);
                                for kk in 0..kc {
                                    let (s, pku) = db_ref[kk0 + kk];
                                    if let RowS::Dense { v, .. } = &b_ref[pku] {
                                        let a_col = s.max(j0);
                                        let src = &v[1 + a_col - s..1 + j1 - s];
                                        fp_gemm::pack_b_put_u32_row(
                                            simd,
                                            &mut b_pack,
                                            kc,
                                            nr,
                                            kk,
                                            a_col - j0,
                                            src,
                                        );
                                    }
                                }
                                fp_gemm::gemm_f64_with_packed_b(
                                    simd,
                                    ms,
                                    w,
                                    kc,
                                    &ad_ref[kk0..],
                                    kd,
                                    &b_pack,
                                    &mut c_tile,
                                    w,
                                );
                                k_acc += kc;
                                kk0 += kc;
                            }
                        } else {
                            let ldb = w.div_ceil(8) * 8;
                            let mut b_tile = Vec::new();
                            let mut sub_a: Vec<(u32, f64)> = Vec::new();
                            let mut kk0 = 0;
                            while kk0 < kd_act {
                                let kc = (kd_act - kk0).min(kc_max);
                                let kk1 = kk0 + kc;
                                if k_acc + kc > k_safe {
                                    fp_gemm::reduce_f64_slice_mod_p(&mut c_tile, p, bm);
                                    k_acc = 0;
                                }
                                b_tile.clear();
                                b_tile.resize(kc * ldb, 0.0);
                                for kk in 0..kc {
                                    let (s, pku) = db_ref[kk0 + kk];
                                    if let RowS::Dense { v, .. } = &b_ref[pku] {
                                        let a_col = s.max(j0);
                                        let src = &v[1 + a_col - s..1 + j1 - s];
                                        let dst =
                                            &mut b_tile[kk * ldb + (a_col - j0)..][..src.len()];
                                        fp_gemm::cvt_u32_to_f64(simd, src, dst);
                                    }
                                }
                                for i in 0..ms {
                                    let row_s = &as_ref[i];
                                    let lo = row_s.partition_point(|t| (t.0 as usize) < kk0);
                                    let hi = row_s.partition_point(|t| (t.0 as usize) < kk1);
                                    if lo < hi {
                                        sub_a.clear();
                                        for &(k_idx, f) in &row_s[lo..hi] {
                                            sub_a.push((k_idx - kk0 as u32, f));
                                        }
                                        fp_gemm::spmm_row_f64(
                                            simd,
                                            &mut c_tile[i * w..(i + 1) * w],
                                            w,
                                            &sub_a,
                                            &b_tile,
                                            ldb,
                                        );
                                    }
                                }
                                k_acc += kc;
                                kk0 = kk1;
                            }
                        }

                        let mut nnz_local = vec![0u32; ms];
                        for i in 0..ms {
                            let s = row_starts[i];
                            if s >= j1 {
                                continue;
                            }
                            let a = s.max(j0);
                            let src = &c_tile[i * w + (a - j0)..i * w + w];
                            // SAFETY: each tile owns disjoint column range [j0, j1) of row i.
                            let dst = unsafe {
                                std::slice::from_raw_parts_mut(
                                    (row_ptrs[i] as *mut u32).add(1 + a - s),
                                    j1 - a,
                                )
                            };
                            nnz_local[i] = fp_gemm::reduce_f64_to_u32(simd, src, dst, p, bm);
                        }
                        nnz_local
                    })
                    .collect();

                let mut total_nnz = vec![1u32; ms];
                for tn in &tile_nnz {
                    for i in 0..ms {
                        total_nnz[i] += tn[i];
                    }
                }

                let built: Vec<(usize, RowS)> = chunk
                    .par_iter()
                    .zip(v_rows.into_par_iter())
                    .zip(row_starts.par_iter())
                    .zip(total_nnz.par_iter())
                    .map(|(((&(lead, ri), v), &s), &nnz)| {
                        let span = 1 + m_next - s;
                        if (nnz as usize) * d_den > span * d_num {
                            (ri, RowS::Dense { lead, v, nnz })
                        } else {
                            let mut terms: Row = Vec::with_capacity(nnz as usize);
                            terms.push((lead, 1));
                            for (off_j, &x) in v[1..].iter().enumerate() {
                                if x != 0 {
                                    terms.push(((s + off_j) as u32, x));
                                }
                            }
                            (ri, RowS::Sparse(terms))
                        }
                    })
                    .collect();
                updated_affected.extend(built);
            }
        }

        // Install all updated rows and commit the new epoch.
        for (ri, r) in untouched_built {
            self.rows[ri] = r;
        }
        for (ops, built) in sparse_res {
            total_ops += ops;
            for (ri, r) in built {
                self.rows[ri] = r;
            }
        }
        for (ri, r) in updated_affected {
            self.rows[ri] = r;
        }
        let b_dense = b_rows
            .iter()
            .filter(|r| matches!(r, RowS::Dense { .. }))
            .count();
        for (k, r) in b_rows.into_iter().enumerate() {
            self.rows[p_new_rows[k]] = r;
        }
        self.np_cols = np_next;
        self.of_col.iter_mut().for_each(|x| *x = NONE);
        for (j, &c) in self.np_cols.iter().enumerate() {
            self.of_col[c as usize] = j as u32;
        }
        if prof {
            let t_tot = t_ir0.elapsed().as_secs_f64();
            eprintln!(
                "  [ir_prof] k={k_tot} (b_dense={b_dense}) m_next={m_next} aff_d={} aff_s={} tot={t_tot:.3}s phaseA={t_phase_a:.3}s phaseB={:.3}s ops={total_ops} ({:.1} G/s)",
                affected_dense.len(),
                affected_sparse.len(),
                t_tot - t_phase_a,
                (total_ops as f64) / t_tot.max(1e-9) / 1e9
            );
        }
        (total_ops, total_changed)
    }

    /// `acc[tail] += f · tail` of a stored row, in epoch coordinates.
    /// Returns the last index touched; marks the bitmap when `mark`.
    #[inline]
    fn apply(
        &self,
        cx: &Ctx,
        row: &RowS,
        f: u64,
        acc: &mut [u64],
        bm: &mut [u64],
        mark: bool,
    ) -> usize {
        let p2 = cx.p2;
        match row {
            RowS::Sparse(r) => {
                if mark {
                    for &(j, x) in &r[1..] {
                        let i = j as usize;
                        let t = acc[i] + f * u64::from(x);
                        acc[i] = t.min(t.wrapping_sub(p2));
                        bm[i >> 6] |= 1u64 << (j & 63);
                    }
                } else {
                    for &(j, x) in &r[1..] {
                        let i = j as usize;
                        let t = acc[i] + f * u64::from(x);
                        acc[i] = t.min(t.wrapping_sub(p2));
                    }
                }
                if r.len() > 1 {
                    r[r.len() - 1].0 as usize
                } else {
                    0
                }
            }
            RowS::Dense { lead, v, .. } => {
                let s = self.np_start(*lead);
                if v.len() > 1 {
                    axpy_dense(cx.simd, &mut acc[s..s + v.len() - 1], &v[1..], f, p2);
                    s + v.len() - 2
                } else {
                    0
                }
            }
        }
    }

    /// Eliminates every pivot column from `r` (the part's columns, ascending,
    /// values below `p`), following the cascade, and appends the remaining
    /// terms (the part's columns, ascending, not normalised) to `out`.
    ///
    /// Phase 0: a term of `r` at a pivot column of the epoch is final (no
    /// stored tail has such a column), so its row is subtracted at once; the
    /// other terms go to the accumulator. Then the cascade over the
    /// accumulator in epoch coordinates, against the rows inserted since the
    /// epoch (their leads are epoch columns), as v2: a bitmap while sparse,
    /// then a scan. The result is the unique vector of `r + span(rows)` that
    /// is zero at every pivot column, as before; the rows subtracted and
    /// their factors are the same, so the multiply-adds are counted alike.
    fn eliminate(&self, cx: &Ctx, r: &[(u32, u32)], s: &mut Scratch, out: &mut Row) -> Red {
        let (p, bm_) = (cx.p, cx.barrett_m);
        let m = self.np_cols.len();
        let mut red = Red::default();
        if r.is_empty() {
            return red;
        }
        let acc = &mut s.acc[..m];
        let bm = &mut s.bm;
        // first epoch index touched, and the shape of phase 0
        let mut first = usize::MAX;
        let mut last = 0usize;
        let mut core = false;
        for &(col, _) in r {
            let j = self.of_col[col as usize];
            if j != NONE {
                first = first.min(j as usize);
                last = last.max(j as usize);
            } else {
                core = true;
                let piv = self.pivot[col as usize];
                assert!(
                    piv != NONE,
                    "hky_fp_tower_fast2: a column is neither in the epoch nor a pivot"
                );
                match &self.rows[piv as usize] {
                    RowS::Sparse(rr) => {
                        if rr.len() > 1 {
                            first = first.min(rr[1].0 as usize);
                        }
                    }
                    RowS::Dense { v, .. } => {
                        if v.len() > 1 {
                            first = first.min(self.np_start(col));
                        }
                    }
                }
            }
        }
        if first == usize::MAX {
            // only pivot columns whose rows have no tail: r reduces to 0
            for &(col, x) in r {
                let v = u64::from(x) % p;
                if v != 0 {
                    let piv = self.pivot[col as usize];
                    red.nred += 1;
                    red.ops += self.rows[piv as usize].nnz() as u64;
                    red.ops0 += self.rows[piv as usize].nnz() as u64;
                }
            }
            return red;
        }
        let span = m - first;
        let budget = (span / 4) as u64;
        let mut marking = cx.bitmap && r.len() * 8 < span;
        let marked = marking;
        for &(col, x) in r {
            let j = self.of_col[col as usize];
            if j != NONE {
                acc[j as usize] = u64::from(x);
                if marking {
                    bm[j as usize >> 6] |= 1u64 << (j & 63);
                }
            }
        }
        if core {
            for &(col, x) in r {
                if self.of_col[col as usize] != NONE {
                    continue;
                }
                let v = u64::from(x) % p;
                if v == 0 {
                    continue;
                }
                let row = &self.rows[self.pivot[col as usize] as usize];
                red.nred += 1;
                red.ops += row.nnz() as u64;
                red.ops0 += row.nnz() as u64;
                if matches!(row, RowS::Dense { .. }) {
                    marking = false;
                }
                let l = self.apply(cx, row, p - v, acc, bm, marking);
                last = last.max(l);
                if marking && red.ops > budget {
                    marking = false;
                }
            }
        }
        let mut ci = first;
        if marking {
            // Phase A: follow the bitmap while the reduction is sparse.
            let mut w = ci >> 6;
            let mut switched = false;
            'a: loop {
                let mut word = bm[w];
                while word == 0 {
                    w += 1;
                    if w > last >> 6 {
                        break 'a;
                    }
                    word = bm[w];
                }
                ci = (w << 6) | word.trailing_zeros() as usize;
                bm[w] = word & (word - 1);
                let raw = acc[ci];
                if raw == 0 {
                    continue;
                }
                acc[ci] = 0;
                let v = barrett(raw, p, bm_);
                if v == 0 {
                    continue;
                }
                let col = self.np_cols[ci];
                let piv = self.pivot[col as usize];
                if piv == NONE {
                    out.push((col, v as u32));
                    continue;
                }
                let row = &self.rows[piv as usize];
                red.nred += 1;
                red.ops += row.nnz() as u64;
                let dense = matches!(row, RowS::Dense { .. });
                let l = self.apply(cx, row, p - v, acc, bm, !dense);
                last = last.max(l);
                if dense || red.ops > budget {
                    switched = true;
                }
                if switched {
                    for x in &mut bm[w..=last >> 6] {
                        *x = 0;
                    }
                    ci += 1;
                    break 'a;
                }
            }
            if !switched {
                return red;
            }
        } else if marked {
            // phase 0 stopped marking: clear what it marked
            for x in &mut bm[first >> 6..=last >> 6] {
                *x = 0;
            }
        }
        // Phase B: the scan.
        self.scan(cx, acc, ci, last, &mut red, out);
        red
    }

    /// The cascade by a scan of `acc[ci..=last]` (epoch coordinates), against
    /// the rows of the pivots inserted since the epoch; leaves `acc` zero
    /// there and appends the other terms (the part's columns) to `out`.
    fn scan(
        &self,
        cx: &Ctx,
        acc: &mut [u64],
        mut ci: usize,
        mut last: usize,
        red: &mut Red,
        out: &mut Row,
    ) {
        let (p, bm_) = (cx.p, cx.barrett_m);
        while ci <= last {
            let raw = acc[ci];
            if raw == 0 {
                ci += 1;
                continue;
            }
            acc[ci] = 0;
            let v = barrett(raw, p, bm_);
            if v == 0 {
                ci += 1;
                continue;
            }
            let col = self.np_cols[ci];
            let piv = self.pivot[col as usize];
            if piv == NONE {
                out.push((col, v as u32));
                ci += 1;
                continue;
            }
            let row = &self.rows[piv as usize];
            red.nred += 1;
            red.ops += row.nnz() as u64;
            let l = self.apply(cx, row, p - v, acc, &mut [], false);
            last = last.max(l);
            ci += 1;
        }
    }

    /// A row reduced against the echelon and made monic (empty if zero),
    /// in the part's columns.
    fn reduce(&self, cx: &Ctx, r: &[(u32, u32)], s: &mut Scratch) -> (Row, Red) {
        let mut out = Vec::new();
        let mut red = self.eliminate(cx, r, s, &mut out);
        red.ops += normalise(&mut out, cx.p);
        (out, red)
    }

    /// Whether a stored row has a pivot column after its leading one.
    fn tail_meets_pivot(&self, row: &RowS) -> bool {
        match row {
            RowS::Sparse(r) => r[1..]
                .iter()
                .any(|&(j, _)| self.pivot[self.np_cols[j as usize] as usize] != NONE),
            RowS::Dense { lead, v, .. } => {
                let s = self.np_start(*lead);
                v[1..]
                    .iter()
                    .zip(&self.np_cols[s..])
                    .any(|(&x, &c)| x != 0 && self.pivot[c as usize] != NONE)
            }
        }
    }

    /// A monic stored row with every pivot column of its tail eliminated,
    /// in the part's columns.
    fn reduce_tail(&self, cx: &Ctx, row: &RowS, s: &mut Scratch) -> (Row, Red) {
        let tail = self.tail_full(row);
        let mut out: Row = Vec::with_capacity(row.nnz());
        out.push((lead_of(row), 1));
        let red = self.eliminate(cx, &tail, s, &mut out);
        (out, red)
    }
}

fn lead_of(row: &RowS) -> u32 {
    match row {
        RowS::Sparse(r) => r[0].0,
        RowS::Dense { lead, .. } => *lead,
    }
}

pub(super) enum Item {
    Row(u8, Row),
    Prod { part: u8, row: u32, var: u16 },
}

/// What the parallel section returns for one queue item.
struct ItemOut {
    cands: Vec<(u8, Row)>,
    ops: u64,
    nred: u64,
    formed: u32,
    prod_nnz: u64,
    src_deg: u8,
    src_age: u64,
    src_nnz: u64,
    max_task: u64,
    /// Phase-0 multiply-adds; multiply-adds of products that reduced to 0.
    ops0: u64,
    zero_ops: u64,
    /// Tracing only: the epoch pivot rows the products need (part << 32 | row).
    core: Vec<u64>,
    /// Source class (`2 · degree + covered`, 31 for a generator row), and
    /// the covers this item's products give (part, column, κ).
    cls: u8,
    coverable: bool,
    covers: Vec<(u8, u32, u16)>,
}

struct Pool(Vec<Mutex<Scratch>>);

impl Pool {
    fn new(n: usize) -> Self {
        let k = rayon::current_num_threads() + 1;
        Pool(
            (0..k)
                .map(|_| {
                    Mutex::new(Scratch {
                        acc: vec![0u64; n],
                        bm: vec![0u64; n / 64 + 2],
                    })
                })
                .collect(),
        )
    }
    fn with<T>(&self, f: impl FnOnce(&mut Scratch) -> T) -> T {
        let i = rayon::current_thread_index().map_or(self.0.len() - 1, |i| i % (self.0.len() - 1));
        let mut g = self.0[i].lock().unwrap_or_else(|e| e.into_inner());
        f(&mut g)
    }
}

/// A raw pointer that parallel sections share; every use writes a range no
/// other task writes (see [`reduce_products`]).
#[derive(Clone, Copy)]
struct SharedMut(*mut u64);
// SAFETY: the tasks that receive a copy write disjoint index ranges.
unsafe impl Send for SharedMut {}
unsafe impl Sync for SharedMut {}
impl SharedMut {
    #[inline(always)]
    fn ptr(self) -> *mut u64 {
        self.0
    }
}

/// What phase 0 of one product needs, found without touching memory.
#[derive(Clone, Default)]
struct Plan {
    /// First and last epoch index the reduction writes (`first == usize::MAX`:
    /// none).
    first: usize,
    last: usize,
    /// Rows of the epoch's pivots to subtract, with their factors.
    core: Vec<(u32, u64)>,
    ops0: u64,
}

/// The rows `prods` (each in its part's columns) reduced against `ech`, as
/// [`Echelon::reduce`] reduces them one by one, with the same result and the
/// same counts. When the epoch has pivots and the work against their rows is
/// large ([`Fast2Options::sweep_ratio`]), phase 0 is done for the whole
/// batch at once, by tiles of epoch columns: each tile (one task, its own
/// columns of every product's accumulator) reads each needed row once and
/// subtracts it from every product that needs it, so a stored row is read
/// once per batch instead of once per product. The cascade then runs per
/// product. Otherwise every product is reduced alone.
fn reduce_products(
    ech: &[Echelon],
    cx: &Ctx,
    prods: &[(u8, Row)],
    pool: &Pool,
    sweep: &mut Vec<u64>,
    opts: &Fast2Options,
) -> (Vec<(Row, Red)>, bool) {
    let alone = || -> Vec<(Row, Red)> {
        prods
            .par_iter()
            .with_max_len(1)
            .map(|(q, r)| pool.with(|s| ech[*q as usize].reduce(cx, r, s)))
            .collect()
    };
    let n_parts = ech.len();
    let epochs = ech.iter().all(|e| e.np_cols.len() < e.pivot.len());
    if !opts.sweep || prods.is_empty() || !epochs {
        return (alone(), false);
    }
    let p = cx.p;
    // Phase 0, planned: which epoch-pivot rows each product needs.
    let plans: Vec<Plan> = prods
        .par_iter()
        .map(|(q, r)| {
            let e = &ech[*q as usize];
            let mut pl = Plan {
                first: usize::MAX,
                ..Plan::default()
            };
            for &(col, x) in r {
                let j = e.of_col[col as usize];
                if j != NONE {
                    pl.first = pl.first.min(j as usize);
                    pl.last = pl.last.max(j as usize);
                    continue;
                }
                let v = u64::from(x) % p;
                if v == 0 {
                    continue;
                }
                let piv = e.pivot[col as usize];
                assert!(
                    piv != NONE,
                    "hky_fp_tower_fast2: a column is neither in the epoch nor a pivot"
                );
                let row = &e.rows[piv as usize];
                pl.ops0 += row.nnz() as u64;
                pl.core.push((piv, p - v));
                match row {
                    RowS::Sparse(rr) if rr.len() > 1 => {
                        pl.first = pl.first.min(rr[1].0 as usize);
                        pl.last = pl.last.max(rr[rr.len() - 1].0 as usize);
                    }
                    RowS::Dense { v, .. } if v.len() > 1 => {
                        pl.first = pl.first.min(e.np_start(col));
                        pl.last = e.np_cols.len() - 1;
                    }
                    _ => {}
                }
            }
            pl
        })
        .collect();
    // Is the batch worth it: phase-0 work against the accumulators' size.
    let mut off = Vec::with_capacity(prods.len());
    let mut total = 0usize;
    let mut work = 0u64;
    for ((q, _), pl) in prods.iter().zip(&plans) {
        off.push(total);
        total += ech[*q as usize].np_cols.len();
        work += pl.ops0;
    }
    if work < opts.sweep_ratio * total as u64 || total > opts.sweep_max_words {
        return (alone(), false);
    }
    if sweep.len() < total {
        sweep.resize(total, 0);
    }
    let base = SharedMut(sweep.as_mut_ptr());
    // The rows' own epoch terms (each product writes its own range).
    prods.par_iter().zip(&off).for_each(|((q, r), &o)| {
        let e = &ech[*q as usize];
        for &(col, x) in r {
            let j = e.of_col[col as usize];
            if j != NONE {
                // SAFETY: [o, o + m_q) belongs to this product alone.
                unsafe { *base.ptr().add(o + j as usize) = u64::from(x) };
            }
        }
    });
    // Runs: every (row, product, factor) of a part, by row.
    let mut runs: Vec<Vec<(u32, u32, u64)>> = vec![Vec::new(); n_parts];
    for (k, ((q, _), pl)) in prods.iter().zip(&plans).enumerate() {
        for &(row, f) in &pl.core {
            runs[*q as usize].push((row, k as u32, f));
        }
    }
    for r in runs.iter_mut() {
        r.par_sort_unstable_by_key(|&(row, k, _)| (row, k));
    }
    // Per needed row: its users (a range of `runs`) and the epoch indices
    // its tail covers, so a tile skips the rows that miss it cheaply.
    let meta: Vec<Vec<(u32, u32, u32, u32, u32)>> = runs
        .iter()
        .zip(ech)
        .map(|(rs, e)| {
            let mut starts: Vec<u32> = Vec::new();
            for i in 0..rs.len() {
                if i == 0 || rs[i].0 != rs[i - 1].0 {
                    starts.push(i as u32);
                }
            }
            starts.push(rs.len() as u32);
            starts
                .par_windows(2)
                .filter_map(|w| {
                    let row = rs[w[0] as usize].0;
                    let (a, b) = match &e.rows[row as usize] {
                        RowS::Sparse(rr) if rr.len() > 1 => (rr[1].0, rr[rr.len() - 1].0),
                        RowS::Dense { lead, v, .. } if v.len() > 1 => {
                            (e.np_start(*lead) as u32, (e.np_cols.len() - 1) as u32)
                        }
                        _ => return None,
                    };
                    Some((row, w[0], w[1], a, b))
                })
                .collect()
        })
        .collect();
    // Tiles of epoch columns: one task each, across both parts.
    let nprod_q: Vec<usize> = (0..n_parts)
        .map(|q| {
            prods
                .iter()
                .filter(|(pq, _)| *pq as usize == q)
                .count()
                .max(1)
        })
        .collect();
    // Precompute per-part product list so dense tiles can execute AVX-512 GEMM.
    let mut prods_in_part: Vec<Vec<usize>> = vec![Vec::new(); n_parts];
    let mut prod_local = vec![0usize; prods.len()];
    for (k, (q, _)) in prods.iter().enumerate() {
        let pq = *q as usize;
        prod_local[k] = prods_in_part[pq].len();
        prods_in_part[pq].push(k);
    }
    let mut tiles: Vec<(usize, usize, usize)> = Vec::new();
    for (q, e) in ech.iter().enumerate() {
        if runs[q].is_empty() {
            continue;
        }
        let m = e.np_cols.len();
        let w = if cx.use_gemm {
            let nr = fp_gemm::nr_for(cx.gemm_simd);
            let raw = (opts.sweep_tile_bytes / (8 * nprod_q[q]))
                .min(m.div_ceil(4 * rayon::current_num_threads()))
                .clamp(96, m.max(96));
            (raw.div_ceil(nr) * nr).min(m.max(nr))
        } else {
            (opts.sweep_tile_bytes / (8 * nprod_q[q]))
                .min(m.div_ceil(4 * rayon::current_num_threads()))
                .clamp(64, m.max(64))
        };
        let mut j0 = 0;
        while j0 < m {
            let j1 = (j0 + w).min(m);
            tiles.push((q, j0, j1));
            j0 = j1;
        }
    }
    let p2 = cx.p2;
    let gemm_simd = cx.gemm_simd;
    let use_gemm = cx.use_gemm;
    let k_safe = fp_gemm::k_safe_f64(cx.p);
    let kc_max = 256usize.min(k_safe.max(1));
    tiles.par_iter().with_max_len(1).for_each(|&(q, j0, j1)| {
        if opts.fault == 5 && j0 == 0 {
            return;
        }
        let e = &ech[q];
        let rs = &runs[q];
        let mut dense_rows: Vec<(u32, u32, u32, usize)> = Vec::new();
        for &(row_i, i0, i1, a, b) in &meta[q] {
            if (a as usize) >= j1 || (b as usize) < j0 {
                continue;
            }
            match &e.rows[row_i as usize] {
                RowS::Sparse(rr) => {
                    let users = &rs[i0 as usize..i1 as usize];
                    let tail = &rr[1..];
                    if tail.is_empty()
                        || (tail[0].0 as usize) >= j1
                        || (tail[tail.len() - 1].0 as usize) < j0
                    {
                        continue;
                    }
                    let lo = tail.partition_point(|t| (t.0 as usize) < j0);
                    let hi = tail.partition_point(|t| (t.0 as usize) < j1);
                    let seg = &tail[lo..hi];
                    for &(_, k, f) in users {
                        let o = off[k as usize];
                        for &(j, x) in seg {
                            // SAFETY: column j of product k is in this tile,
                            // which only this task writes.
                            unsafe {
                                let a = base.ptr().add(o + j as usize);
                                let t = *a + f * u64::from(x);
                                *a = t.min(t.wrapping_sub(p2));
                            }
                        }
                    }
                }
                RowS::Dense { lead, v, .. } => {
                    if v.len() <= 1 {
                        continue;
                    }
                    let s = e.np_start(*lead);
                    if s < j1 {
                        dense_rows.push((row_i, i0, i1, s));
                    }
                }
            }
        }
        if dense_rows.is_empty() {
            return;
        }
        let b_q = prods_in_part[q].len();
        if use_gemm && b_q >= 1 && dense_rows.len() >= 2 {
            let w = j1 - j0;
            let ldb = w.div_ceil(8) * 8;
            let mut c_tile = vec![0.0f64; b_q * w];
            let mut b_tile: Vec<f64> = Vec::new();
            let mut a_sub: Vec<Vec<(u32, f64)>> = vec![Vec::new(); b_q];
            let mut a_dense_buf: Vec<f64> = Vec::new();
            let mut b_pack_buf: Vec<f64> = Vec::new();
            let mut k_acc = 0usize;
            for chunk in dense_rows.chunks(kc_max) {
                let kc = chunk.len();
                if k_acc + kc > k_safe {
                    fp_gemm::reduce_f64_slice_mod_p(&mut c_tile, cx.p, cx.barrett_m);
                    k_acc = 0;
                }
                b_tile.clear();
                b_tile.resize(kc * ldb, 0.0);
                for a_r in a_sub.iter_mut() {
                    a_r.clear();
                }
                for (kk, &(row_i, i0, i1, s)) in chunk.iter().enumerate() {
                    if let RowS::Dense { v, .. } = &e.rows[row_i as usize] {
                        let a_col = s.max(j0);
                        let src = &v[1 + a_col - s..1 + j1 - s];
                        let dst = &mut b_tile[kk * ldb + (a_col - j0)..][..src.len()];
                        fp_gemm::cvt_u32_to_f64(gemm_simd, src, dst);
                    }
                    for &(_, k, f) in &rs[i0 as usize..i1 as usize] {
                        a_sub[prod_local[k as usize]].push((kk as u32, f as f64));
                    }
                }
                fp_gemm::spmm_or_gemm_f64(
                    gemm_simd,
                    b_q,
                    w,
                    kc,
                    &a_sub,
                    &b_tile,
                    ldb,
                    &mut c_tile,
                    w,
                    &mut a_dense_buf,
                    &mut b_pack_buf,
                );
                k_acc += kc;
            }
            for (li, &k) in prods_in_part[q].iter().enumerate() {
                let o = off[k];
                // SAFETY: [o + j0, o + j1) is this tile's columns of product k.
                let dst = unsafe { std::slice::from_raw_parts_mut(base.ptr().add(o + j0), w) };
                fp_gemm::add_f64_reduced_into_u64(
                    dst,
                    &c_tile[li * w..(li + 1) * w],
                    cx.p,
                    cx.barrett_m,
                    p2,
                );
            }
        } else {
            for &(row_i, i0, i1, s) in &dense_rows {
                if let RowS::Dense { v, .. } = &e.rows[row_i as usize] {
                    let a = s.max(j0);
                    let src = &v[1 + a - s..1 + j1 - s];
                    for &(_, k, f) in &rs[i0 as usize..i1 as usize] {
                        let o = off[k as usize];
                        let dst = unsafe {
                            std::slice::from_raw_parts_mut(base.ptr().add(o + a), j1 - a)
                        };
                        axpy_dense(cx.simd, dst, src, f, p2);
                    }
                }
            }
        }
    });
    // Optional blocked cascade: classical scan on each column tile (exact), then
    // AVX-512 GEMM of that tile's pivot tails onto later columns for all products
    // at once. Gate with HKY3_CASC_GEMM=1; default remains the classical scan.
    // Opt-in blocked cascade GEMM (HKY3_CASC_GEMM=1). Correct but currently
    // slower than the classical per-product scan on P1-scale batches; kept for
    // further tuning. Default remains the classical scan.
    let use_casc_gemm = cx.use_gemm
        && prods.len() >= 32
        && matches!(
            std::env::var("HKY3_CASC_GEMM").ok().as_deref(),
            Some("1") | Some("on") | Some("true")
        );
    let done: Vec<(Row, Red)> = if use_casc_gemm {
        cascade_blocked_gemm(ech, cx, prods, &plans, &off, base.ptr(), n_parts, p, p2)
    } else {
        prods
            .par_iter()
            .with_max_len(1)
            .zip(plans.par_iter())
            .zip(off.par_iter())
            .map(|(((q, _), pl), &o)| {
                let e = &ech[*q as usize];
                let m = e.np_cols.len();
                let mut red = Red {
                    ops: pl.ops0,
                    nred: pl.core.len() as u32,
                    ops0: pl.ops0,
                };
                let mut out: Row = Vec::new();
                if pl.first != usize::MAX {
                    // SAFETY: [o, o + m) belongs to this product alone.
                    let acc = unsafe { std::slice::from_raw_parts_mut(base.ptr().add(o), m) };
                    e.scan(cx, acc, pl.first, pl.last, &mut red, &mut out);
                }
                red.ops += normalise(&mut out, p);
                (out, red)
            })
            .collect()
    };
    (done, true)
}

/// Left-looking blocked cascade with a persistent f64 panel: classical in-tile
/// decisions, AVX-512 GEMM trailing updates, one u64↔f64 conversion per part.
fn cascade_blocked_gemm(
    ech: &[Echelon],
    cx: &Ctx,
    prods: &[(u8, Row)],
    plans: &[Plan],
    off: &[usize],
    base_ptr: *mut u64,
    n_parts: usize,
    p: u64,
    p2: u64,
) -> Vec<(Row, Red)> {
    let _ = (base_ptr, p2); // panel owns the data after conversion
    let bm_ = cx.barrett_m;
    let simd = cx.gemm_simd;
    let nr = fp_gemm::nr_for(simd);
    let k_safe = fp_gemm::k_safe_f64(p).max(1);
    let kc_max = 64usize.min(k_safe);
    let nc = (512usize.div_ceil(nr) * nr).max(nr);
    let mut out_rows: Vec<Row> = vec![Vec::new(); prods.len()];
    let mut reds: Vec<Red> = plans
        .iter()
        .map(|pl| Red {
            ops: pl.ops0,
            nred: pl.core.len() as u32,
            ops0: pl.ops0,
        })
        .collect();
    let mut lasts: Vec<usize> = plans.iter().map(|pl| pl.last).collect();

    for q in 0..n_parts {
        let idxs: Vec<usize> = (0..prods.len())
            .filter(|&k| prods[k].0 as usize == q)
            .collect();
        if idxs.is_empty() {
            continue;
        }
        let e = &ech[q];
        let m = e.np_cols.len();
        let nprod = idxs.len();
        let (mut g_first, mut g_last) = (m, 0usize);
        for &k in &idxs {
            if plans[k].first != usize::MAX {
                g_first = g_first.min(plans[k].first);
                g_last = g_last.max(lasts[k]);
            }
        }
        if g_first > g_last {
            continue;
        }
        // Persistent f64 panel for [g_first, m); convert once from the u64 sweep.
        let width = m - g_first;
        let mut panel = vec![0.0f64; nprod * width];
        for (li, &k) in idxs.iter().enumerate() {
            if plans[k].first == usize::MAX {
                continue;
            }
            let o = off[k];
            let acc = unsafe { std::slice::from_raw_parts(base_ptr.add(o + g_first), width) };
            for j in 0..width {
                let raw = acc[j];
                if raw != 0 {
                    panel[li * width + j] = raw as f64;
                }
            }
            unsafe { std::slice::from_raw_parts_mut(base_ptr.add(o + g_first), width).fill(0) };
        }

        let mut j0 = g_first;
        while j0 <= g_last {
            let j1 = (j0 + nc).min(g_last + 1);
            // Parallel in-tile scans (products independent). Factors keyed by
            // absolute pivot-row index; remapped to a dense K list afterwards.
            let j1_local = j1 - g_first;
            let panel_ptr = panel.as_mut_ptr() as usize;
            let tile_outs: Vec<(usize, Vec<(u32, u32)>, Vec<(usize, f64)>, usize, u32, u64)> = (0
                ..nprod)
                .into_par_iter()
                .map(|li| {
                    let k = idxs[li];
                    let mut emits: Vec<(u32, u32)> = Vec::new();
                    let mut fac: Vec<(usize, f64)> = Vec::new();
                    let mut nred = 0u32;
                    let mut ops = 0u64;
                    if plans[k].first == usize::MAX || lasts[k] < j0 {
                        return (li, emits, fac, lasts[k], nred, ops);
                    }
                    // SAFETY: each li owns a disjoint panel row.
                    let row_p = unsafe {
                        std::slice::from_raw_parts_mut(
                            (panel_ptr as *mut f64).add(li * width),
                            width,
                        )
                    };
                    let mut ci = j0 - g_first;
                    let mut last_local = lasts[k] - g_first;
                    while ci < j1_local && ci <= last_local {
                        let mut raw = row_p[ci];
                        if raw != 0.0 {
                            let mut tmp = [raw];
                            fp_gemm::reduce_f64_slice_mod_p(&mut tmp, p, bm_);
                            raw = tmp[0];
                            row_p[ci] = 0.0;
                        }
                        if raw == 0.0 {
                            ci += 1;
                            continue;
                        }
                        let v = raw as u64;
                        let col = e.np_cols[g_first + ci];
                        let piv = e.pivot[col as usize];
                        if piv == NONE {
                            emits.push((col, v as u32));
                            ci += 1;
                            continue;
                        }
                        let brow = &e.rows[piv as usize];
                        nred += 1;
                        ops += brow.nnz() as u64;
                        let f = (p - v) as f64;
                        match brow {
                            RowS::Sparse(rr) => {
                                for &(jj, x) in &rr[1..] {
                                    let ju = jj as usize;
                                    if ju >= j1 {
                                        break;
                                    }
                                    if ju > g_first + ci {
                                        row_p[ju - g_first] += f * (x as f64);
                                    }
                                }
                                if rr.len() > 1 {
                                    last_local =
                                        last_local.max(rr[rr.len() - 1].0 as usize - g_first);
                                }
                            }
                            RowS::Dense { lead, v: vv, .. } => {
                                let s = e.np_start(*lead);
                                let a = (g_first + ci + 1).max(s);
                                if a < j1 && vv.len() > 1 {
                                    let src = &vv[1 + a - s..1 + j1 - s];
                                    // SIMD f64 axpy when the in-tile span is long.
                                    if src.len() >= 16 {
                                        let mut src_f = vec![0.0f64; src.len()];
                                        fp_gemm::cvt_u32_to_f64(simd, src, &mut src_f);
                                        fp_gemm::axpy_f64(
                                            simd,
                                            &mut row_p[a - g_first..a - g_first + src.len()],
                                            &src_f,
                                            f,
                                        );
                                    } else {
                                        for (off_j, &x) in src.iter().enumerate() {
                                            row_p[a - g_first + off_j] += f * (x as f64);
                                        }
                                    }
                                }
                                last_local = width - 1;
                            }
                        }
                        let extends = match brow {
                            RowS::Sparse(rr) => rr.len() > 1 && (rr[rr.len() - 1].0 as usize) >= j1,
                            RowS::Dense { lead, v: vv, .. } => {
                                vv.len() > 1 && e.np_start(*lead) + vv.len() - 1 > j1
                            }
                        };
                        if extends {
                            fac.push((piv as usize, f));
                        }
                        ci += 1;
                    }
                    let lo = (j0 - g_first + 1).min(width);
                    let hi = j1_local.min(width);
                    if lo < hi {
                        fp_gemm::reduce_f64_slice_mod_p(&mut row_p[lo..hi], p, bm_);
                    }
                    (li, emits, fac, g_first + last_local, nred, ops)
                })
                .collect();

            let mut piv_rows: Vec<usize> = Vec::new();
            let mut piv_mark = vec![NONE; e.rows.len()];
            let mut factors: Vec<Vec<(u32, f64)>> = vec![Vec::new(); nprod];
            for (li, emits, fac, last_k, nred, ops) in tile_outs {
                let k = idxs[li];
                out_rows[k].extend(emits);
                reds[k].nred += nred;
                reds[k].ops += ops;
                lasts[k] = lasts[k].max(last_k);
                g_last = g_last.max(lasts[k]);
                for (pri, f) in fac {
                    let pm = &mut piv_mark[pri];
                    if *pm == NONE {
                        *pm = piv_rows.len() as u32;
                        piv_rows.push(pri);
                    }
                    factors[li].push((*pm, f));
                }
            }

            // Trailing GEMM onto panel[:, j1 - g_first ..].
            if !piv_rows.is_empty() && j1 < m {
                let ka = piv_rows.len();
                let mut t0 = j1 - g_first;
                while t0 < width {
                    let tw = (width - t0).min(nc);
                    // C is a view into panel columns [t0, t0+tw).
                    let mut ct = vec![0.0f64; nprod * tw];
                    for li in 0..nprod {
                        ct[li * tw..(li + 1) * tw]
                            .copy_from_slice(&panel[li * width + t0..li * width + t0 + tw]);
                    }
                    let mut k_acc = 0usize;
                    let mut kk0 = 0usize;
                    let mut b_pack = Vec::new();
                    let mut a_dense = Vec::new();
                    let mut a_sub: Vec<Vec<(u32, f64)>> = vec![Vec::new(); nprod];
                    while kk0 < ka {
                        let kc = (ka - kk0).min(kc_max);
                        if k_acc + kc > k_safe {
                            fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm_);
                            k_acc = 0;
                        }
                        let mut nnz = 0usize;
                        for li in 0..nprod {
                            a_sub[li].clear();
                            for &(pm, f) in &factors[li] {
                                let pmu = pm as usize;
                                if pmu >= kk0 && pmu < kk0 + kc {
                                    a_sub[li].push(((pmu - kk0) as u32, f));
                                    nnz += 1;
                                }
                            }
                        }
                        if nnz == 0 {
                            kk0 += kc;
                            continue;
                        }
                        let jj0 = g_first + t0;
                        let jj1 = jj0 + tw;
                        let ldb = tw.div_ceil(8) * 8;
                        let mut b_tile = vec![0.0f64; kc * ldb];
                        for kk in 0..kc {
                            match &e.rows[piv_rows[kk0 + kk]] {
                                RowS::Sparse(rr) => {
                                    for &(jj, x) in &rr[1..] {
                                        let ju = jj as usize;
                                        if ju >= jj1 {
                                            break;
                                        }
                                        if ju >= jj0 {
                                            b_tile[kk * ldb + (ju - jj0)] = x as f64;
                                        }
                                    }
                                }
                                RowS::Dense { lead, v, .. } => {
                                    if v.len() > 1 {
                                        let s = e.np_start(*lead);
                                        let a = s.max(jj0);
                                        if a < jj1 {
                                            let src = &v[1 + a - s..1 + jj1 - s];
                                            fp_gemm::cvt_u32_to_f64(
                                                simd,
                                                src,
                                                &mut b_tile[kk * ldb + (a - jj0)..][..src.len()],
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        let use_dense = nprod >= 6 && kc >= 4 && nnz * 3 >= nprod * kc;
                        if use_dense {
                            a_dense.clear();
                            a_dense.resize(nprod * kc, 0.0);
                            for li in 0..nprod {
                                for &(pk, f) in &a_sub[li] {
                                    a_dense[li * kc + pk as usize] = f;
                                }
                            }
                            fp_gemm::prepare_b_pack(&mut b_pack, kc, tw, nr);
                            for kk in 0..kc {
                                let src = &b_tile[kk * ldb..][..tw];
                                let n_panels = tw.div_ceil(nr);
                                for p_idx in 0..n_panels {
                                    let j_off = p_idx * nr;
                                    let w = (tw - j_off).min(nr);
                                    b_pack[p_idx * kc * nr + kk * nr..][..w]
                                        .copy_from_slice(&src[j_off..j_off + w]);
                                }
                            }
                            fp_gemm::gemm_f64_with_packed_b(
                                simd, nprod, tw, kc, &a_dense, kc, &b_pack, &mut ct, tw,
                            );
                        } else {
                            for li in 0..nprod {
                                if !a_sub[li].is_empty() {
                                    fp_gemm::spmm_row_f64(
                                        simd,
                                        &mut ct[li * tw..(li + 1) * tw],
                                        tw,
                                        &a_sub[li],
                                        &b_tile,
                                        ldb,
                                    );
                                }
                            }
                        }
                        k_acc += kc;
                        kk0 += kc;
                    }
                    fp_gemm::reduce_f64_slice_mod_p(&mut ct, p, bm_);
                    for li in 0..nprod {
                        panel[li * width + t0..li * width + t0 + tw]
                            .copy_from_slice(&ct[li * tw..(li + 1) * tw]);
                    }
                    t0 += tw;
                }
            }
            j0 = j1;
        }
        // Emit remaining non-pivot terms from the panel.
        for (li, &k) in idxs.iter().enumerate() {
            if plans[k].first == usize::MAX {
                continue;
            }
            let row_p = &mut panel[li * width..(li + 1) * width];
            fp_gemm::reduce_f64_slice_mod_p(row_p, p, bm_);
            for j in 0..width {
                let v = row_p[j];
                if v == 0.0 {
                    continue;
                }
                let col = e.np_cols[g_first + j];
                if e.pivot[col as usize] == NONE {
                    out_rows[k].push((col, v as u32));
                } else {
                    // Leftover pivot: apply classically into a scratch u64 span.
                    let mut scratch = vec![0u64; m];
                    for jj in 0..width {
                        scratch[g_first + jj] = row_p[jj] as u64;
                        row_p[jj] = 0.0;
                    }
                    let piv = e.pivot[col as usize];
                    let brow = &e.rows[piv as usize];
                    reds[k].nred += 1;
                    reds[k].ops += brow.nnz() as u64;
                    let mut last = m - 1;
                    let mut c2 = g_first + j;
                    let v_u = v as u64;
                    scratch[c2] = 0;
                    let l = e.apply(cx, brow, p - v_u, &mut scratch, &mut [], false);
                    last = last.max(l);
                    c2 += 1;
                    while c2 <= last {
                        let raw2 = scratch[c2];
                        if raw2 == 0 {
                            c2 += 1;
                            continue;
                        }
                        scratch[c2] = 0;
                        let v2 = barrett(raw2, p, bm_);
                        if v2 == 0 {
                            c2 += 1;
                            continue;
                        }
                        let col2 = e.np_cols[c2];
                        let piv2 = e.pivot[col2 as usize];
                        if piv2 == NONE {
                            out_rows[k].push((col2, v2 as u32));
                            c2 += 1;
                            continue;
                        }
                        let brow2 = &e.rows[piv2 as usize];
                        reds[k].nred += 1;
                        reds[k].ops += brow2.nnz() as u64;
                        let l = e.apply(cx, brow2, p - v2, &mut scratch, &mut [], false);
                        last = last.max(l);
                        c2 += 1;
                    }
                    break;
                }
            }
            reds[k].ops += normalise(&mut out_rows[k], p);
        }
    }
    out_rows.into_iter().zip(reds).collect()
}

/// The closedness check of [`Fast2Options::verify_closed`]. `W` is the span
/// of the final echelon (part by part). Every generator of degree at most
/// `c` and every product `x_v · g`, for **every** variable `v` (not only one
/// of each pair the symmetry swaps) and every final basis row `g` whose
/// leading column has degree below `c`, is reduced against `W` and must be
/// zero. The rows of degree below `c` span `W ∩ R_{<c}` (the columns are
/// sorted by degree, highest first), so this is the definition of closed.
/// The echelon is first copied into the part's own columns (one epoch of
/// every column), so the check uses neither the epoch coordinates nor the
/// sweep of the run it checks. Returns (reductions, how many were not 0).
fn verify_closed(
    space: &Space,
    ech: &[Echelon],
    cx: &Ctx,
    c: u32,
    nv: usize,
    gens: &[(u8, Row)],
    pool: &Pool,
) -> (u64, u64) {
    let full: Vec<Echelon> = ech
        .iter()
        .map(|e| {
            let rows: Vec<RowS> = (0..e.rows.len())
                .into_par_iter()
                .map(|ri| RowS::Sparse(e.row_full(ri)))
                .collect();
            let mut f = Echelon::new(e.pivot.len());
            f.pivot = e.pivot.clone();
            f.rows = rows;
            f
        })
        .collect();
    let cx = Ctx {
        bitmap: false,
        ..*cx
    };
    let check = |q: u8, r: &Row| -> u64 {
        let (red, _) = pool.with(|s| full[q as usize].reduce(&cx, r, s));
        u64::from(!red.is_empty())
    };
    let mut done = gens.len() as u64;
    let mut bad: u64 = gens.iter().map(|(q, r)| check(*q, r)).sum();
    let jobs: Vec<(u8, u32)> = full
        .iter()
        .enumerate()
        .flat_map(|(q, e)| {
            (0..e.rows.len() as u32)
                .filter(move |&ri| {
                    u32::from(space.parts[q].deg[lead_of(&e.rows[ri as usize]) as usize]) < c
                })
                .map(move |ri| (q as u8, ri))
        })
        .collect();
    let (n, b) = jobs
        .par_iter()
        .with_max_len(1)
        .map(|&(q, ri)| {
            let src = &full[q as usize].rows[ri as usize];
            let (mut n, mut b) = (0u64, 0u64);
            let mut formed: Vec<(u8, Row)> = Vec::new();
            for v in 0..nv {
                formed.clear();
                space.multiply(q as usize, src, v, &mut formed);
                for (q2, r) in &formed {
                    n += 1;
                    b += check(*q2, r);
                }
            }
            (n, b)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    done += n;
    bad += b;
    (done, bad)
}

/// `V_{F,c}` with the instrument of `opts`.
pub fn closure_fast2(
    ring: &HkyRing,
    gens: &[NfPoly],
    c: u32,
    limits: &HkyLimits,
    opts: &Fast2Options,
) -> (Closure, Fast2Stats) {
    let started = Instant::now();
    let p = ring.p;
    assert!(p < 1 << 31, "hky_fp_tower_fast2: p < 2^31");
    let n_cols = count_monomials(ring.n_tower(), ring.n_free, c);
    let mut stats = Fast2Stats {
        symmetry: "off".into(),
        threads: rayon::current_num_threads(),
        skip_on: opts.skip,
        ..Fast2Stats::default()
    };
    let censored = |why: &str, rows: u64, entries: u64, stats: Fast2Stats| {
        (
            Closure {
                c,
                cols: n_cols,
                dims: Vec::new(),
                contains_one: false,
                censored: Some(why.to_string()),
                rows_reduced: rows,
                entries,
                muladds: stats.muladds,
                ms: started.elapsed().as_secs_f64() * 1e3,
            },
            stats,
        )
    };
    if n_cols > limits.max_cols {
        return censored("max_cols", 0, 0, stats);
    }
    let live: Vec<NfPoly> = gens.iter().filter(|g| !g.is_empty()).cloned().collect();
    let (perm, signs) = match &opts.symmetry {
        None => (None, vec![1i8; live.len()]),
        Some(perm) => match check_symmetry(ring, perm, &live) {
            Ok(signs) => {
                stats.symmetry = "split".into();
                (Some(perm.clone()), signs)
            }
            Err(why) => {
                stats.symmetry = format!("rejected: {why}");
                (None, vec![1i8; live.len()])
            }
        },
    };
    let space = Space::build(ring, c, perm, true);
    if !space.split() {
        let total: usize = space.parts.iter().map(|q| q.keys.len()).sum();
        assert_eq!(total as u128, n_cols, "hky_fp_tower_fast2: column count");
    }
    stats.part_cols = space.parts.iter().map(|q| q.keys.len() as u64).collect();
    let simd = detect_simd();
    stats.simd = match simd {
        Simd::Portable => "portable",
        Simd::Avx2 => "avx2",
        Simd::Avx512 => "avx512",
    };
    let cx = Ctx {
        p,
        p2: if opts.fault == 2 { p * p - 1 } else { p * p },
        barrett_m: (u128::from(u64::MAX) / u128::from(p)) as u64,
        simd,
        gemm_simd: fp_gemm::detect_gemm_simd(),
        use_gemm: opts.fault == 0
            && fp_gemm::supports_f64_gemm(p)
            && std::env::var_os("HKY3_NO_GEMM").is_none(),
        bitmap: opts.bitmap,
    };
    let nv = ring.n_vars();
    let applies = |v: usize| space.applies(v) && !(opts.fault == 1 && v == 0);
    let max_n = space.parts.iter().map(|q| q.keys.len()).max().unwrap_or(0);
    let one_col = (space.parts[0].keys.len() - 1) as u32;
    let mut ech: Vec<Echelon> = space
        .parts
        .iter()
        .map(|q| Echelon::new(q.keys.len()))
        .collect();
    let n_buckets: usize = 66 + opts.deg_weight * (c as usize + 1);
    let bucket_of = |nnz: usize, d: u32| {
        (sparse_bucket(nnz, 1) + opts.deg_weight * d as usize).min(n_buckets - 1)
    };
    let mut queue: Vec<Vec<Item>> = (0..n_buckets).map(|_| Vec::new()).collect();
    for (g, &sign) in live.iter().zip(&signs) {
        if HkyRing::degree(g) <= c {
            let (q, r) = space.gen_row(g, sign);
            if !r.is_empty() {
                queue[0].push(Item::Row(q, r));
            }
        }
    }
    let pool = Pool::new(max_n);
    // PROOF.md: the order κ of the variables whose products are formed
    let kappa: Vec<u16> = {
        let reps: Vec<usize> = (0..nv).filter(|&v| applies(v)).collect();
        let mut k = vec![NO_COVER; nv];
        for (i, &v) in reps.iter().enumerate() {
            k[v] = if opts.skip_order == 0 {
                i
            } else {
                reps.len() - 1 - i
            } as u16;
        }
        k
    };
    let mut cover: Vec<Vec<u16>> = space
        .parts
        .iter()
        .map(|q| vec![NO_COVER; q.keys.len()])
        .collect();
    // one witness space per part and degree (the vectors are homogeneous;
    // spaces of different degrees are independent), filled on demand
    let mut witness: Vec<Vec<Witness>> = space
        .parts
        .iter()
        .map(|_| (0..=c).map(|_| Witness::new(0)).collect())
        .collect();
    stats.ops_by_src = vec![0; 32];
    stats.prods_by_src = vec![0; 32];
    let mut sweep_buf: Vec<u64> = Vec::new();
    let mut trace = opts.trace.as_ref().map(|path| {
        std::io::BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap_or_else(|e| panic!("hky3 trace {path}: {e}")),
        )
    });
    let tracing = trace.is_some();
    let mut rows_reduced = 0u64;
    let mut contains_one = false;
    // Running pivot yield, in 1/65536ths (integer arithmetic: deterministic).
    let mut yield_q16: u64 = 65536;
    let mut fresh_since_ir = 0usize;
    let mut rank_at_ir = 0usize;
    stats.setup_s = started.elapsed().as_secs_f64();
    let dense = (opts.dense_num, opts.dense_den);
    // ── io hook 1 of 3 (hky_closure_io): resume from a checkpoint ──────
    let io_fp =
        crate::cryptanalysis::hky_closure_io::fingerprint(ring, gens, c, opts, &stats.part_cols);
    let mut io_clock = crate::cryptanalysis::hky_closure_io::Clock::new(&opts.io);
    let mut elapsed_before_s = 0.0f64;
    let mut io_report = crate::cryptanalysis::hky_closure_io::IoReport::default();
    if let Some(st) =
        crate::cryptanalysis::hky_closure_io::try_resume(&opts.io, &io_fp, &space, opts.skip)
    {
        (ech, queue, rows_reduced, contains_one) =
            (st.ech, st.queue, st.rows_reduced, st.contains_one);
        if opts.skip {
            cover = st.cover;
        }
        (yield_q16, fresh_since_ir, rank_at_ir) = (st.yield_q16, st.fresh_since_ir, st.rank_at_ir);
        // counters from the checkpoint; this host's facts from this process
        let here = std::mem::take(&mut stats);
        stats = Fast2Stats {
            setup_s: here.setup_s,
            symmetry: here.symmetry,
            part_cols: here.part_cols,
            threads: here.threads,
            simd: here.simd,
            skip_on: here.skip_on,
            ops_by_src: here.ops_by_src,
            prods_by_src: here.prods_by_src,
            ..st.stats
        };
        elapsed_before_s = st.elapsed_before_s;
        io_report = stats.io.clone();
        io_report.resumed_from = opts.io.checkpoint_dir.clone();
        io_report.resumes += 1;
    }
    // ── end io hook 1 ──────────────────────────────────────────────────
    'outer: while let Some(b) = queue.iter().position(|q| !q.is_empty()) {
        if crate::cryptanalysis::hky_closure_io::stop_requested() {
            // a checkpoint first (if configured), then a censored result
            let st = ClosureState {
                ech,
                queue,
                rows_reduced,
                contains_one,
                yield_q16,
                fresh_since_ir,
                rank_at_ir,
                stats: Fast2Stats {
                    io: io_report.clone(),
                    ..stats.clone()
                },
                elapsed_before_s: elapsed_before_s + started.elapsed().as_secs_f64(),
                cover: if opts.skip { cover } else { Vec::new() },
            };
            crate::cryptanalysis::hky_closure_io::checkpoint_now(
                &opts.io,
                &io_fp,
                &st,
                &mut io_report,
            );
            let entries: u64 = st
                .ech
                .iter()
                .flat_map(|e| &e.rows)
                .map(|r| r.nnz() as u64)
                .sum();
            stats.io = io_report;
            return censored("signal", st.rows_reduced, entries, stats);
        }
        if started.elapsed() > limits.budget {
            let entries: u64 = ech
                .iter()
                .flat_map(|e| &e.rows)
                .map(|r| r.nnz() as u64)
                .sum();
            stats.cover_hist = cover_hist(&cover, &space, c, &kappa);
            stats.part_rank = ech.iter().map(|e| e.rows.len() as u64).collect();
            stats.rank_by_deg = rank_by_deg(&ech, &space, c);
            return censored("budget", rows_reduced, entries, stats);
        }
        let want = ((opts.target_pivots as u64 * 65536).div_ceil(yield_q16.max(1)) as usize).clamp(
            opts.batch_min.max(1),
            opts.batch_max.max(opts.batch_min).max(1),
        );
        let batch: Vec<Item> = if opts.skip {
            // PROOF.md: a product x_b · r whose row's leading column is
            // covered with κ(a) < κ(b) is not formed
            let mut got: Vec<Item> = Vec::with_capacity(want);
            while got.len() < want {
                let Some(it) = queue[b].pop() else { break };
                if let Item::Prod { part, row, var } = &it {
                    let ld = lead_of(&ech[*part as usize].rows[*row as usize]);
                    if covered(&cover, *part, ld, kappa[*var as usize], opts.fault) {
                        stats.skipped_items += 1;
                        continue;
                    }
                }
                got.push(it);
            }
            got.reverse();
            got
        } else {
            let take = queue[b].len().min(want);
            let at = queue[b].len() - take;
            queue[b].drain(at..).collect()
        };
        if batch.is_empty() {
            continue 'outer;
        }
        let take = batch.len();
        stats.batches += 1;
        let t0 = Instant::now();
        let ech_ref = &ech;
        let cov = &cover;
        let kappa = &kappa;
        let formed_items: Vec<(ItemOut, Vec<(u8, Row)>)> = batch
            .par_iter()
            .with_max_len(1)
            .map(|item| {
                let mut o = ItemOut {
                    cands: Vec::new(),
                    ops: 0,
                    nred: 0,
                    formed: 0,
                    prod_nnz: 0,
                    src_deg: 0,
                    src_age: 0,
                    src_nnz: 0,
                    max_task: 0,
                    ops0: 0,
                    zero_ops: 0,
                    core: Vec::new(),
                    cls: 31,
                    coverable: false,
                    covers: Vec::new(),
                };
                let mut formed: Vec<(u8, Row)> = Vec::new();
                match item {
                    Item::Row(q, r) => formed.push((*q, r.clone())),
                    Item::Prod { part, row, var } => {
                        let e = &ech_ref[*part as usize];
                        let src = &e.rows[*row as usize];
                        // products are formed in the part's columns
                        let full = RowS::Sparse(e.row_full(*row as usize));
                        o.ops += space.multiply(*part as usize, &full, *var as usize, &mut formed);
                        // PROOF.md: covers given by this product, and whether
                        // the criterion covers it (counted when formed)
                        let ld = lead_of(src);
                        let ds = u32::from(space.parts[*part as usize].deg[ld as usize]);
                        let ka = kappa[*var as usize];
                        o.coverable = covered(cov, *part, ld, ka, opts.fault);
                        o.cls = (2 * ds.min(14) as u8
                            + u8::from(covered(cov, *part, ld, NO_COVER, 0)))
                        .min(30);
                        {
                            for (q, r) in &formed {
                                if let Some(&(pl, _)) = r.first() {
                                    let dp = u32::from(space.parts[*q as usize].deg[pl as usize]);
                                    if (dp == ds + 1 && ds + 2 <= c) || opts.fault == 7 {
                                        o.covers.push((*q, pl, ka));
                                    }
                                }
                            }
                        }
                        if tracing {
                            o.src_deg = space.parts[*part as usize].deg[lead_of(src) as usize];
                            o.src_age = (e.rows.len() - *row as usize) as u64;
                            o.src_nnz = src.nnz() as u64;
                        }
                    }
                }
                o.formed = formed.len() as u32;
                if tracing {
                    for (q, r) in &formed {
                        let e = &ech_ref[*q as usize];
                        for &(col, _) in r {
                            if e.of_col[col as usize] == NONE {
                                o.core
                                    .push(u64::from(*q) << 32 | u64::from(e.pivot[col as usize]));
                            }
                        }
                    }
                }
                (o, formed)
            })
            .collect();
        // Reduce every product of the batch (alone, or by the sweep).
        let mut prods: Vec<(u8, Row)> = Vec::new();
        let mut items: Vec<ItemOut> = Vec::with_capacity(formed_items.len());
        for (o, formed) in formed_items {
            prods.extend(formed);
            items.push(o);
        }
        let (done, swept) = reduce_products(&ech, &cx, &prods, &pool, &mut sweep_buf, opts);
        stats.swept_batches += u64::from(swept);
        let mut done = done.into_iter().zip(prods);
        let outs: Vec<ItemOut> = items
            .into_iter()
            .map(|mut o| {
                for ((red, rd), (q, r)) in done.by_ref().take(o.formed as usize) {
                    o.prod_nnz += r.len() as u64;
                    o.ops += rd.ops;
                    o.max_task = o.max_task.max(rd.ops);
                    o.nred += u64::from(rd.nred);
                    o.ops0 += rd.ops0;
                    if red.is_empty() {
                        o.zero_ops += rd.ops;
                    } else {
                        o.cands.push((q, red));
                    }
                }
                o
            })
            .collect();
        let par_dt = t0.elapsed().as_secs_f64();
        stats.parallel_s += par_dt;
        let t1 = Instant::now();
        rows_reduced += take as u64;
        // per-batch counts
        let mut b_ops = 0u64;
        let mut b_max = 0u64;
        let mut b_nred = 0u64;
        let mut b_formed = 0u64;
        let mut b_nnz = 0u64;
        let mut b_age = 0u64;
        let mut b_src = 0u64;
        let mut b_ops0 = 0u64;
        let mut b_zero_ops = 0u64;
        // tracing: epoch pivot rows needed, summed and distinct, per batch and per 8 items
        let (mut core_sum, mut core_union, mut core_union8) = (0u64, 0u64, 0u64);
        if tracing {
            let mut all: Vec<u64> = Vec::new();
            for blk in outs.chunks(8) {
                let mut v: Vec<u64> = blk.iter().flat_map(|o| o.core.iter().copied()).collect();
                core_sum += v.len() as u64;
                all.extend_from_slice(&v);
                v.sort_unstable();
                v.dedup();
                core_union8 += v.len() as u64;
            }
            all.sort_unstable();
            all.dedup();
            core_union = all.len() as u64;
        }
        let mut b_deg = [0u32; 16];
        let mut cands: Vec<(u8, Row)> = Vec::new();
        for o in outs {
            b_ops0 += o.ops0;
            b_zero_ops += o.zero_ops;
            b_ops += o.ops;
            b_max = b_max.max(o.max_task);
            b_nred += o.nred;
            b_formed += u64::from(o.formed);
            b_nnz += o.prod_nnz;
            b_age += o.src_age;
            b_src += o.src_nnz;
            b_deg[(o.src_deg as usize).min(15)] += 1;
            stats.ops_by_src[o.cls as usize] += o.ops;
            stats.prods_by_src[o.cls as usize] += u64::from(o.formed);
            if o.coverable {
                stats.coverable_items += 1;
                stats.coverable_ops += o.ops;
            }
            for &(q, col, ka) in &o.covers {
                let x = &mut cover[q as usize][col as usize];
                *x = (*x).min(ka);
            }
            cands.extend(o.cands);
        }
        stats.muladds += b_ops;
        stats.products += b_formed;
        stats.reducers += b_nred;
        stats.core_muladds += b_ops0;
        stats.zero_muladds += b_zero_ops;
        let b_cands = cands.len() as u64;
        stats.zero += b_formed - b_cands;
        // Second pass: reduce the survivors against each other and insert.
        let mut inserted: Vec<(u8, u32)> = Vec::new();
        let mut second_ops = 0u64;
        let mut serial = 0.0f64;
        let mut rounds = 0u64;
        if opts.par_second {
            while !cands.is_empty() {
                rounds += 1;
                let ts = Instant::now();
                // winners: the first candidate of each leading column
                let mut losers: Vec<(u8, Row)> = Vec::new();
                let mut winners: Vec<(u8, u32)> = Vec::new();
                for (q, r) in cands.drain(..) {
                    let e = &mut ech[q as usize];
                    let lead = r[0].0 as usize;
                    if e.pivot[lead] == NONE {
                        e.pivot[lead] = e.rows.len() as u32;
                        winners.push((q, e.rows.len() as u32));
                        let row = e.to_epoch(&r);
                        e.rows.push(row);
                    } else {
                        losers.push((q, r));
                    }
                }
                serial += ts.elapsed().as_secs_f64();
                let ech_ref = &ech;
                // winners: clear the other winners' columns from the tail;
                // losers: reduce again. One immutable echelon for both.
                let (wn, ln): (Vec<Option<(Row, Red)>>, Vec<(u8, Row, Red)>) = rayon::join(
                    || {
                        winners
                            .par_iter()
                            .with_max_len(1)
                            .map(|&(q, ri)| {
                                let e = &ech_ref[q as usize];
                                let row = &e.rows[ri as usize];
                                e.tail_meets_pivot(row)
                                    .then(|| pool.with(|s| e.reduce_tail(&cx, row, s)))
                            })
                            .collect()
                    },
                    || {
                        losers
                            .par_iter()
                            .with_max_len(1)
                            .map(|(q, r)| {
                                let (red, rd) =
                                    pool.with(|s| ech_ref[*q as usize].reduce(&cx, r, s));
                                (*q, red, rd)
                            })
                            .collect()
                    },
                );
                let ts = Instant::now();
                for (&(q, ri), w) in winners.iter().zip(wn) {
                    if let Some((r, rd)) = w {
                        second_ops += rd.ops;
                        let e = &mut ech[q as usize];
                        e.rows[ri as usize] = e.to_epoch(&r);
                    }
                    inserted.push((q, ri));
                }
                for (q, r, rd) in ln {
                    second_ops += rd.ops;
                    if !r.is_empty() {
                        cands.push((q, r));
                    }
                }
                serial += ts.elapsed().as_secs_f64();
            }
        } else {
            let ts = Instant::now();
            let mut fresh = vec![false; ech.len()];
            for (q, r) in cands.drain(..) {
                let qi = q as usize;
                let e = &ech[qi];
                let r = if fresh[qi] && r.iter().any(|&(c, _)| e.pivot[c as usize] != NONE) {
                    let (r, rd) = pool.with(|s| e.reduce(&cx, &r, s));
                    second_ops += rd.ops;
                    r
                } else {
                    r
                };
                if r.is_empty() {
                    continue;
                }
                let e = &mut ech[qi];
                e.pivot[r[0].0 as usize] = e.rows.len() as u32;
                inserted.push((q, e.rows.len() as u32));
                let row = e.to_epoch(&r);
                e.rows.push(row);
                fresh[qi] = true;
            }
            serial += ts.elapsed().as_secs_f64();
        }
        // Finalise the inserted rows: storage form, queue, counts.
        let ts = Instant::now();
        // storage forms in parallel (a dense form is as long as the epoch)
        let taken: Vec<RowS> = inserted
            .iter()
            .map(|&(q, ri)| {
                std::mem::replace(
                    &mut ech[q as usize].rows[ri as usize],
                    RowS::Sparse(Vec::new()),
                )
            })
            .collect();
        let ech_ref = &ech;
        let forms: Vec<RowS> = taken
            .into_par_iter()
            .zip(inserted.par_iter())
            .with_max_len(16)
            .map(|(row, &(q, _))| {
                assert!(matches!(row, RowS::Sparse(_)), "rows are inserted sparse");
                ech_ref[q as usize].densify(row, dense.0, dense.1)
            })
            .collect();
        for (&(q, ri), stored) in inserted.iter().zip(forms) {
            let qi = q as usize;
            let lead = lead_of(&stored);
            match stored {
                RowS::Dense { .. } => stats.dense_rows += 1,
                RowS::Sparse(_) => stats.sparse_rows += 1,
            }
            stats.stored_words += stored.words() as u64;
            let d = u32::from(space.parts[qi].deg[lead as usize]);
            if d < c {
                let bucket = bucket_of(stored.nnz(), d);
                for v in 0..nv {
                    if applies(v) {
                        stats.queued_items += 1;
                        queue[bucket].push(Item::Prod {
                            part: q,
                            row: ri,
                            var: v as u16,
                        });
                    }
                }
            }
            ech[qi].rows[ri as usize] = stored;
            if qi == 0 && lead == one_col {
                contains_one = true;
            }
        }
        // PROOF.md, witnesses at insertion: for an inserted row s of degree
        // d − 1 ≤ c − 2 and every variable a, the leading column of x_a · s
        // (each projection) when it has degree d covers that column with
        // κ(a). Only the degree-d part of x_a · s decides it, and that is
        // the degree-d part of x_a · top(s).
        if opts.skip && opts.skip_witness == 1 {
            // (1) leading columns of the single products
            let ech_ref = &ech;
            let found: Vec<(u8, u32, u16)> = inserted
                .par_iter()
                .with_max_len(1)
                .flat_map_iter(|&(q, ri)| {
                    let e = &ech_ref[q as usize];
                    let degs = &space.parts[q as usize].deg;
                    let ds = u32::from(degs[lead_of(&e.rows[ri as usize]) as usize]);
                    let mut out: Vec<(u8, u32, u16)> = Vec::new();
                    if ds + 2 <= c {
                        let top = RowS::Sparse(
                            e.row_full(ri as usize)
                                .into_iter()
                                .filter(|&(col, _)| u32::from(degs[col as usize]) == ds)
                                .collect(),
                        );
                        let mut formed: Vec<(u8, Row)> = Vec::new();
                        for v in 0..nv {
                            if !applies(v) {
                                continue;
                            }
                            formed.clear();
                            space.multiply(q as usize, &top, v, &mut formed);
                            for (q2, r) in &formed {
                                if let Some(&(pl, _)) = r.first() {
                                    let dp = u32::from(space.parts[*q2 as usize].deg[pl as usize]);
                                    if dp == ds + 1 || opts.fault == 7 {
                                        out.push((*q2, pl, kappa[v]));
                                    }
                                }
                            }
                        }
                    }
                    out
                })
                .collect();
            for (q, col, ka) in found {
                let x = &mut cover[q as usize][col as usize];
                *x = (*x).min(ka);
            }
        }
        if opts.skip && opts.skip_witness == 2 {
            let ech_ref = &ech;
            let found: Vec<(u8, u16, Row)> = inserted
                .par_iter()
                .with_max_len(1)
                .flat_map_iter(|&(q, ri)| {
                    let e = &ech_ref[q as usize];
                    let degs = &space.parts[q as usize].deg;
                    let ds = u32::from(degs[lead_of(&e.rows[ri as usize]) as usize]);
                    let mut out: Vec<(u8, u16, Row)> = Vec::new();
                    if ds + 2 <= c {
                        let top: Row = e
                            .row_full(ri as usize)
                            .into_iter()
                            .filter(|&(col, _)| u32::from(degs[col as usize]) == ds)
                            .collect();
                        let top = RowS::Sparse(top);
                        let mut formed: Vec<(u8, Row)> = Vec::new();
                        for v in 0..nv {
                            if !applies(v)
                                || (opts.skip_witness_max > 0 && kappa[v] >= opts.skip_witness_max)
                            {
                                continue;
                            }
                            formed.clear();
                            space.multiply(q as usize, &top, v, &mut formed);
                            for (q2, r) in formed.drain(..) {
                                // the degree-(ds + 1) part of the product
                                let dq = &space.parts[q2 as usize].deg;
                                let hv: Row = if opts.fault == 7 {
                                    r
                                } else {
                                    r.into_iter()
                                        .filter(|&(col, _)| u32::from(dq[col as usize]) == ds + 1)
                                        .collect()
                                };
                                if !hv.is_empty() {
                                    out.push((q2, kappa[v], hv));
                                }
                            }
                        }
                    }
                    out
                })
                .collect();
            let tw = Instant::now();
            // each part's witness space on its own task (insertion order kept)
            let mut tasks: Vec<(usize, usize, &mut Witness, Vec<(u16, &Row)>)> = Vec::new();
            for (qi, ws) in witness.iter_mut().enumerate() {
                for (d, w) in ws.iter_mut().enumerate() {
                    let mine: Vec<(u16, &Row)> = found
                        .iter()
                        .filter(|(q, _, hv)| {
                            *q as usize == qi
                                && usize::from(space.parts[qi].deg[hv[0].0 as usize]) == d
                        })
                        .map(|(_, ka, hv)| (*ka, hv))
                        .collect();
                    if !mine.is_empty() {
                        // smaller labels first: fewer swaps
                        let mut mine = mine;
                        mine.sort_by_key(|t| t.0);
                        if w.pivot.is_empty() {
                            *w = Witness::new(space.parts[qi].keys.len());
                        }
                        tasks.push((qi, d, w, mine));
                    }
                }
            }
            let (seq, mut surv) = (0.0f64, 0u64);
            let results: Vec<(usize, u64, Vec<(u32, u16)>)> = tasks
                .into_iter()
                .map(|(qi, _d, w, mine)| {
                    // by parallel rounds (Witness::insert_batch)
                    let mut set: Vec<(u32, u16)> = Vec::new();
                    let cands: Vec<(u16, Row)> =
                        mine.iter().map(|&(k, hv)| (k, hv.clone())).collect();
                    let (ops, rounds) =
                        w.insert_batch(cands, p, &pool, &mut |col, k| set.push((col, k)));
                    surv += rounds;
                    (qi, ops, set)
                })
                .collect();
            let mut ops = 0u64;
            for (qi, o, set) in results {
                ops += o;
                for (col, k) in set {
                    let x = &mut cover[qi][col as usize];
                    *x = (*x).min(k);
                }
            }
            stats.witness_muladds += ops;
            stats.witness_seq_s += seq;
            stats.witness_survivors += surv;
            stats.witness_s += tw.elapsed().as_secs_f64();
        }
        stats.peak_words = stats.peak_words.max(stats.stored_words);
        stats.muladds += second_ops;
        stats.second_muladds += second_ops;
        stats.rounds += rounds;
        fresh_since_ir += inserted.len();
        // yield: exponential average over batches, weight 1/4
        let y = (inserted.len() as u64 * 65536) / (take as u64).max(1);
        yield_q16 = (3 * yield_q16 + y) / 4;
        serial += ts.elapsed().as_secs_f64();
        stats.serial_s += serial;
        stats.second_s += t1.elapsed().as_secs_f64();
        let rank: usize = ech.iter().map(|e| e.rows.len()).sum();
        if let Some(f) = trace.as_mut() {
            let _ = writeln!(
                f,
                "{{\"ev\":\"batch\",\"c\":{c},\"i\":{},\"bucket\":{b},\"items\":{take},\"formed\":{b_formed},\"prod_nnz\":{b_nnz},\"src_nnz\":{b_src},\"zero\":{},\"cands\":{b_cands},\"pivots\":{},\"rounds\":{rounds},\"ops\":{b_ops},\"ops0\":{b_ops0},\"zero_ops\":{b_zero_ops},\"core_sum\":{core_sum},\"core_union\":{core_union},\"core_union8\":{core_union8},\"max_ops\":{b_max},\"ops2\":{second_ops},\"nred\":{b_nred},\"age\":{b_age},\"deg\":{:?},\"rank\":{rank},\"words\":{},\"par_s\":{par_dt:.6},\"sec_s\":{:.6},\"serial_s\":{serial:.6}}}",
                stats.batches,
                b_formed - b_cands,
                inserted.len(),
                &b_deg[..=(c as usize).min(15)],
                stats.stored_words,
                t1.elapsed().as_secs_f64(),
            );
        }
        if contains_one {
            break 'outer;
        }
        let entries_now: u64 = if stats.stored_words > limits.max_entries {
            ech.iter()
                .flat_map(|e| &e.rows)
                .map(|r| r.nnz() as u64)
                .sum()
        } else {
            0
        };
        if entries_now > limits.max_entries {
            return censored("max_entries", rows_reduced, entries_now, stats);
        }
        // Interreduction.
        if opts.interreduce > 0.0
            && fresh_since_ir >= opts.interreduce_min
            && fresh_since_ir as f64 >= opts.interreduce * rank_at_ir as f64
        {
            let ti = Instant::now();
            let before = stats.stored_words;
            let mut ir_ops = 0u64;
            let mut changed = 0u64;
            let mut words = 0u64;
            let (mut n_dense, mut n_sparse) = (0u64, 0u64);
            for e in ech.iter_mut() {
                if cx.use_gemm && opts.np_epochs {
                    let (ops_e, ch_e) = e.interreduce_and_epoch_gemm(&cx, dense);
                    ir_ops += ops_e;
                    changed += ch_e;
                } else {
                    let mut order: Vec<u32> = (0..e.rows.len() as u32).collect();
                    order
                        .sort_unstable_by_key(|&i| std::cmp::Reverse(lead_of(&e.rows[i as usize])));
                    for chunk in order.chunks(opts.interreduce_block.max(1)) {
                        let e_ref = &*e;
                        let new: Vec<Option<(RowS, Red)>> = chunk
                            .par_iter()
                            .with_max_len(1)
                            .map(|&ri| {
                                let row = &e_ref.rows[ri as usize];
                                e_ref.tail_meets_pivot(row).then(|| {
                                    let (mut r, rd) = pool.with(|s| e_ref.reduce_tail(&cx, row, s));
                                    if opts.fault == 3 && r.len() > 1 {
                                        r.pop();
                                    }
                                    (e_ref.densify(e_ref.to_epoch(&r), dense.0, dense.1), rd)
                                })
                            })
                            .collect();
                        for (&ri, x) in chunk.iter().zip(new) {
                            if let Some((row, rd)) = x {
                                ir_ops += rd.ops;
                                changed += 1;
                                e.rows[ri as usize] = row;
                            }
                        }
                    }
                    if opts.np_epochs {
                        e.new_epoch(dense.0, dense.1, opts.fault);
                    }
                }
                for r in &e.rows {
                    words += r.words() as u64;
                    match r {
                        RowS::Dense { .. } => n_dense += 1,
                        RowS::Sparse(_) => n_sparse += 1,
                    }
                }
            }
            stats.stored_words = words;
            stats.dense_rows = n_dense;
            stats.sparse_rows = n_sparse;
            stats.muladds += ir_ops;
            stats.interreduce_muladds += ir_ops;
            stats.interreductions += 1;
            if opts.rebucket {
                let old: Vec<Vec<Item>> =
                    std::mem::replace(&mut queue, (0..n_buckets).map(|_| Vec::new()).collect());
                for (bi, items) in old.into_iter().enumerate() {
                    for it in items {
                        let nb = match &it {
                            Item::Row(..) => bi,
                            Item::Prod { part, row, .. } => {
                                let r = &ech[*part as usize].rows[*row as usize];
                                bucket_of(
                                    r.nnz(),
                                    u32::from(space.parts[*part as usize].deg[lead_of(r) as usize]),
                                )
                            }
                        };
                        queue[nb].push(it);
                    }
                }
            }
            fresh_since_ir = 0;
            rank_at_ir = rank;
            let dt = ti.elapsed().as_secs_f64();
            stats.interreduce_s += dt;
            if let Some(f) = trace.as_mut() {
                // Shape of the (now fully reduced) echelon: per part, columns,
                // rank, stored terms, and the non-pivot columns after each
                // row's leading column (the most terms a reduced row can have).
                let shape: Vec<String> = ech
                    .iter()
                    .map(|e| {
                        let n_q = e.pivot.len();
                        let mut np_after = vec![0u32; n_q + 1];
                        for i in (0..n_q).rev() {
                            np_after[i] = np_after[i + 1] + u32::from(e.pivot[i] == NONE);
                        }
                        let nnz: u64 = e.rows.iter().map(|r| r.nnz() as u64).sum();
                        let room: u64 = e
                            .rows
                            .iter()
                            .map(|r| u64::from(np_after[lead_of(r) as usize + 1]) + 1)
                            .sum();
                        format!("[{n_q},{},{nnz},{room}]", e.rows.len())
                    })
                    .collect();
                let _ = writeln!(
                    f,
                    "{{\"ev\":\"interreduce\",\"c\":{c},\"i\":{},\"rank\":{rank},\"changed\":{changed},\"ops\":{ir_ops},\"words_before\":{before},\"words_after\":{words},\"s\":{dt:.6},\"shape\":[{}]}}",
                    stats.batches,
                    shape.join(",")
                );
            }
        }
        // ── io hook 2 of 3: periodic checkpoint, at a batch boundary ───
        if io_clock.due(stats.batches) {
            let st = ClosureState {
                ech,
                queue,
                rows_reduced,
                contains_one,
                yield_q16,
                fresh_since_ir,
                rank_at_ir,
                stats: Fast2Stats {
                    io: io_report.clone(),
                    ..stats.clone()
                },
                elapsed_before_s: elapsed_before_s + started.elapsed().as_secs_f64(),
                cover: std::mem::take(&mut cover),
            };
            crate::cryptanalysis::hky_closure_io::checkpoint_now(
                &opts.io,
                &io_fp,
                &st,
                &mut io_report,
            );
            (ech, queue, cover) = (st.ech, st.queue, st.cover);
            io_clock.reset();
        }
        // ── end io hook 2 ──────────────────────────────────────────────
    }
    if let Some(f) = trace.as_mut() {
        let _ = f.flush();
    }
    if opts.verify_closed && !contains_one {
        let tv = Instant::now();
        let gen_rows: Vec<(u8, Row)> = live
            .iter()
            .zip(&signs)
            .filter(|(g, _)| HkyRing::degree(g) <= c)
            .map(|(g, &sign)| space.gen_row(g, sign))
            .filter(|(_, r)| !r.is_empty())
            .collect();
        let (n, bad) = verify_closed(&space, &ech, &cx, c, nv, &gen_rows, &pool);
        stats.verify_products = n;
        stats.verify_nonzero = bad;
        stats.verify_s = tv.elapsed().as_secs_f64();
    }
    stats.part_rank = ech.iter().map(|e| e.rows.len() as u64).collect();
    stats.cover_hist = cover_hist(&cover, &space, c, &kappa);
    stats.rank_by_deg = rank_by_deg(&ech, &space, c);
    stats.witness_rows = witness.iter().flatten().map(|w| w.rows.len() as u64).sum();
    stats.covered_cols = cover.iter().flatten().filter(|&&x| x != NO_COVER).count() as u64;
    stats.final_row_bytes = ech
        .iter()
        .flat_map(|e| &e.rows)
        .map(|r| match r {
            RowS::Sparse(v) => 8 * v.len() as u64,
            RowS::Dense { v, .. } => 4 * v.len() as u64,
        })
        .sum();
    stats.sweep_bytes = 8 * sweep_buf.len() as u64;
    let entries: u64 = ech
        .iter()
        .flat_map(|e| &e.rows)
        .map(|r| r.nnz() as u64)
        .sum();
    let mut dims = vec![0u64; c as usize + 1];
    if contains_one {
        for part in &space.parts {
            for &dg in &part.deg {
                dims[dg as usize] += 1;
            }
        }
    } else {
        for (e, part) in ech.iter().zip(&space.parts) {
            for (col, &pv) in e.pivot.iter().enumerate() {
                if pv != NONE {
                    dims[part.deg[col] as usize] += 1;
                }
            }
        }
    }
    for d in 1..dims.len() {
        dims[d] += dims[d - 1];
    }
    if contains_one {
        assert_eq!(
            u128::from(dims[c as usize]),
            n_cols,
            "hky_fp_tower_fast2: full space"
        );
    }
    // ── io hook 3 of 3: export the closed basis ───────────────────────
    if let Some(path) = &opts.io.export {
        let ex = crate::cryptanalysis::hky_closure_io::export_basis(
            path,
            ring,
            &space,
            &ech,
            opts.symmetry.as_deref(),
            contains_one,
            c,
        );
        assert_eq!(
            ex.profile, dims,
            "hky_closure_io: exported profile differs from the closure's"
        );
        io_report.export = Some(ex);
    }
    io_report.elapsed_total_s = elapsed_before_s + started.elapsed().as_secs_f64();
    stats.io = io_report;
    // ── end io hook 3 ──────────────────────────────────────────────────
    (
        Closure {
            c,
            cols: n_cols,
            dims,
            contains_one,
            censored: None,
            rows_reduced,
            entries,
            muladds: stats.muladds,
            ms: started.elapsed().as_secs_f64() * 1e3,
        },
        stats,
    )
}

/// [`fall_degrees_with`](super::hky_fp_tower::fall_degrees_with) on
/// [`closure_fast2`]: the same reading of the closures, word for word.
pub fn fall_degrees_fast2_with<F: FnMut(&Closure, &Fast2Stats)>(
    ring: &HkyRing,
    gens: &[NfPoly],
    c_min: u32,
    c_max: u32,
    limits: &HkyLimits,
    opts: &Fast2Options,
    mut on_closure: F,
) -> FallDegrees {
    let mut closures: Vec<Closure> = Vec::new();
    let mut falls = Vec::new();
    let mut last_fall = None;
    for c in c_min..=c_max {
        let (cl, st) = closure_fast2(ring, gens, c, limits, opts);
        let stop = cl.contains_one || cl.censored.is_some();
        if let Some(prev) = closures.last() {
            if cl.censored.is_none()
                && prev.censored.is_none()
                && cl.dims[c as usize - 1] > prev.dim()
            {
                falls.push(c);
            }
        } else if cl.censored.is_none() && c > 0 && cl.dims[c as usize - 1] > 0 && c == c_min {
            let min_deg = gens
                .iter()
                .filter(|g| !g.is_empty())
                .map(HkyRing::degree)
                .min()
                .unwrap_or(0);
            if c_min - 1 < min_deg {
                falls.push(c);
            }
        }
        if cl.contains_one {
            last_fall = Some(c);
        }
        on_closure(&cl, &st);
        closures.push(cl);
        if stop {
            break;
        }
    }
    let first_fall = falls.first().copied();
    FallDegrees {
        closures,
        last_fall,
        first_fall,
        falls,
    }
}

pub fn fall_degrees_fast2(
    ring: &HkyRing,
    gens: &[NfPoly],
    c_min: u32,
    c_max: u32,
    limits: &HkyLimits,
    opts: &Fast2Options,
) -> FallDegrees {
    fall_degrees_fast2_with(ring, gens, c_min, c_max, limits, opts, |_, _| {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::f4_fp_tower::SquareRule;
    use crate::cryptanalysis::hky_fp_tower::fall_degrees;
    use crate::cryptanalysis::hky_fp_tower_fast::swap_first_two_blocks;

    const P: u64 = 2013265921;

    fn rules(t: usize, blocks: usize, top: u64, general: bool) -> Vec<SquareRule> {
        let mut rules = Vec::new();
        for b in 0..blocks {
            for j in 0..t {
                let last = j + 1 == t;
                let next = (!last).then_some((b * t + j + 1) as u8);
                rules.push(if general {
                    SquareRule {
                        next,
                        a: if last { 0 } else { 3 + j as u64 },
                        b: if last { 0 } else { 5 + 2 * j as u64 },
                        c: 7 + j as u64,
                        d: 11 + 3 * j as u64,
                    }
                } else {
                    SquareRule {
                        next,
                        a: 0,
                        b: u64::from(!last),
                        c: 0,
                        d: if last { top } else { 0 },
                    }
                });
            }
        }
        rules
    }

    fn symmetric_gen(ring: &HkyRing, t: usize, k: u64) -> NfPoly {
        let n = ring.n_vars();
        let e = |pairs: &[(usize, u32)]| {
            let mut v = vec![0u32; n];
            for &(i, x) in pairs {
                v[i] = x;
            }
            v
        };
        ring.from_raw(&[
            (e(&[(0, 2), (t, 2)]), 1),
            (e(&[(0, 1), (t, 1)]), 17 + k),
            (e(&[(0, 2)]), 5),
            (e(&[(t, 2)]), 5),
            (e(&[(0, 1)]), 9 + k),
            (e(&[(t, 1)]), 9 + k),
            (e(&[]), 23 + 7 * k),
        ])
    }

    /// A consistent system (see v1's tests): 1 is in no closure.
    fn consistent(t: usize) -> (HkyRing, Vec<NfPoly>) {
        let y0 = 123_456_789u64;
        let mut top = y0;
        for _ in 0..t {
            top = top * top % P;
        }
        let ring = HkyRing::new(P, rules(t, 2, top, false), 0);
        let mut point = Vec::new();
        for start in [y0, P - y0] {
            let mut y = start;
            for _ in 0..t {
                point.push(y);
                y = y * y % P;
            }
        }
        let g = symmetric_gen(&ring, t, 3);
        let value = ring.eval(&g, &point);
        let n = ring.n_vars();
        let shifted: Vec<(Vec<u32>, u64)> = g
            .iter()
            .map(|&(m, _, x)| ((0..n).map(|j| (m >> j & 1) as u32).collect(), x))
            .chain(std::iter::once((vec![0u32; n], P - value)))
            .collect();
        let h = ring.from_raw(&shifted);
        assert_eq!(ring.eval(&h, &point), 0);
        (ring, vec![h])
    }

    fn differs(a: &FallDegrees, b: &FallDegrees) -> bool {
        a.last_fall != b.last_fall
            || a.falls != b.falls
            || a.closures.len() != b.closures.len()
            || a.closures.iter().zip(&b.closures).any(|(x, y)| {
                x.dims != y.dims || x.contains_one != y.contains_one || x.censored != y.censored
            })
    }

    fn variants(perm: Option<Vec<usize>>) -> Vec<(String, Fast2Options)> {
        let mut out = Vec::new();
        for sym in [false, true] {
            for (ir, min, block) in [(0.0, 0, 1), (0.25, 1, 3), (0.01, 1, 64), (2.0, 4, 2)] {
                for par in [false, true] {
                    for (bmin, bmax, tp) in [(1, 1, 1), (5, 5, 1), (2, 64, 4), (32, 1024, 32)] {
                        for bitmap in [false, true] {
                            for rebucket in [false, true] {
                                let o = Fast2Options {
                                    symmetry: if sym { perm.clone() } else { None },
                                    batch_min: bmin,
                                    batch_max: bmax,
                                    target_pivots: tp,
                                    interreduce: ir,
                                    interreduce_min: min,
                                    interreduce_block: block,
                                    rebucket,
                                    par_second: par,
                                    bitmap,
                                    dense_num: 1,
                                    dense_den: 8,
                                    fault: 0,
                                    verify_closed: false,
                                    deg_weight: [0, 1, 64][(bmin + block) % 3],
                                    np_epochs: (bmin + bmax + usize::from(bitmap)) % 2 == 0,
                                    sweep: rebucket,
                                    sweep_ratio: [0, 1, 4][(bmin + block) % 3],
                                    sweep_tile_bytes: [64, 4096, 1 << 20][block % 3],
                                    sweep_max_words: 1 << 28,
                                    skip: (bmin + block + usize::from(par)) % 2 == 0,
                                    skip_order: (bmax % 2) as u8,
                                    skip_witness: [0u8, 1, 2][(bmax + block) % 3],
                                    skip_witness_max: (block % 3) as u16,
                                    trace: None,
                                    io: Default::default(),
                                };
                                out.push((format!("{o:?}"), o));
                            }
                        }
                    }
                }
            }
        }
        out
    }

    fn all_equal(ring: &HkyRing, gens: &[NfPoly], perm: Option<Vec<usize>>, cmax: u32) {
        let limits = HkyLimits::default();
        let reference = fall_degrees(ring, gens, 1, cmax, &limits);
        for (name, o) in variants(perm) {
            let got = fall_degrees_fast2(ring, gens, 1, cmax, &limits, &o);
            assert!(!differs(&reference, &got), "{name}");
            for (x, y) in reference.closures.iter().zip(&got.closures) {
                assert_eq!(x.cols, y.cols);
            }
        }
    }

    #[test]
    fn v2_equals_the_reference_on_kummer_towers() {
        for t in 2..=4 {
            let ring = HkyRing::new(P, rules(t, 2, 3, false), 0);
            let gens = vec![symmetric_gen(&ring, t, t as u64)];
            all_equal(&ring, &gens, swap_first_two_blocks(t, 2), 5);
        }
    }

    #[test]
    fn v2_equals_the_reference_on_general_rules() {
        for t in 2..=3 {
            let ring = HkyRing::new(P, rules(t, 2, 3, true), 0);
            let gens = vec![symmetric_gen(&ring, t, 1)];
            all_equal(&ring, &gens, swap_first_two_blocks(t, 2), 5);
        }
    }

    #[test]
    fn v2_equals_the_reference_without_one() {
        let t = 4;
        let (ring, gens) = consistent(t);
        let reference = fall_degrees(&ring, &gens, 1, 5, &HkyLimits::default());
        assert_eq!(reference.last_fall, None);
        all_equal(&ring, &gens, swap_first_two_blocks(t, 2), 5);
    }

    #[test]
    fn v2_free_variables_and_three_blocks() {
        let t = 2;
        let ring = HkyRing::new(P, rules(t, 3, 5, false), 1);
        let n = ring.n_vars();
        let e = |pairs: &[(usize, u32)]| {
            let mut v = vec![0u32; n];
            for &(i, x) in pairs {
                v[i] = x;
            }
            v
        };
        let u = 3 * t;
        let g1 = ring.from_raw(&[
            (e(&[(0, 1), (t, 1)]), 1),
            (e(&[(0, 1)]), 4),
            (e(&[(t, 1)]), 4),
            (e(&[(u, 1)]), P - 1),
            (e(&[]), 6),
        ]);
        let g2 = ring.from_raw(&[
            (e(&[(u, 1), (2 * t, 1)]), 1),
            (e(&[(u, 2)]), 3),
            (e(&[(2 * t, 1)]), 8),
            (e(&[]), 2),
        ]);
        let anti = ring.from_raw(&[(e(&[(0, 1), (u, 1)]), 1), (e(&[(t, 1), (u, 1)]), P - 1)]);
        let perm = swap_first_two_blocks(t, 3);
        all_equal(&ring, &[g1.clone(), g2.clone()], perm.clone(), 4);
        all_equal(&ring, &[g1, g2, anti], perm, 4);
    }

    #[test]
    fn v2_faults_are_detected_on_a_consistent_system() {
        let t = 4;
        let (ring, gens) = consistent(t);
        let limits = HkyLimits::default();
        let a = fall_degrees(&ring, &gens, 1, 5, &limits);
        for fault in [1u8, 2, 3, 4, 5] {
            for sym in [false, true] {
                let o = Fast2Options {
                    fault,
                    symmetry: if sym {
                        swap_first_two_blocks(t, 2)
                    } else {
                        None
                    },
                    interreduce: 0.05,
                    interreduce_min: 1,
                    sweep_ratio: 0,
                    ..Fast2Options::default()
                };
                let b = fall_degrees_fast2(&ring, &gens, 1, 5, &limits, &o);
                assert!(differs(&a, &b), "fault {fault} sym {sym} must not pass");
            }
        }
    }

    #[test]
    fn v2_schedule_does_not_depend_on_the_thread_count() {
        let t = 4;
        let ring = HkyRing::new(P, rules(t, 2, 3, false), 0);
        let gens = vec![symmetric_gen(&ring, t, 2)];
        let limits = HkyLimits::default();
        let o = Fast2Options {
            symmetry: swap_first_two_blocks(t, 2),
            interreduce_min: 8,
            ..Fast2Options::default()
        };
        let run = |threads: usize| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| closure_fast2(&ring, &gens, 5, &limits, &o).0)
        };
        let (a, b) = (run(1), run(3));
        assert_eq!(a.dims, b.dims);
        assert_eq!(a.muladds, b.muladds);
        assert_eq!(a.rows_reduced, b.rows_reduced);
        assert_eq!(a.entries, b.entries);
    }
}

#[cfg(test)]
mod opt_tests {
    //! `research/hky_engine_opt_20261118/`: the epoch coordinates and the
    //! sweep do the same reductions as v2, and the closedness check.
    use super::*;
    use crate::cryptanalysis::f4_fp_tower::SquareRule;
    use crate::cryptanalysis::hky_fp_tower_fast::swap_first_two_blocks;

    const P: u64 = 2013265921;

    fn kummer(t: usize, top: u64) -> HkyRing {
        let mut rules = Vec::new();
        for b in 0..2 {
            for j in 0..t {
                let last = j + 1 == t;
                rules.push(SquareRule {
                    next: (!last).then_some((b * t + j + 1) as u8),
                    a: 0,
                    b: u64::from(!last),
                    c: 0,
                    d: if last { top } else { 0 },
                });
            }
        }
        HkyRing::new(P, rules, 0)
    }

    /// A symmetric generator with the root `(y0, P - y0)` planted when
    /// `planted` (then 1 is in no closure).
    fn system(t: usize, planted: bool) -> (HkyRing, Vec<NfPoly>) {
        let y0 = 123_456_789u64;
        let mut top = y0;
        for _ in 0..t {
            top = top * top % P;
        }
        let ring = kummer(t, if planted { top } else { 3 });
        let n = ring.n_vars();
        let e = |pairs: &[(usize, u32)]| {
            let mut v = vec![0u32; n];
            for &(i, x) in pairs {
                v[i] = x;
            }
            v
        };
        let mut terms = vec![
            (e(&[(0, 1), (t, 1)]), 19u64),
            (e(&[(0, 1), (1, 1), (t, 1), (t + 1, 1)]), 7),
            (e(&[(0, 1), (1, 1)]), 5),
            (e(&[(t, 1), (t + 1, 1)]), 5),
            (e(&[(1, 1)]), 11),
            (e(&[(t + 1, 1)]), 11),
        ];
        let g = ring.from_raw(&terms);
        if planted {
            let mut point = Vec::new();
            for start in [y0, P - y0] {
                let mut y = start;
                for _ in 0..t {
                    point.push(y);
                    y = y * y % P;
                }
            }
            terms.push((vec![0u32; n], P - ring.eval(&g, &point)));
        } else {
            terms.push((vec![0u32; n], 29));
        }
        let g = ring.from_raw(&terms);
        (ring, vec![g])
    }

    fn opts(np: bool, sweep: bool, sym: Option<Vec<usize>>) -> Fast2Options {
        Fast2Options {
            symmetry: sym,
            interreduce: 0.1,
            interreduce_min: 8,
            np_epochs: np,
            sweep,
            sweep_ratio: 0,
            sweep_tile_bytes: 64,
            ..Fast2Options::default()
        }
    }

    #[test]
    fn epochs_and_sweep_do_the_same_reductions() {
        let mut exercised = (false, false);
        let limits = HkyLimits::default();
        for planted in [false, true] {
            for t in [3usize, 4] {
                let (ring, gens) = system(t, planted);
                for sym in [None, swap_first_two_blocks(t, 2)] {
                    for c in 3..=5 {
                        let (a, sa) = closure_fast2(
                            &ring,
                            &gens,
                            c,
                            &limits,
                            &opts(false, false, sym.clone()),
                        );
                        for (np, sw) in [(true, false), (true, true), (false, true)] {
                            let (b, sb) =
                                closure_fast2(&ring, &gens, c, &limits, &opts(np, sw, sym.clone()));
                            let tag = format!("planted {planted} t {t} c {c} np {np} sweep {sw}");
                            assert_eq!(a.dims, b.dims, "{tag}");
                            assert_eq!(a.contains_one, b.contains_one, "{tag}");
                            assert_eq!(a.muladds, b.muladds, "{tag}");
                            assert_eq!(a.rows_reduced, b.rows_reduced, "{tag}");
                            assert_eq!(a.entries, b.entries, "{tag}");
                            assert_eq!(sa.zero, sb.zero, "{tag}");
                            assert_eq!(sa.reducers, sb.reducers, "{tag}");
                            exercised.0 |= sb.core_muladds > 0;
                            exercised.1 |= sb.swept_batches > 0;
                        }
                    }
                }
            }
        }
        assert!(
            exercised.0 && exercised.1,
            "phase 0 and the sweep must both run"
        );
    }

    #[test]
    fn the_closedness_check_passes_and_sees_a_fault() {
        let limits = HkyLimits::default();
        let t = 4;
        let (ring, gens) = system(t, true);
        for sym in [None, swap_first_two_blocks(t, 2)] {
            let mut o = opts(true, true, sym.clone());
            o.verify_closed = true;
            let (cl, st) = closure_fast2(&ring, &gens, 5, &limits, &o);
            assert!(!cl.contains_one);
            assert!(st.verify_products > 0);
            assert_eq!(st.verify_nonzero, 0, "a correct closure is closed");
            o.fault = 1;
            let (cl, st) = closure_fast2(&ring, &gens, 5, &limits, &o);
            assert!(!cl.contains_one);
            assert!(
                st.verify_nonzero > 0,
                "products by variable 0 left out: not closed"
            );
        }
    }

    /// The operators x_v commute on every monomial (Kummer and general
    /// rules, t = 3, two blocks): the fact PROOF.md uses, x_b x_a = x_a x_b.
    #[test]
    fn variable_products_commute() {
        use crate::cryptanalysis::hky_fp_tower_fast::mul_tower_var_pub as mul;
        for general in [false, true] {
            let t = 3;
            let mut rules = Vec::new();
            for b in 0..2 {
                for j in 0..t {
                    let last = j + 1 == t;
                    rules.push(SquareRule {
                        next: (!last).then_some((b * t + j + 1) as u8),
                        a: if general && !last { 3 + j as u64 } else { 0 },
                        b: u64::from(!last) * if general { 5 } else { 1 },
                        c: if general { 7 } else { 0 },
                        d: if last {
                            11
                        } else if general {
                            13
                        } else {
                            0
                        },
                    });
                }
            }
            let ring = HkyRing::new(P, rules, 0);
            let n = 2 * t;
            let apply = |v: usize, x: &std::collections::BTreeMap<u64, u64>| {
                let mut out = std::collections::BTreeMap::new();
                for (&m, &c) in x {
                    let mut tmp = Vec::new();
                    mul(&ring, m, v, c, &mut tmp);
                    for (mm, cc) in tmp {
                        let e = out.entry(mm).or_insert(0u64);
                        *e = (*e + cc) % P;
                    }
                }
                out.retain(|_, c| *c != 0);
                out
            };
            for m in 0..(1u64 << n) {
                let x: std::collections::BTreeMap<u64, u64> = [(m, 1u64)].into();
                for a in 0..n {
                    for b in 0..n {
                        assert_eq!(
                            apply(b, &apply(a, &x)),
                            apply(a, &apply(b, &x)),
                            "m {m} a {a} b {b}"
                        );
                    }
                    // degree rises by at most one
                    let d = m.count_ones();
                    assert!(apply(a, &x).keys().all(|mm| mm.count_ones() <= d + 1));
                }
            }
        }
    }

    /// The skip gives the same space as the reference and the closedness
    /// check passes, with and without the symmetry, both orders, with and
    /// without the witness spaces; it does skip; fault 8 (skip every product
    /// of a covered row but the first by order) is seen by the check.
    #[test]
    fn the_skip_keeps_the_space() {
        let limits = HkyLimits::default();
        let mut skipped = 0u64;
        let mut fault_seen = false;
        for planted in [false, true] {
            for t in [3usize, 4] {
                let (ring, gens) = system(t, planted);
                for sym in [None, swap_first_two_blocks(t, 2)] {
                    for c in 3..=6 {
                        let (a, _) =
                            closure_fast2(&ring, &gens, c, &limits, &opts(true, true, sym.clone()));
                        for (order, wit) in [(0u8, 0u8), (1, 0), (0, 1), (1, 1), (0, 2), (1, 2)] {
                            let mut o = opts(true, true, sym.clone());
                            o.skip = true;
                            o.skip_order = order;
                            o.skip_witness = wit;
                            o.verify_closed = true;
                            let (b, sb) = closure_fast2(&ring, &gens, c, &limits, &o);
                            let tag = format!(
                                "planted {planted} t {t} c {c} order {order} witness {wit}"
                            );
                            assert_eq!(a.dims, b.dims, "{tag}");
                            assert_eq!(a.contains_one, b.contains_one, "{tag}");
                            if !b.contains_one {
                                assert_eq!(sb.verify_nonzero, 0, "{tag}");
                            }
                            skipped += sb.skipped_items;
                            o.fault = 8;
                            let (f, sf) = closure_fast2(&ring, &gens, c, &limits, &o);
                            fault_seen |= f.dims != a.dims
                                || f.contains_one != a.contains_one
                                || sf.verify_nonzero > 0;
                        }
                    }
                }
            }
        }
        assert!(skipped > 0, "the criterion must skip something");
        assert!(fault_seen, "fault 8 must be detected");
    }
}
