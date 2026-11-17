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

use crate::cryptanalysis::hky_fp_tower::{count_monomials, Closure, FallDegrees, HkyLimits, HkyRing, NfPoly};
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
    /// drops the last term of every row it rewrites.
    pub fault: u8,
    /// Append one JSON line per batch and per interreduction to this file.
    pub trace: Option<String>,
}

impl Default for Fast2Options {
    fn default() -> Self {
        Fast2Options {
            symmetry: None,
            batch_min: 32,
            batch_max: 1024,
            target_pivots: 32,
            interreduce: 0.25,
            interreduce_min: 256,
            interreduce_block: 256,
            rebucket: true,
            par_second: true,
            bitmap: true,
            dense_num: 1,
            dense_den: 3,
            fault: 0,
            trace: None,
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
    pub dense_rows: u64,
    pub sparse_rows: u64,
    pub stored_words: u64,
    /// The largest `stored_words` seen (before an interreduction).
    pub peak_words: u64,
    pub threads: usize,
    pub simd: &'static str,
}

struct Echelon {
    pivot: Vec<u32>,
    rows: Vec<RowS>,
}

struct Scratch {
    acc: Vec<u64>,
    bm: Vec<u64>,
}

struct Ctx {
    p: u64,
    p2: u64,
    barrett_m: u64,
    simd: Simd,
    bitmap: bool,
}

/// Counts of one reduction.
#[derive(Clone, Copy, Default)]
struct Red {
    ops: u64,
    nred: u32,
}

impl Echelon {
    /// Eliminates every pivot column from `r` (sorted by column, values
    /// below `p`), following the cascade. The result is not normalised.
    /// A pivot row only touches columns after its own.
    fn eliminate(&self, cx: &Ctx, r: &[(u32, u32)], s: &mut Scratch, out: &mut Row) -> Red {
        let (p, p2, bm_) = (cx.p, cx.p2, cx.barrett_m);
        let n = self.pivot.len();
        let mut red = Red::default();
        let (Some(&(first, _)), Some(&(last, _))) = (r.first(), r.last()) else {
            return red;
        };
        let acc = &mut s.acc[..n];
        let bm = &mut s.bm;
        let first = first as usize;
        let mut last = last as usize;
        let span = n - first;
        let mut ci = first;
        // Phase A: follow the bitmap while the reduction is sparse.
        let budget = (span / 4) as u64;
        if cx.bitmap && r.len() * 8 < span {
            for &(col, c) in r {
                acc[col as usize] = u64::from(c);
                bm[col as usize >> 6] |= 1u64 << (col & 63);
            }
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
                let piv = self.pivot[ci];
                if piv == NONE {
                    out.push((ci as u32, v as u32));
                    continue;
                }
                let f = p - v;
                red.nred += 1;
                match &self.rows[piv as usize] {
                    RowS::Sparse(row) => {
                        red.ops += row.len() as u64;
                        for &(c2, x) in &row[1..] {
                            let i = c2 as usize;
                            let t = acc[i] + f * u64::from(x);
                            acc[i] = t.min(t.wrapping_sub(p2));
                            bm[i >> 6] |= 1u64 << (c2 & 63);
                        }
                        last = last.max(row[row.len() - 1].0 as usize);
                        if red.ops > budget {
                            switched = true;
                        }
                    }
                    RowS::Dense { v: dv, nnz, .. } => {
                        red.ops += u64::from(*nnz);
                        axpy_dense(cx.simd, &mut acc[ci + 1..], &dv[1..], f, p2);
                        last = n - 1;
                        switched = true;
                    }
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
        } else {
            for &(col, c) in r {
                acc[col as usize] = u64::from(c);
            }
        }
        // Phase B: v1's scan.
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
            let piv = self.pivot[ci];
            if piv == NONE {
                out.push((ci as u32, v as u32));
                ci += 1;
                continue;
            }
            let f = p - v;
            red.nred += 1;
            match &self.rows[piv as usize] {
                RowS::Sparse(row) => {
                    red.ops += row.len() as u64;
                    for &(c2, x) in &row[1..] {
                        let i = c2 as usize;
                        let t = acc[i] + f * u64::from(x);
                        acc[i] = t.min(t.wrapping_sub(p2));
                    }
                    last = last.max(row[row.len() - 1].0 as usize);
                }
                RowS::Dense { v: dv, nnz, .. } => {
                    red.ops += u64::from(*nnz);
                    axpy_dense(cx.simd, &mut acc[ci + 1..], &dv[1..], f, p2);
                    last = n - 1;
                }
            }
            ci += 1;
        }
        red
    }

    /// A row reduced against the echelon and made monic (empty if zero).
    fn reduce(&self, cx: &Ctx, r: &[(u32, u32)], s: &mut Scratch) -> (Row, Red) {
        let mut out = Vec::new();
        let mut red = self.eliminate(cx, r, s, &mut out);
        red.ops += normalise(&mut out, cx.p);
        (out, red)
    }

    /// Whether a stored row has a pivot column after its leading one.
    fn tail_meets_pivot(&self, row: &RowS) -> bool {
        match row {
            RowS::Sparse(r) => r[1..].iter().any(|&(c, _)| self.pivot[c as usize] != NONE),
            RowS::Dense { lead, v, .. } => v[1..]
                .iter()
                .zip(&self.pivot[*lead as usize + 1..])
                .any(|(&x, &pv)| x != 0 && pv != NONE),
        }
    }

    /// A monic stored row with every pivot column of its tail eliminated.
    fn reduce_tail(&self, cx: &Ctx, row: &RowS, s: &mut Scratch) -> (Row, Red) {
        let mut tail: Row = Vec::with_capacity(row.nnz());
        let mut lead = 0u32;
        let mut first = true;
        row.for_each(|c, x| {
            if first {
                lead = c;
                first = false;
            } else {
                tail.push((c, x));
            }
        });
        let mut out: Row = Vec::with_capacity(row.nnz());
        out.push((lead, 1));
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

fn store(r: Row, n: usize, num: usize, den: usize) -> RowS {
    let lead = r[0].0;
    let span = n - lead as usize;
    if r.len() * den > span * num {
        let mut v = vec![0u32; span];
        for &(col, x) in &r {
            v[(col - lead) as usize] = x;
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

enum Item {
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
        bitmap: opts.bitmap,
    };
    let nv = ring.n_vars();
    let applies = |v: usize| space.applies(v) && !(opts.fault == 1 && v == 0);
    let max_n = space.parts.iter().map(|q| q.keys.len()).max().unwrap_or(0);
    let one_col = (space.parts[0].keys.len() - 1) as u32;
    let mut ech: Vec<Echelon> = space
        .parts
        .iter()
        .map(|q| Echelon {
            pivot: vec![NONE; q.keys.len()],
            rows: Vec::new(),
        })
        .collect();
    const BUCKETS: usize = 66;
    let mut queue: Vec<Vec<Item>> = (0..BUCKETS).map(|_| Vec::new()).collect();
    for (g, &sign) in live.iter().zip(&signs) {
        if HkyRing::degree(g) <= c {
            let (q, r) = space.gen_row(g, sign);
            if !r.is_empty() {
                queue[0].push(Item::Row(q, r));
            }
        }
    }
    let pool = Pool::new(max_n);
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
    'outer: loop {
        let Some(b) = queue.iter().position(|q| !q.is_empty()) else {
            break;
        };
        if started.elapsed() > limits.budget {
            let entries: u64 = ech.iter().flat_map(|e| &e.rows).map(|r| r.nnz() as u64).sum();
            return censored("budget", rows_reduced, entries, stats);
        }
        let want = ((opts.target_pivots as u64 * 65536).div_ceil(yield_q16.max(1)) as usize)
            .clamp(opts.batch_min.max(1), opts.batch_max.max(opts.batch_min).max(1));
        let take = queue[b].len().min(want);
        let at = queue[b].len() - take;
        let batch: Vec<Item> = queue[b].drain(at..).collect();
        stats.batches += 1;
        let t0 = Instant::now();
        let ech_ref = &ech;
        let outs: Vec<ItemOut> = batch
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
                };
                let mut formed: Vec<(u8, Row)> = Vec::new();
                match item {
                    Item::Row(q, r) => formed.push((*q, r.clone())),
                    Item::Prod { part, row, var } => {
                        let e = &ech_ref[*part as usize];
                        let src = &e.rows[*row as usize];
                        o.ops += space.multiply(*part as usize, src, *var as usize, &mut formed);
                        if tracing {
                            o.src_deg = space.parts[*part as usize].deg[lead_of(src) as usize];
                            o.src_age = (e.rows.len() - *row as usize) as u64;
                            o.src_nnz = src.nnz() as u64;
                        }
                    }
                }
                o.formed = formed.len() as u32;
                pool.with(|s| {
                    for (q, r) in formed {
                        o.prod_nnz += r.len() as u64;
                        let (red, rd) = ech_ref[q as usize].reduce(&cx, &r, s);
                        o.ops += rd.ops;
                        o.nred += u64::from(rd.nred);
                        if !red.is_empty() {
                            o.cands.push((q, red));
                        }
                    }
                });
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
        let mut b_deg = [0u32; 16];
        let mut cands: Vec<(u8, Row)> = Vec::new();
        for o in outs {
            b_ops += o.ops;
            b_max = b_max.max(o.ops);
            b_nred += o.nred;
            b_formed += u64::from(o.formed);
            b_nnz += o.prod_nnz;
            b_age += o.src_age;
            b_src += o.src_nnz;
            b_deg[(o.src_deg as usize).min(15)] += 1;
            cands.extend(o.cands);
        }
        stats.muladds += b_ops;
        stats.products += b_formed;
        stats.reducers += b_nred;
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
                        e.rows.push(RowS::Sparse(r));
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
                        ech[q as usize].rows[ri as usize] = RowS::Sparse(r);
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
                e.rows.push(RowS::Sparse(r));
                fresh[qi] = true;
            }
            serial += ts.elapsed().as_secs_f64();
        }
        // Finalise the inserted rows: storage form, queue, counts.
        let ts = Instant::now();
        for &(q, ri) in &inserted {
            let qi = q as usize;
            let n_q = ech[qi].pivot.len();
            let RowS::Sparse(r) = std::mem::replace(&mut ech[qi].rows[ri as usize], RowS::Sparse(Vec::new()))
            else {
                unreachable!("rows are inserted sparse");
            };
            let lead = r[0].0;
            let stored = store(r, n_q, dense.0, dense.1);
            match stored {
                RowS::Dense { .. } => stats.dense_rows += 1,
                RowS::Sparse(_) => stats.sparse_rows += 1,
            }
            stats.stored_words += stored.words() as u64;
            let d = u32::from(space.parts[qi].deg[lead as usize]);
            if d < c {
                let bucket = sparse_bucket(stored.nnz(), 1);
                for v in 0..nv {
                    if applies(v) {
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
                "{{\"ev\":\"batch\",\"c\":{c},\"i\":{},\"bucket\":{b},\"items\":{take},\"formed\":{b_formed},\"prod_nnz\":{b_nnz},\"src_nnz\":{b_src},\"zero\":{},\"cands\":{b_cands},\"pivots\":{},\"rounds\":{rounds},\"ops\":{b_ops},\"max_ops\":{b_max},\"ops2\":{second_ops},\"nred\":{b_nred},\"age\":{b_age},\"deg\":{:?},\"rank\":{rank},\"words\":{},\"par_s\":{par_dt:.6},\"sec_s\":{:.6},\"serial_s\":{serial:.6}}}",
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
            ech.iter().flat_map(|e| &e.rows).map(|r| r.nnz() as u64).sum()
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
                let n_q = e.pivot.len();
                let mut order: Vec<u32> = (0..e.rows.len() as u32).collect();
                order.sort_unstable_by_key(|&i| std::cmp::Reverse(lead_of(&e.rows[i as usize])));
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
                                (store(r, n_q, dense.0, dense.1), rd)
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
                let old: Vec<Vec<Item>> = std::mem::replace(&mut queue, (0..BUCKETS).map(|_| Vec::new()).collect());
                for (bi, items) in old.into_iter().enumerate() {
                    for it in items {
                        let nb = match &it {
                            Item::Row(..) => bi,
                            Item::Prod { part, row, .. } => {
                                sparse_bucket(ech[*part as usize].rows[*row as usize].nnz(), 1)
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
                let _ = writeln!(
                    f,
                    "{{\"ev\":\"interreduce\",\"c\":{c},\"i\":{},\"rank\":{rank},\"changed\":{changed},\"ops\":{ir_ops},\"words_before\":{before},\"words_after\":{words},\"s\":{dt:.6}}}",
                    stats.batches
                );
            }
        }
    }
    if let Some(f) = trace.as_mut() {
        let _ = f.flush();
    }
    stats.part_rank = ech.iter().map(|e| e.rows.len() as u64).collect();
    let entries: u64 = ech.iter().flat_map(|e| &e.rows).map(|r| r.nnz() as u64).sum();
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
        assert_eq!(u128::from(dims[c as usize]), n_cols, "hky_fp_tower_fast2: full space");
    }
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
                                    trace: None,
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
        for fault in [1u8, 2, 3] {
            for sym in [false, true] {
                let o = Fast2Options {
                    fault,
                    symmetry: if sym { swap_first_two_blocks(t, 2) } else { None },
                    interreduce: 0.05,
                    interreduce_min: 1,
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
