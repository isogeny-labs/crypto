//! # Fast `F_2` Macaulay kernels for the Boolean matrix-F4 engine.
//!
//! [`super::koblitz_groebner`] reduces every node of its splitting search
//! with one or two Macaulay matrices.  The matrices are small — a few
//! hundred rows over a few hundred columns on the frozen ladder — and
//! there are tens of thousands of them per relation, so the reference
//! implementation's cost was never the elimination alone: building a
//! matrix (a hash map from monomial to column, two sorts, one heap row per
//! product) and reading every reduced row back into a sorted polynomial
//! cost as much again.  This module removes both, and reduces what is left
//! with less work, without changing a single answer.
//!
//! ## Columns without hashing
//!
//! A column is a Boolean monomial of degree at most `D` over the variables
//! that occur.  Its position in descending DegRevLex order is computable:
//! within one degree, DegRevLex descending is ascending integer order of
//! the mask, which is colex order of the variable subset, so
//!
//! ```text
//!     rank(x) = base[deg x] + Σ_i C(c_i, i)      (c_1 < c_2 < … compact indices)
//! ```
//!
//! is monotone in the column order.  One stamped array indexed by rank
//! replaces the hash map, and a scan (or a sort of the distinct ranks when
//! the rank space is sparse) replaces both sorts.  Rows are written by XOR,
//! so terms that collide under the Boolean product cancel with no sort.
//!
//! ## Deciding without the full reduction
//!
//! The splitting solver reads exactly two facts off a reduction: does the
//! row space contain the constant `1`, and which rows have collapsed to
//! `v` or `v + 1`.  Both live in the span of the degree-≤1 columns, which
//! DegRevLex puts last.  After forward elimination over the high columns
//! alone, the rows left with a zero high part span precisely the row space
//! intersected with that span (any combination that involves a pivot row
//! keeps a nonzero entry in that pivot's column), so the reduced echelon
//! form of those rows — at most 65 columns wide, one `u128` each — is the
//! low block of the full reduced echelon form, and answers both questions
//! identically.  No back substitution over the high columns and no
//! polynomial readback is performed.
//!
//! ## Only the variables that occur
//!
//! The reference builder shifts every equation by monomials over *all*
//! `n_vars` variables, including the ones the search has already
//! substituted away.  A row whose multiplier contains such a variable has
//! every monomial divisible by it, so those rows occupy columns no other
//! row touches and cannot contribute a linear consequence in the remaining
//! variables (a constant `1` among them would already be a refutation one
//! degree lower, which the other rows contain).  They are therefore not
//! built.  The size caps are nevertheless applied in the reference units:
//! a nonempty row with slack `s = D − deg p − deg u` stands for
//! `Σ_{j ≤ s} C(N, j)` reference rows (`N` non-occurring variables), and a
//! column whose largest slack is `s` for as many reference columns, so a
//! matrix is declared oversize exactly when the reference builder would
//! have refused it and the search takes the same path.
//!
//! ## What is exact
//!
//! [`decide`] returns the same refutation flag and the same forced
//! assignments, in the same order, as reading them off the reference
//! reduced rows; [`matrix_rows`] returns the reference reduced rows
//! bit-for-bit.  Both report the reference row and column counts.  The
//! 64-bit word XORs they report are their own, which is the point.
//!
//! ## Port and same-binary comparison against this crate's kernel
//!
//! This file is the sibling suite's `f4_gf2` ported to this crate's
//! `F2BoolPoly`/`F2BoolMono` (no shim was needed; the suite's file imports
//! only `fx_hash`, `pq_groebner_f2` and `sparse_macaulay::xor_sorted`, all
//! present here).  The suite's three `f4_gpu` hooks are host-only stubs in
//! `f4_gpu_stub` at the end of the file (`offload_expected` is `false`, the
//! echelon hooks return `None`); port 3 of
//! `docs/ic/STACK_DIVERGENCE_20261018.md` (the GPU echelon) replaces them.
//! The only edit outside this file is `pub(crate)` on
//! `koblitz_groebner::matrix_f4_f2_solver_consequences`, so the comparison
//! can time the call the default `SolverEngine::MatrixF4` makes.
//! The suite compared this kernel with *its* reference kernel; the question
//! here is the gain over *this* crate's `matrix_f4_f2`.
//!
//! Measured in one test binary (`f4_gf2_same_binary_comparison`, ignored by
//! default; command in `docs/ic/perf/F4_GF2_PORT_COMPARISON_20261018.md`):
//! the Koblitz Weil-descended `m = 3` cell `n = 15`, `ℓ = 5` (30 Boolean
//! variables, 30 equations), Macaulay degree 4, 20 draws from
//! `ladder_sample`, release build, 4-vCPU KVM guest (Xeon 2.80 GHz, no PMU).
//! Per-draw wall time in ms, median over the 20 draws (range of the median
//! over four full runs); every draw's decision (here: undecided) is equal
//! across the four arms:
//!
//! | arm | median ms | vs crate full | 64-bit word XORs (median) |
//! |:--|--:|--:|--:|
//! | crate `matrix_f4_f2_counted_with(.., None)` (full readback) | 199-224 | 1.00x | 154 963 841 |
//! | crate `matrix_f4_f2_solver_consequences` (what the engine calls) | 386-397 | 0.50-0.58x | 64 342 540 |
//! | port `decide`, F5 on (the suite default) | 35.6-37.3 | 5.6-6.0x | 57 472 744 (elimination only) |
//! | port `decide`, F5 off | 40.1-42.2 | 5.0-5.4x | 68 746 716 |
//!
//! The port is 10.4-11.2x faster than the crate's solver path and 5.6-6.0x
//! faster than its full-readback path on the same inputs and decisions.  The
//! word-XOR count falls only 1.12x against the solver path (57.5 M against
//! 64.3 M; the F5 plan's sorted-list XORs are not in the port's count), so
//! the gain is the rank-indexed build and the absent readback, not the
//! elimination.  The crate's solver path is slower than its own full path
//! at this width (30 variables is above its 24-variable switch to the
//! active-multiplier linear-tail build).  Class: engineering, a kernel-level
//! improvement at one cell and one degree; it is not an end-to-end `S`
//! measurement and makes no comparison with rho.

use std::cell::RefCell;
use std::sync::OnceLock;

use super::fx_hash::FxMap;
use super::pq_groebner_f2::{F2BoolMono, F2BoolPoly};

/// Size caps of one Macaulay matrix, in the reference builder's units:
/// nonempty rows over all `n_vars` variables, and distinct monomials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MacaulayCaps {
    /// Largest number of nonempty rows accepted.
    pub max_rows: usize,
    /// Largest number of distinct monomials accepted.
    pub max_cols: usize,
}

/// What the reduced rows of one Macaulay matrix decide.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Decision {
    /// The row space contains the constant `1`.
    pub refuted: bool,
    /// Rows of the form `v` (`false`) or `v + 1` (`true`), in increasing
    /// variable order — the order of their pivots.  Empty when `refuted`:
    /// once `1` is in the ideal every `v = v·1` is too, including variables
    /// the system no longer contains, and nothing reads them.
    pub forced: Vec<(u32, bool)>,
}

impl Decision {
    /// A refutation or at least one forced variable.
    pub fn is_decisive(&self) -> bool {
        self.refuted || !self.forced.is_empty()
    }
}

/// Options of one kernel call.  None changes an answer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KernelOptions {
    /// Leave out the rows the F5 criterion and the Boolean field-equation
    /// criterion prove to be in the span of the others.  For the rows
    /// `t·f_i` of the equations in order: F5 drops `t·f_i` when `t` leads
    /// an element of the degree-`D − deg f_i` row space of `f_0 … f_{i−1}`;
    /// the field equations (`f² = f`, so `s·f = Σ_{a ∈ supp f} (s·a)·f`)
    /// drop the largest row of each such relation within the degree bound.
    /// Each dropped row is a sum of rows of smaller key, so every reduced
    /// echelon form is unchanged.  Dropped rows are still counted against
    /// the caps.
    pub f5: bool,
}

/// The options the solver uses: [`KernelOptions::f5`] on unless `F4_F2_F5`
/// is `0`, `off` or `false`.  The plan is empty — and free — below the
/// degree where a trivial syzygy fits, and measured 1.2–1.8× faster at
/// degree 5 (`examples/f4_degree_bench.rs`).
pub fn default_options() -> KernelOptions {
    static F5: OnceLock<bool> = OnceLock::new();
    KernelOptions {
        f5: *F5.get_or_init(|| {
            !matches!(
                std::env::var("F4_F2_F5").as_deref(),
                Ok("0") | Ok("off") | Ok("false")
            )
        }),
    }
}

/// Counters of one kernel call, in the fields of
/// [`super::koblitz_groebner::F4Profile`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KernelCounters {
    /// A matrix was built (possibly empty).  `false` for an empty input
    /// system and for an oversize matrix.
    pub built: bool,
    /// The matrix exceeded the caps.
    pub oversize: bool,
    /// Rows in reference units.
    pub rows: u64,
    /// Columns in reference units.
    pub cols: u64,
    /// Rows actually eliminated.
    pub eliminated_rows: u64,
    /// Columns actually eliminated.
    pub eliminated_cols: u64,
    /// Nonempty rows left out by the F5 criteria.
    pub f5_skipped: u64,
    /// Rank of the matrix built.  For [`decide`] that is the matrix over
    /// the occurring variables, whose rank equals the reference matrix's
    /// only when every variable occurs: rows shifted by an absent variable
    /// are copies in columns of their own and add their own rank.
    pub rank: u64,
    /// 64-bit word XORs performed by this kernel.
    pub word_ops: u64,
    /// Nanoseconds building the matrix.
    pub build_ns: u128,
    /// Nanoseconds eliminating.
    pub reduce_ns: u128,
    /// Nanoseconds extracting the answer.
    pub readback_ns: u128,
}

/// Rank spaces up to this many entries are indexed directly; larger ones
/// fall back to a hash map.
const RANK_LIMIT: u64 = 1 << 20;

/// A candidate matrix of at most this many words is filled before its
/// reference column count is known; a larger one is counted sparsely first,
/// so a matrix the caps refuse is never allocated.
const FILL_LIMIT: usize = 1 << 22;

/// `C(n, k)` for `n, k ≤ 64`, saturating at `u64::MAX`.
fn binom() -> &'static [[u64; 65]; 65] {
    static TABLE: OnceLock<Box<[[u64; 65]; 65]>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = Box::new([[0u64; 65]; 65]);
        for n in 0..=64 {
            t[n][0] = 1;
            for k in 1..=n {
                let above = if k < n { t[n - 1][k] } else { 0 };
                t[n][k] = t[n - 1][k - 1].saturating_add(above);
            }
        }
        t
    })
}

/// `Σ_{j ≤ s} C(n, j)`: the multipliers of degree at most `s` over `n`
/// variables.
fn upto(n: usize, s: usize) -> u64 {
    let b = binom();
    (0..=s.min(n)).fold(0u64, |acc, j| acc.saturating_add(b[n][j]))
}

/// Mask of the variables `0 .. n_vars`.
fn all_vars(n_vars: usize) -> u64 {
    if n_vars >= 64 {
        u64::MAX
    } else {
        (1u64 << n_vars) - 1
    }
}

/// Highest term degree of `p` (0 for the zero polynomial).
fn poly_degree(p: &F2BoolPoly) -> u32 {
    p.terms
        .iter()
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(0)
}

/// Every subset of `bits` (single-bit masks) of size at most `k`, as
/// `(mask, size)`, the empty set first.
fn for_each_multiplier(bits: &[u64], k: usize, mut f: impl FnMut(u64, usize)) {
    f(0, 0);
    let v = bits.len();
    let mut idx = [0usize; 64];
    for j in 1..=k.min(v) {
        for (t, slot) in idx.iter_mut().enumerate().take(j) {
            *slot = t;
        }
        loop {
            let mask = idx[..j].iter().fold(0u64, |m, &i| m | bits[i]);
            f(mask, j);
            // Next j-combination of 0..v in lexicographic order.
            let mut t = j;
            let mut advanced = false;
            while t > 0 {
                t -= 1;
                if idx[t] < v - j + t {
                    idx[t] += 1;
                    for s in t + 1..j {
                        idx[s] = idx[s - 1] + 1;
                    }
                    advanced = true;
                    break;
                }
            }
            if !advanced {
                break;
            }
        }
    }
}

/// Per-thread buffers, reused across calls so a reduction allocates
/// nothing once warm.
#[derive(Default)]
struct Scratch {
    /// Provisional column key of every monomial occurrence, row-major.
    prov: Vec<u32>,
    /// Start of each candidate row in `prov`; one extra entry at the end.
    row_start: Vec<u32>,
    /// Slack `D − deg p − deg u` of each candidate row.
    row_slack: Vec<u8>,
    /// Whether the F5 plan leaves each candidate row out of elimination.
    row_skip: Vec<bool>,
    /// Rank-space stamps: `stamp[r] == generation` marks rank `r` seen.
    stamp: Vec<u32>,
    generation: u32,
    /// Rank (or hash id) → distinct index while building, then → column.
    remap: Vec<u32>,
    /// Distinct monomials: `(rank or id, mask)`.
    distinct: Vec<(u32, u64)>,
    /// Column → monomial mask, in column order.
    col_mask: Vec<u64>,
    /// Row-major bit matrix of the nonempty rows.
    matrix: Vec<u64>,
    /// Slack of each stored row.
    stored_slack: Vec<u8>,
    /// Stored rows left out of elimination (filled only to be counted).
    stored_skip: Vec<bool>,
    /// Per-slack OR of the stored rows (reference column count).
    class_or: Vec<u64>,
    /// Elimination: leading-column buckets.
    head: Vec<u32>,
    next: Vec<u32>,
    pivots: Vec<u32>,
    low_rows: Vec<u32>,
    low: Vec<u128>,
    /// Hash strategy for rank spaces above [`RANK_LIMIT`].
    hash: FxMap<u64, u32>,
    /// Sorting buffer for the exact oversize count.
    row_buf: Vec<u64>,
}

thread_local! {
    static SCRATCH: RefCell<Scratch> = RefCell::new(Scratch::default());
}

/// Shape of a built matrix.
struct Built {
    rows: usize,
    cols: usize,
    stride: usize,
    low_start: usize,
    ref_rows: u64,
    ref_cols: u64,
}

enum BuildOutcome {
    /// No nonempty row: the reference returns no rows either.
    Empty,
    Oversize,
    Built(Built),
}

/// The multiplier set, in both modes.
struct Layout {
    /// Variables multipliers range over, as single-bit masks.
    bits: Vec<u64>,
    /// Variables the reference multiplies by that are not in `bits`.
    missing: usize,
    /// Compact index of each variable in `bits` (by bit position).
    pos: [u8; 64],
    /// Rank offset of each degree, `base[d]`.
    base: [u64; 65],
    /// Size of the rank space.
    rank_space: u64,
}

impl Layout {
    fn new(occurring: u64, n_vars: usize, degree: u32) -> Self {
        let bits: Vec<u64> = (0..64u32)
            .filter(|b| occurring >> b & 1 == 1)
            .map(|b| 1u64 << b)
            .collect();
        let mut pos = [0u8; 64];
        for (i, b) in bits.iter().enumerate() {
            pos[b.trailing_zeros() as usize] = i as u8;
        }
        let v = bits.len();
        let dm = (degree as usize).min(v);
        let b = binom();
        let mut base = [0u64; 65];
        // Highest degree first: base[dm] = 0, base[d] = base[d + 1] + C(v, d + 1).
        for d in (0..dm).rev() {
            base[d] = base[d + 1].saturating_add(b[v][d + 1]);
        }
        let rank_space = base[0].saturating_add(1);
        let missing = (all_vars(n_vars) & !occurring).count_ones() as usize;
        Layout {
            bits,
            missing,
            pos,
            base,
            rank_space,
        }
    }

    #[inline]
    fn rank(&self, x: u64) -> u32 {
        let b = binom();
        let d = x.count_ones() as usize;
        let mut r = self.base[d];
        let mut m = x;
        let mut k = 1usize;
        while m != 0 {
            let bit = m.trailing_zeros() as usize;
            r += b[self.pos[bit] as usize][k];
            k += 1;
            m &= m - 1;
        }
        r as u32
    }
}

/// Rows of one Macaulay matrix that are provably in the span of the rest.
///
/// Rows are `t·f_i` for the generating equations `f_0, f_1, …` in order,
/// keyed `(i, t)` with `t` in DegRevLex.  Two criteria, each expressing a
/// row through rows of strictly smaller key, so by induction on the key
/// every removed row lies in the span of the kept ones and every reduced
/// echelon form — hence every answer — is unchanged:
///
/// - **F5** (Faugère).  If `t` is the leading monomial of some `g` in the
///   row space of `M_{≤D−deg f_i}(f_0 … f_{i−1})`, then
///   `t·f_i = g·f_i + (g + t)·f_i`.  The first term is a sum of rows
///   `w·f_j` with `j < i` (expand `g·f_i = Σ m_k f_{j_k}·f_i` and move each
///   `f_i` into the multiplier; degrees stay within `D`), the second a sum
///   of rows `u·f_i` with `u < t`.  The leading monomials come from an
///   echelon form of the lower-degree matrix built in equation order,
///   recording the first equation whose rows produce each one.
/// - **Field equations.**  In the Boolean ring `f² = f`, so for any `s`,
///   `s·f = s·f·f = Σ_{a ∈ supp f} (s·a)·f`.  When every multiplier in that
///   relation is within the degree bound, the largest one with an odd
///   coefficient names a row equal to the sum of the others.
///
/// Neither fires below `D = 2·deg f` for the degree-`deg f` equations, so on
/// the splitting search's quadratic systems at `D ≤ 3` the plan is empty.
struct F5Plan {
    /// Per lower degree `E`: leading monomial rank → first equation index.
    leading: Vec<(u32, FxMap<u32, u32>)>,
    /// `(equation index, multiplier)` rows removed by the field equations.
    frobenius: std::collections::HashSet<(u32, u64), super::fx_hash::FxBuild>,
}

impl F5Plan {
    /// The plan for the generating equations at `degree`, or `None` when
    /// neither criterion can apply.
    fn new(generating: &[(&F2BoolPoly, u32)], layout: &Layout, degree: u32) -> Option<Self> {
        let min_deg = generating.iter().map(|&(_, d)| d).min()?;
        let applicable = generating
            .iter()
            .any(|&(_, d)| degree - d >= min_deg.max(1) || degree - d >= d.max(1));
        if !applicable {
            return None;
        }
        let mut leading: Vec<(u32, FxMap<u32, u32>)> = Vec::new();
        for &(_, d) in generating {
            let e = degree - d;
            if e >= min_deg && !leading.iter().any(|(x, _)| *x == e) {
                leading.push((e, Self::first_leading(generating, layout, e)));
            }
        }
        let mut frobenius = std::collections::HashSet::default();
        let mut ws: Vec<u64> = Vec::new();
        for (i, &(p, d)) in generating.iter().enumerate() {
            let e = degree - d;
            if e < d.max(1) {
                continue;
            }
            for_each_multiplier(&layout.bits, e as usize, |s, _| {
                if p.terms.iter().any(|t| (t.mask | s).count_ones() > e) {
                    return;
                }
                ws.clear();
                ws.extend(p.terms.iter().map(|t| t.mask | s));
                ws.push(s);
                ws.sort_unstable_by_key(|&w| (std::cmp::Reverse(w.count_ones()), w));
                let mut k = 0;
                while k < ws.len() {
                    let mut l = k;
                    while l < ws.len() && ws[l] == ws[k] {
                        l += 1;
                    }
                    if (l - k) % 2 == 1 {
                        frobenius.insert((i as u32, ws[k]));
                        break;
                    }
                    k = l;
                }
            });
        }
        Some(F5Plan { leading, frobenius })
    }

    /// Leading monomials of the row space of `M_{≤e}` of the equations of
    /// degree at most `e`, each mapped to the first equation whose rows
    /// produce it, by top-reducing the rows in equation order.
    fn first_leading(
        generating: &[(&F2BoolPoly, u32)],
        layout: &Layout,
        e: u32,
    ) -> FxMap<u32, u32> {
        use super::sparse_macaulay::xor_sorted;
        let mut pivots: FxMap<u32, Vec<u32>> = FxMap::default();
        let mut first: FxMap<u32, u32> = FxMap::default();
        for (j, &(p, d)) in generating.iter().enumerate() {
            if d > e {
                continue;
            }
            for_each_multiplier(&layout.bits, (e - d) as usize, |u, _| {
                let mut row: Vec<u32> = p.terms.iter().map(|t| layout.rank(t.mask | u)).collect();
                row.sort_unstable();
                let mut kept = Vec::with_capacity(row.len());
                let mut k = 0;
                while k < row.len() {
                    if k + 1 < row.len() && row[k] == row[k + 1] {
                        k += 2;
                    } else {
                        kept.push(row[k]);
                        k += 1;
                    }
                }
                while let Some(&lead) = kept.first() {
                    match pivots.get(&lead) {
                        Some(pivot) => kept = xor_sorted(&kept, pivot),
                        None => break,
                    }
                }
                if let Some(&lead) = kept.first() {
                    first.insert(lead, j as u32);
                    pivots.insert(lead, kept);
                }
            });
        }
        first
    }

    /// Whether row `(i, u)` of an equation of degree `d` is redundant.
    fn skips(&self, layout: &Layout, degree: u32, i: usize, d: u32, u: u64) -> bool {
        let e = degree - d;
        if let Some((_, lead)) = self.leading.iter().find(|(x, _)| *x == e) {
            if lead.get(&layout.rank(u)).is_some_and(|&j| (j as usize) < i) {
                return true;
            }
        }
        self.frobenius.contains(&(i as u32, u))
    }
}

/// Build the Macaulay matrix of `polys` at `degree` into the scratch.
///
/// With `compact`, multipliers range over the variables occurring in the
/// row-generating equations; otherwise over all `n_vars` (the reference
/// matrix exactly).  Either way the caps are enforced in reference units.
fn build(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    compact: bool,
    caps: MacaulayCaps,
    opts: KernelOptions,
    s: &mut Scratch,
) -> BuildOutcome {
    let generating: Vec<(&F2BoolPoly, u32)> = polys
        .iter()
        .filter(|p| !p.is_zero())
        .map(|p| (p, poly_degree(p)))
        .filter(|&(_, d)| d <= degree)
        .collect();
    let term_vars = generating
        .iter()
        .flat_map(|(p, _)| p.terms.iter())
        .fold(0u64, |acc, t| acc | t.mask);
    let occurring = if compact {
        term_vars
    } else {
        term_vars | all_vars(n_vars)
    };
    let layout = Layout::new(occurring, n_vars, degree);
    let v = layout.bits.len();
    let missing = layout.missing;

    // Reference rows, before cancellation: an upper bound on the count.
    let b = binom();
    let mut rows_bound = 0u64;
    let mut candidates_bound = 0u64;
    for &(_, pdeg) in &generating {
        let k = (degree - pdeg) as usize;
        for j in 0..=k.min(v) {
            rows_bound = rows_bound.saturating_add(b[v][j].saturating_mul(upto(missing, k - j)));
            candidates_bound = candidates_bound.saturating_add(b[v][j]);
        }
    }
    if rows_bound > caps.max_rows as u64
        && exact_reference_rows_exceed(&generating, &layout, degree, caps.max_rows as u64, s)
    {
        return BuildOutcome::Oversize;
    }

    // Candidate rows: provisional column keys of every product term.
    s.prov.clear();
    s.row_start.clear();
    s.row_slack.clear();
    s.row_skip.clear();
    s.distinct.clear();
    // F5 only drops rows in the span of the others.  On a matrix that will
    // be eliminated on a device its symbolic half costs more host time than
    // the dropped rows save there, so it is skipped; the answer is the same.
    let words_bound = candidates_bound.saturating_mul(layout.rank_space.div_ceil(64));
    let plan = if opts.f5 && !f4_gpu_stub::offload_expected(words_bound) {
        F5Plan::new(&generating, &layout, degree)
    } else {
        None
    };
    let use_rank = layout.rank_space <= RANK_LIMIT;
    if use_rank {
        let r = layout.rank_space as usize;
        if s.stamp.len() < r {
            s.stamp.resize(r, 0);
            s.remap.resize(r, 0);
        }
        s.generation = s.generation.wrapping_add(1);
        if s.generation == 0 {
            s.stamp.iter_mut().for_each(|x| *x = 0);
            s.generation = 1;
        }
    } else {
        s.hash.clear();
    }
    let generation = s.generation;
    let mut max_key = 0u32;
    for (i, &(p, pdeg)) in generating.iter().enumerate() {
        let k = (degree - pdeg) as usize;
        for_each_multiplier(&layout.bits, k, |u, du| {
            let slack = (k - du) as u8;
            s.row_start.push(s.prov.len() as u32);
            s.row_slack.push(slack);
            s.row_skip.push(
                plan.as_ref()
                    .is_some_and(|plan| plan.skips(&layout, degree, i, pdeg, u)),
            );
            for t in &p.terms {
                let x = t.mask | u;
                let key = if use_rank {
                    let r = layout.rank(x) as usize;
                    if s.stamp[r] != generation {
                        s.stamp[r] = generation;
                        s.remap[r] = s.distinct.len() as u32;
                        s.distinct.push((r as u32, x));
                    }
                    r as u32
                } else {
                    let next_id = s.distinct.len() as u32;
                    let id = *s.hash.entry(x).or_insert(next_id);
                    if id == next_id {
                        s.distinct.push((id, x));
                    }
                    id
                };
                max_key = max_key.max(key);
                s.prov.push(key);
            }
        });
    }
    s.row_start.push(s.prov.len() as u32);
    let candidates = s.row_slack.len();
    if s.distinct.is_empty() || candidates == 0 {
        return BuildOutcome::Empty;
    }

    // Reference columns, before cancellation: an upper bound.  When it
    // exceeds the cap, a matrix small enough to fill is counted exactly by
    // the fill below; only a larger one is counted sparsely first.
    let max_slack = s.row_slack.iter().copied().max().unwrap_or(0) as usize;
    let cols_bound = (s.distinct.len() as u64).saturating_mul(upto(missing, max_slack));
    let fill_words = candidates.saturating_mul(s.distinct.len().div_ceil(64));
    if cols_bound > caps.max_cols as u64
        && fill_words > FILL_LIMIT
        && exact_reference_cols_exceed(missing, caps, s)
    {
        return BuildOutcome::Oversize;
    }

    // Column order: descending DegRevLex.
    let n_cols = s.distinct.len();
    s.col_mask.clear();
    s.col_mask.resize(n_cols, 0);
    if use_rank {
        let r = layout.rank_space as usize;
        if r <= 8 * n_cols + 64 {
            let mut next = 0u32;
            for rank in 0..r {
                if s.stamp[rank] == generation {
                    s.remap[rank] = next;
                    next += 1;
                }
            }
        } else {
            s.distinct.sort_unstable_by_key(|e| e.0);
            for (col, e) in s.distinct.iter().enumerate() {
                s.remap[e.0 as usize] = col as u32;
            }
        }
        for e in &s.distinct {
            s.col_mask[s.remap[e.0 as usize] as usize] = e.1;
        }
    } else {
        s.distinct
            .sort_unstable_by_key(|e| (std::cmp::Reverse(e.1.count_ones()), e.1));
        if s.remap.len() < n_cols {
            s.remap.resize(n_cols, 0);
        }
        for (col, e) in s.distinct.iter().enumerate() {
            s.remap[e.0 as usize] = col as u32;
            s.col_mask[col] = e.1;
        }
    }
    let low_start = s
        .col_mask
        .iter()
        .rposition(|m| m.count_ones() >= 2)
        .map_or(0, |i| i + 1);

    // Fill.  XOR cancels colliding products; empty rows are dropped.
    let stride = n_cols.div_ceil(64);
    if candidates * stride > 1 << 22 {
        // Fresh zeroed pages instead of writing gigabytes of zeros.
        s.matrix = vec![0; candidates * stride];
    } else {
        s.matrix.clear();
        s.matrix.resize(candidates * stride, 0);
    }
    s.stored_slack.clear();
    let classes = degree as usize + 1;
    s.class_or.clear();
    s.class_or.resize(classes * stride, 0);
    s.stored_skip.clear();
    let mut stored = 0usize;
    let mut ref_rows = 0u64;
    assert!(
        (max_key as usize) < s.remap.len(),
        "column key out of range"
    );
    for row in 0..candidates {
        let dst = stored * stride;
        let (lo, hi) = (s.row_start[row] as usize, s.row_start[row + 1] as usize);
        let words = &mut s.matrix[dst..dst + stride];
        let keys = &s.prov[lo..hi];
        // SAFETY (both loops below): every key is at most `max_key`, inside
        // `remap` (asserted above), and `remap` sends every key that occurs
        // to a column `< n_cols ≤ 64 · stride`.
        let word_of = |key: u32| unsafe { *s.remap.get_unchecked(key as usize) } as usize / 64;
        for &key in keys {
            let c = unsafe { *s.remap.get_unchecked(key as usize) } as usize;
            debug_assert!(c < n_cols);
            unsafe {
                *words.get_unchecked_mut(c / 64) ^= 1u64 << (c % 64);
            }
        }
        // A row's nonzero words are among those its keys touched; on a
        // wide matrix reading only those beats scanning the row.
        let sparse = 2 * keys.len() < stride;
        let empty = if sparse {
            keys.iter().all(|&key| words[word_of(key)] == 0)
        } else {
            words.iter().all(|&w| w == 0)
        };
        if empty {
            continue;
        }
        let slack = s.row_slack[row];
        let class = &mut s.class_or[slack as usize * stride..(slack as usize + 1) * stride];
        if sparse {
            for &key in keys {
                let w = word_of(key);
                class[w] |= words[w];
            }
        } else {
            for (acc, &w) in class.iter_mut().zip(words.iter()) {
                *acc |= w;
            }
        }
        s.stored_slack.push(slack);
        s.stored_skip.push(s.row_skip[row]);
        ref_rows = ref_rows.saturating_add(upto(missing, slack as usize));
        stored += 1;
    }
    if stored == 0 {
        return BuildOutcome::Empty;
    }
    if ref_rows > caps.max_rows as u64 {
        return BuildOutcome::Oversize;
    }
    // Reference columns: each surviving monomial counts once per
    // non-occurring multiplier its largest-slack row admits.
    let mut ref_cols = 0u64;
    let mut covered = vec![0u64; stride];
    for class in (0..classes).rev() {
        let mask = &s.class_or[class * stride..(class + 1) * stride];
        let mut fresh = 0u64;
        for (cov, &m) in covered.iter_mut().zip(mask) {
            fresh += (m & !*cov).count_ones() as u64;
            *cov |= m;
        }
        ref_cols = ref_cols.saturating_add(fresh.saturating_mul(upto(missing, class)));
    }
    if ref_cols > caps.max_cols as u64 {
        return BuildOutcome::Oversize;
    }
    BuildOutcome::Built(Built {
        rows: stored,
        cols: n_cols,
        stride,
        low_start,
        ref_rows,
        ref_cols,
    })
}

/// Exact reference row count for the rare case its bound exceeds the cap:
/// forms each candidate row's monomials, drops those cancelled in pairs,
/// and stops as soon as the count passes `max_rows`.
fn exact_reference_rows_exceed(
    generating: &[(&F2BoolPoly, u32)],
    layout: &Layout,
    degree: u32,
    max_rows: u64,
    s: &mut Scratch,
) -> bool {
    let mut count = 0u64;
    let mut exceeded = false;
    for &(p, pdeg) in generating {
        let k = (degree - pdeg) as usize;
        for_each_multiplier(&layout.bits, k, |u, du| {
            if exceeded {
                return;
            }
            s.row_buf.clear();
            s.row_buf.extend(p.terms.iter().map(|t| t.mask | u));
            s.row_buf.sort_unstable();
            let mut i = 0;
            let mut nonempty = false;
            while i < s.row_buf.len() {
                let mut j = i;
                while j < s.row_buf.len() && s.row_buf[j] == s.row_buf[i] {
                    j += 1;
                }
                if (j - i) % 2 == 1 {
                    nonempty = true;
                    break;
                }
                i = j;
            }
            if nonempty {
                count = count.saturating_add(upto(layout.missing, k - du));
                exceeded = count > max_rows;
            }
        });
        if exceeded {
            return true;
        }
    }
    false
}

/// Exact reference column count for the rare case its bound exceeds the
/// cap, from the candidate rows already in the scratch.
fn exact_reference_cols_exceed(missing: usize, caps: MacaulayCaps, s: &mut Scratch) -> bool {
    // Largest slack of a row in which each distinct monomial survives.
    let mut best: Vec<i16> = vec![-1; s.distinct.len()];
    let use_rank_keys = !s.distinct.is_empty() && s.hash.is_empty();
    let candidates = s.row_slack.len();
    for row in 0..candidates {
        let (lo, hi) = (s.row_start[row] as usize, s.row_start[row + 1] as usize);
        s.row_buf.clear();
        s.row_buf
            .extend(s.prov[lo..hi].iter().map(|&k| u64::from(k)));
        s.row_buf.sort_unstable();
        let slack = s.row_slack[row] as i16;
        let mut i = 0;
        while i < s.row_buf.len() {
            let mut j = i;
            while j < s.row_buf.len() && s.row_buf[j] == s.row_buf[i] {
                j += 1;
            }
            if (j - i) % 2 == 1 {
                let key = s.row_buf[i] as usize;
                let di = if use_rank_keys {
                    s.remap[key] as usize
                } else {
                    key
                };
                best[di] = best[di].max(slack);
            }
            i = j;
        }
    }
    let total = best.iter().filter(|&&b| b >= 0).fold(0u64, |acc, &b| {
        acc.saturating_add(upto(missing, b as usize))
    });
    total > caps.max_cols as u64
}

/// Leading column of `row` at or after word `from`.
#[inline]
fn lead_from(row: &[u64], from: usize) -> Option<usize> {
    row[from..]
        .iter()
        .position(|&w| w != 0)
        .map(|i| (from + i) * 64 + row[from + i].trailing_zeros() as usize)
}

/// `rows[dst] ^= rows[src]` over words `from..stride`.
#[inline]
fn xor_row_suffix(m: &mut [u64], stride: usize, dst: usize, src: usize, from: usize) {
    debug_assert_ne!(dst, src);
    if dst < src {
        let (a, b) = m.split_at_mut(src * stride);
        let d = &mut a[dst * stride + from..dst * stride + stride];
        let p = &b[from..stride];
        for (x, &y) in d.iter_mut().zip(p) {
            *x ^= y;
        }
    } else {
        let (a, b) = m.split_at_mut(dst * stride);
        let p = &a[src * stride + from..src * stride + stride];
        let d = &mut b[from..stride];
        for (x, &y) in d.iter_mut().zip(p) {
            *x ^= y;
        }
    }
}

/// `rows[dst] ^= rows[src]` over words `from..stride`, then the leading
/// column of the result at or after word `from`.
///
/// The rows here are a few words long, so bounds checks and slice
/// splitting cost as much as the XOR; one assertion covers both rows.
#[inline]
fn xor_then_lead(
    m: &mut [u64],
    stride: usize,
    dst: usize,
    src: usize,
    from: usize,
) -> Option<usize> {
    assert!(dst != src && dst.max(src) * stride + stride <= m.len() && from <= stride);
    let base = m.as_mut_ptr();
    // SAFETY: both rows lie inside `m` (asserted) and are distinct, so the
    // ranges `dst*stride..` and `src*stride..` of length `stride` are
    // disjoint; every access is at an offset in `from..stride`.
    unsafe {
        let d = base.add(dst * stride);
        let p = base.add(src * stride).cast_const();
        for w in from..stride {
            *d.add(w) ^= *p.add(w);
        }
        for w in from..stride {
            let v = *d.add(w);
            if v != 0 {
                return Some(w * 64 + v.trailing_zeros() as usize);
            }
        }
    }
    None
}

/// Forward elimination over columns `0 .. stop`.
///
/// Rows are bucketed by leading column, so a column's pivot is found
/// without scanning and only the rows that actually lead there are
/// touched.  Returns the rank; `s.pivots` holds the pivot rows in column
/// order and `s.low_rows` the nonzero rows whose leading column is at or
/// past `stop`.
fn echelon(m: &mut [u64], rows: usize, stride: usize, stop: usize, s: &mut Scratch) -> u64 {
    const NIL: u32 = u32::MAX;
    let mut ops = 0u64;
    s.head.clear();
    s.head.resize(stop, NIL);
    s.next.clear();
    s.next.resize(rows, NIL);
    s.pivots.clear();
    s.low_rows.clear();
    for r in 0..rows {
        if s.stored_skip.get(r).copied().unwrap_or(false) {
            continue;
        }
        match lead_from(&m[r * stride..(r + 1) * stride], 0) {
            Some(c) if c < stop => {
                s.next[r] = s.head[c];
                s.head[c] = r as u32;
            }
            Some(_) => s.low_rows.push(r as u32),
            None => {}
        }
    }
    for c in 0..stop {
        let first = s.head[c];
        if first == NIL {
            continue;
        }
        let piv = first as usize;
        s.pivots.push(first);
        let from = c / 64;
        let mut r = s.next[piv];
        while r != NIL {
            let row = r as usize;
            let after = s.next[row];
            ops += (stride - from) as u64;
            match xor_then_lead(m, stride, row, piv, from) {
                Some(c2) if c2 < stop => {
                    s.next[row] = s.head[c2];
                    s.head[c2] = r;
                }
                Some(_) => s.low_rows.push(r),
                None => {}
            }
            r = after;
        }
    }
    ops
}

/// Eliminate the built matrix over columns `..low_start` and leave in
/// `s.low` the linear blocks of the rows reduced to zero there, the zero
/// ones dropped; returns the number of pivots and the word XORs.  A matrix
/// of at least `F4_F2_ECHELON_MIN_WORDS` words goes to a CUDA device when
/// one opens, unless `F4_F2_ECHELON` says otherwise
/// ([`f4_gpu_stub::offload_echelon`]); it eliminates with other pivots,
/// so only the word count can differ.
fn eliminate_high(
    s: &mut Scratch,
    rows: usize,
    stride: usize,
    low_start: usize,
    width: usize,
) -> (usize, u64) {
    if let Some(e) = f4_gpu_stub::offload_echelon(
        &s.matrix[..rows * stride],
        rows,
        stride,
        low_start,
        width,
        &s.stored_skip,
    ) {
        s.low = e.low;
        return (e.pivots, e.word_ops);
    }
    let mut m = std::mem::take(&mut s.matrix);
    let ops = echelon(&mut m, rows, stride, low_start, s);
    s.low.clear();
    for &r in &s.low_rows {
        let r = r as usize;
        s.low.push(extract_bits(
            &m[r * stride..(r + 1) * stride],
            low_start,
            width,
        ));
    }
    s.matrix = m;
    (s.pivots.len(), ops)
}

/// Bits `start .. start + width` of `row` (`width ≤ 128`).
fn extract_bits(row: &[u64], start: usize, width: usize) -> u128 {
    let mut out = 0u128;
    let mut got = 0usize;
    while got < width {
        let bit = start + got;
        let w = bit / 64;
        let off = bit % 64;
        let take = (64 - off).min(width - got);
        let chunk = (row[w] >> off)
            & if take == 64 {
                u64::MAX
            } else {
                (1u64 << take) - 1
            };
        out |= u128::from(chunk) << got;
        got += take;
    }
    out
}

/// Reduced echelon form of `rows` (bit `j` = column `j`), pivots in
/// increasing column order; returns the rank, the pivot rows first.
fn rref_u128(rows: &mut [u128], width: usize) -> usize {
    let mut rank = 0usize;
    for col in 0..width {
        let bit = 1u128 << col;
        let Some(p) = (rank..rows.len()).find(|&i| rows[i] & bit != 0) else {
            continue;
        };
        rows.swap(rank, p);
        let pivot = rows[rank];
        for (i, row) in rows.iter_mut().enumerate() {
            if i != rank && *row & bit != 0 {
                *row ^= pivot;
            }
        }
        rank += 1;
        if rank == rows.len() {
            break;
        }
    }
    rank
}

/// The rows the F5 plan leaves out of the degree-`degree` matrix of
/// `polys` over its occurring variables, as a bitmask in the GPU kernel's
/// candidate order: the generating equations (nonzero, degree at most
/// `degree`) in order, and within each its multipliers of degree `0, 1, …`,
/// each degree in colex order — ascending integer order of the multiplier's
/// mask over the compacted variables.  `None` when the plan is empty.
///
/// This is the host half of F5 on the device: the symbolic preprocessing
/// picks the rows, the kernel eliminates only the rest.
pub fn f5_row_mask(polys: &[F2BoolPoly], n_vars: usize, degree: u32) -> Option<Vec<u32>> {
    let generating: Vec<(&F2BoolPoly, u32)> = polys
        .iter()
        .filter(|p| !p.is_zero())
        .map(|p| (p, poly_degree(p)))
        .filter(|&(_, d)| d <= degree)
        .collect();
    let occurring = generating
        .iter()
        .flat_map(|(p, _)| p.terms.iter())
        .fold(0u64, |acc, t| acc | t.mask);
    let layout = Layout::new(occurring, n_vars, degree);
    let plan = F5Plan::new(&generating, &layout, degree)?;
    let v = layout.bits.len();
    let mut mask: Vec<u32> = Vec::new();
    let mut row = 0usize;
    let mut any = false;
    for (i, &(_, d)) in generating.iter().enumerate() {
        let k = (degree - d) as usize;
        for j in 0..=k.min(v) {
            // j-subsets of the v compact variables, ascending: Gosper's hack.
            let mut c: u128 = (1u128 << j) - 1;
            while c < (1u128 << v) {
                let mut u = 0u64;
                let mut rest = c;
                while rest != 0 {
                    u |= layout.bits[rest.trailing_zeros() as usize];
                    rest &= rest - 1;
                }
                if mask.len() <= row / 32 {
                    mask.push(0);
                }
                if plan.skips(&layout, degree, i, d, u) {
                    mask[row / 32] |= 1 << (row % 32);
                    any = true;
                }
                row += 1;
                if j == 0 {
                    break;
                }
                // `isolate_lowest_one` is not on every toolchain CI runs.
                #[allow(clippy::manual_isolate_lowest_one)]
                let low = c & c.wrapping_neg();
                let ripple = c + low;
                c = (((ripple ^ c) >> 2) / low) | ripple;
            }
        }
    }
    any.then_some(mask)
}

/// Default caps: the reference builder's, with the same environment
/// overrides (read once per call rather than once per row).
pub fn default_caps() -> MacaulayCaps {
    let read = |key: &str, default: usize| {
        std::env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    };
    MacaulayCaps {
        max_rows: read("F4_F2_MAX_ROWS", 20_000),
        max_cols: read("F4_F2_MAX_COLS", 40_000),
    }
}

/// **Decide** what the degree-`degree` Macaulay matrix of `polys` says:
/// refutation and forced assignments, exactly as the reference reduced
/// rows would.  `None` when the matrix exceeds `caps` (in reference
/// units).
///
/// An empty `polys` builds nothing and reports `built = false`, as the
/// reference does.  Uses [`default_options`].
pub fn decide(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    caps: MacaulayCaps,
) -> (Option<Decision>, KernelCounters) {
    decide_with(polys, n_vars, degree, caps, default_options())
}

/// [`decide`] with explicit [`KernelOptions`].
pub fn decide_with(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    caps: MacaulayCaps,
    opts: KernelOptions,
) -> (Option<Decision>, KernelCounters) {
    let mut k = KernelCounters::default();
    if polys.is_empty() {
        return (Some(Decision::default()), k);
    }
    SCRATCH.with(|cell| {
        let s = &mut *cell.borrow_mut();
        let t0 = std::time::Instant::now();
        let outcome = build(polys, n_vars, degree, true, caps, opts, s);
        k.build_ns = t0.elapsed().as_nanos();
        let b = match outcome {
            BuildOutcome::Oversize => {
                k.oversize = true;
                return (None, k);
            }
            BuildOutcome::Empty => {
                k.built = true;
                return (Some(Decision::default()), k);
            }
            BuildOutcome::Built(b) => b,
        };
        k.built = true;
        k.rows = b.ref_rows;
        k.cols = b.ref_cols;
        k.f5_skipped = s.stored_skip.iter().filter(|&&x| x).count() as u64;
        k.eliminated_rows = b.rows as u64 - k.f5_skipped;
        k.eliminated_cols = b.cols as u64;

        let t1 = std::time::Instant::now();
        let width = b.cols - b.low_start;
        let (pivots, ops) = eliminate_high(s, b.rows, b.stride, b.low_start, width);
        k.word_ops = ops;
        let rank = rref_u128(&mut s.low, width);
        k.rank = (pivots + rank) as u64;
        k.reduce_ns = t1.elapsed().as_nanos();

        let t2 = std::time::Instant::now();
        let low_masks = &s.col_mask[b.low_start..];
        let const_bit = match low_masks.last() {
            Some(&0) => 1u128 << (width - 1),
            _ => 0,
        };
        let mut d = Decision::default();
        for &row in &s.low[..rank] {
            if const_bit != 0 && row == const_bit {
                d.refuted = true;
                continue;
            }
            let vars = row & !const_bit;
            if vars.count_ones() == 1 {
                let var = low_masks[vars.trailing_zeros() as usize].trailing_zeros();
                d.forced.push((var, row & const_bit != 0));
            }
        }
        if d.refuted {
            d.forced.clear();
        }
        k.readback_ns = t2.elapsed().as_nanos();
        (Some(d), k)
    })
}

/// Rank and linear consequences of one Macaulay matrix, for the
/// solving-degree and first-fall-degree sweeps.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Profile {
    /// Rank of the reference matrix (all `n_vars` variables).
    pub rank: u64,
    /// The row space contains `1`.
    pub refuted: bool,
    /// Rows of the reduced echelon form of the form `v` or `v + 1`, in
    /// variable order — reported even alongside a refutation, as the
    /// reference reduction has them.
    pub forced: Vec<(u32, bool)>,
}

/// **Profile** the reference Macaulay matrix of `polys` at `degree` —
/// over all `n_vars` variables, so its rank and its linear rows are the
/// reference reduction's exactly — by forward elimination over the high
/// columns and a reduction of the linear block.  `None` over `caps`.
pub fn profile_with(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    caps: MacaulayCaps,
    opts: KernelOptions,
) -> (Option<Profile>, KernelCounters) {
    let mut k = KernelCounters::default();
    if polys.is_empty() {
        return (Some(Profile::default()), k);
    }
    SCRATCH.with(|cell| {
        let s = &mut *cell.borrow_mut();
        let t0 = std::time::Instant::now();
        let outcome = build(polys, n_vars, degree, false, caps, opts, s);
        k.build_ns = t0.elapsed().as_nanos();
        let b = match outcome {
            BuildOutcome::Oversize => {
                k.oversize = true;
                return (None, k);
            }
            BuildOutcome::Empty => {
                k.built = true;
                return (Some(Profile::default()), k);
            }
            BuildOutcome::Built(b) => b,
        };
        k.built = true;
        k.rows = b.ref_rows;
        k.cols = b.ref_cols;
        k.f5_skipped = s.stored_skip.iter().filter(|&&x| x).count() as u64;
        k.eliminated_rows = b.rows as u64 - k.f5_skipped;
        k.eliminated_cols = b.cols as u64;
        let t1 = std::time::Instant::now();
        // Over all variables the linear block can exceed 65 columns only by
        // variables the matrix never touches; it is at most n_vars + 1 ≤ 65.
        let width = b.cols - b.low_start;
        let (pivots, ops) = eliminate_high(s, b.rows, b.stride, b.low_start, width);
        k.word_ops = ops;
        let low_rank = rref_u128(&mut s.low, width);
        k.rank = (pivots + low_rank) as u64;
        k.reduce_ns = t1.elapsed().as_nanos();
        let low_masks = &s.col_mask[b.low_start..];
        let const_bit = match low_masks.last() {
            Some(&0) => 1u128 << (width - 1),
            _ => 0,
        };
        let mut p = Profile {
            rank: k.rank,
            ..Profile::default()
        };
        for &row in &s.low[..low_rank] {
            if const_bit != 0 && row == const_bit {
                p.refuted = true;
                continue;
            }
            let vars = row & !const_bit;
            if vars.count_ones() == 1 {
                let var = low_masks[vars.trailing_zeros() as usize].trailing_zeros();
                p.forced.push((var, row & const_bit != 0));
            }
        }
        (Some(p), k)
    })
}

/// **The reference reduced rows**, bit-for-bit: the full reduced echelon
/// form of the degree-`degree` Macaulay matrix of `polys` over all
/// `n_vars` variables, rows in pivot order, each a polynomial with its
/// terms in descending monomial order.  `None` when the matrix exceeds
/// `caps`.  Uses [`default_options`].
pub fn matrix_rows(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    caps: MacaulayCaps,
) -> (Option<Vec<F2BoolPoly>>, KernelCounters) {
    matrix_rows_with(polys, n_vars, degree, caps, default_options())
}

/// [`matrix_rows`] with explicit [`KernelOptions`].
pub fn matrix_rows_with(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    caps: MacaulayCaps,
    opts: KernelOptions,
) -> (Option<Vec<F2BoolPoly>>, KernelCounters) {
    let mut k = KernelCounters::default();
    if polys.is_empty() {
        return (Some(Vec::new()), k);
    }
    let n_vars_out = polys[0].n_vars;
    SCRATCH.with(|cell| {
        let s = &mut *cell.borrow_mut();
        let t0 = std::time::Instant::now();
        let outcome = build(polys, n_vars, degree, false, caps, opts, s);
        k.build_ns = t0.elapsed().as_nanos();
        let b = match outcome {
            BuildOutcome::Oversize => {
                k.oversize = true;
                return (None, k);
            }
            BuildOutcome::Empty => {
                k.built = true;
                return (Some(Vec::new()), k);
            }
            BuildOutcome::Built(b) => b,
        };
        k.built = true;
        k.rows = b.ref_rows;
        k.cols = b.ref_cols;
        k.f5_skipped = s.stored_skip.iter().filter(|&&x| x).count() as u64;
        k.eliminated_rows = b.rows as u64 - k.f5_skipped;
        k.eliminated_cols = b.cols as u64;

        let t1 = std::time::Instant::now();
        let mut m = std::mem::take(&mut s.matrix);
        let mut ops = echelon(&mut m, b.rows, b.stride, b.cols, s);
        // Back substitution, last pivot first, so every pivot row is
        // already clear of the later pivot columns when it is used.
        let pivots = std::mem::take(&mut s.pivots);
        let stride = b.stride;
        let lead: Vec<usize> = pivots
            .iter()
            .map(|&r| {
                let r = r as usize;
                lead_from(&m[r * stride..(r + 1) * stride], 0).expect("pivot rows are nonzero")
            })
            .collect();
        for i in (0..pivots.len()).rev() {
            let (src, col) = (pivots[i] as usize, lead[i]);
            let (w, bit) = (col / 64, 1u64 << (col % 64));
            for &dst in &pivots[..i] {
                let dst = dst as usize;
                if m[dst * stride + w] & bit != 0 {
                    xor_row_suffix(&mut m, stride, dst, src, w);
                    ops += (stride - w) as u64;
                }
            }
        }
        k.word_ops = ops;
        k.rank = pivots.len() as u64;
        k.reduce_ns = t1.elapsed().as_nanos();

        let t2 = std::time::Instant::now();
        let mut out = Vec::with_capacity(pivots.len());
        for &r in &pivots {
            let r = r as usize;
            let mut terms = Vec::new();
            for (w, &word) in m[r * stride..(r + 1) * stride].iter().enumerate() {
                let mut bits = word;
                while bits != 0 {
                    let c = w * 64 + bits.trailing_zeros() as usize;
                    terms.push(F2BoolMono::from_mask(s.col_mask[c]));
                    bits &= bits - 1;
                }
            }
            out.push(F2BoolPoly {
                terms,
                n_vars: n_vars_out,
            });
        }
        s.pivots = pivots;
        s.matrix = m;
        k.readback_ns = t2.elapsed().as_nanos();
        (Some(out), k)
    })
}

/// Host-only stand-ins for the suite's `f4_gpu` offload hooks.
///
/// The sibling suite offloads large echelons to a CUDA device; this crate
/// has no device backend, so the hooks decline and the host path runs.
/// Port 3 of `docs/ic/STACK_DIVERGENCE_20261018.md` (the GPU echelon)
/// replaces this module with the real `f4_gpu`; the call sites above keep
/// the suite's signatures so that sync is a diff.
mod f4_gpu_stub {
    /// What a device echelon hands back: pivots taken in the high columns,
    /// the linear blocks of the rows reduced to zero there, and the word
    /// XORs performed.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Echelon {
        pub pivots: usize,
        pub low: Vec<u128>,
        pub word_ops: u64,
    }

    /// A device elimination is expected for a matrix of `words` words.
    /// Never, on the host.
    pub fn offload_expected(_words: u64) -> bool {
        false
    }

    /// Eliminate on a device.  `None`: leave it to the host.
    pub fn offload_echelon(
        _matrix: &[u64],
        _rows: usize,
        _stride: usize,
        _low_start: usize,
        _width: usize,
        _skip: &[bool],
    ) -> Option<Echelon> {
        None
    }

    /// Emulated device elimination (tests of the device kernels).  `None`.
    #[allow(dead_code)]
    pub fn emulate_echelon(
        _matrix: &[u64],
        _rows: usize,
        _stride: usize,
        _low_start: usize,
        _width: usize,
        _skip: &[bool],
        _threads: u32,
    ) -> Option<Echelon> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use crate::cryptanalysis::koblitz_bench::ladder_sample;
    use crate::cryptanalysis::koblitz_groebner::{
        build_decomposition_system, f4_profile, f4_profile_reset, matrix_f4_f2,
        matrix_f4_f2_counted, matrix_f4_f2_counted_with, matrix_f4_f2_solver_consequences,
        FieldStructure, RowCriterion,
    };
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::time::Instant;

    const CAPS: MacaulayCaps = MacaulayCaps {
        max_rows: 20_000,
        max_cols: 40_000,
    };

    fn is_constant_one(p: &F2BoolPoly) -> bool {
        p.terms.len() == 1 && p.terms[0].mask == 0
    }

    fn forced_assignment(p: &F2BoolPoly) -> Option<(u32, bool)> {
        let (mut var, mut has_const) = (None, false);
        for t in &p.terms {
            if t.mask == 0 {
                has_const = true;
            } else if t.mask.count_ones() == 1 && var.is_none() {
                var = Some(t.mask.trailing_zeros());
            } else {
                return None;
            }
        }
        var.map(|v| (v, has_const))
    }

    /// What the splitting solver reads off the crate kernel's rows: the
    /// refutation, and, only when there is none, the forced assignments.
    fn decision_of(rows: &[F2BoolPoly]) -> Decision {
        let refuted = rows.iter().any(is_constant_one);
        Decision {
            refuted,
            forced: if refuted {
                Vec::new()
            } else {
                rows.iter().filter_map(forced_assignment).collect()
            },
        }
    }

    /// `count` Weil-descended decomposition systems of the cell `(n, ell, m)`.
    fn ladder_systems(
        n: u32,
        ell: usize,
        m: usize,
        count: usize,
        seed: u64,
    ) -> Vec<(Vec<F2BoolPoly>, usize)> {
        let irr = find_irreducible_sparse(n).expect("irreducible");
        let st = FieldStructure::new(n, &irr);
        let b = F2mElement::one(n);
        let mut rng = StdRng::seed_from_u64(seed);
        let mut out = Vec::new();
        while out.len() < count {
            let (basis, x_r) = ladder_sample(n, ell, &mut rng);
            if let Some(sys) = build_decomposition_system(&basis, &x_r, &b, m, &st) {
                out.push((sys.equations, sys.n_vars));
            }
        }
        out
    }

    /// `count` random dense systems: every monomial of degree at most
    /// `max_deg` joins each equation with probability 1/4.
    fn dense_systems(
        n_vars: usize,
        n_eqs: usize,
        max_deg: u32,
        count: usize,
        seed: u64,
    ) -> Vec<(Vec<F2BoolPoly>, usize)> {
        let mut rng = StdRng::seed_from_u64(seed);
        let monos: Vec<u64> = (0u64..(1 << n_vars))
            .filter(|m| m.count_ones() <= max_deg)
            .collect();
        (0..count)
            .map(|_| {
                let eqs = (0..n_eqs)
                    .map(|_| {
                        let terms: Vec<F2BoolMono> = monos
                            .iter()
                            .filter(|_| rng.gen::<u32>() % 4 == 0)
                            .map(|&m| F2BoolMono::from_mask(m))
                            .collect();
                        F2BoolPoly::from_monos(terms, n_vars)
                    })
                    .filter(|p| !p.is_zero())
                    .collect();
                (eqs, n_vars)
            })
            .collect()
    }

    /// The ported kernel against the crate's `matrix_f4_f2` on one system:
    /// decision, reduced rows (bit-for-bit), the solver path's decision,
    /// and F5 on and off.  `true` when a matrix was built.
    fn agree(polys: &[F2BoolPoly], n_vars: usize, degree: u32) -> bool {
        let reference = matrix_f4_f2(polys, n_vars, degree);
        let off = KernelOptions { f5: false };
        let on = KernelOptions { f5: true };
        let (rows, _) = matrix_rows_with(polys, n_vars, degree, CAPS, off);
        let (rows_f5, _) = matrix_rows_with(polys, n_vars, degree, CAPS, on);
        let (d_off, _) = decide_with(polys, n_vars, degree, CAPS, off);
        let (d_on, _) = decide_with(polys, n_vars, degree, CAPS, on);
        let Some(ref_rows) = reference else {
            assert!(rows.is_none() && rows_f5.is_none());
            assert!(d_off.is_none() && d_on.is_none());
            return false;
        };
        assert_eq!(rows.as_ref(), Some(&ref_rows), "rows, degree {degree}");
        assert_eq!(
            rows_f5.as_ref(),
            Some(&ref_rows),
            "F5 rows, degree {degree}"
        );
        let want = decision_of(&ref_rows);
        assert_eq!(d_off.as_ref(), Some(&want), "decision, degree {degree}");
        assert_eq!(d_on.as_ref(), Some(&want), "F5 decision, degree {degree}");
        // The crate's solver-specialised path reads the same facts.
        let solver = matrix_f4_f2_solver_consequences(polys, n_vars, degree, RowCriterion::None)
            .expect("within the caps");
        assert_eq!(decision_of(&solver), want, "solver path, degree {degree}");
        true
    }

    #[test]
    fn f4_gf2_agrees_on_koblitz_decomposition_systems() {
        // Cells (n, ell), m = 3, 20 draws each: (6, 1) and (9, 1) force
        // variables and refute; (8, 2) and (7, 2) are mostly undecided at
        // degree 3 and 4, so "no decision" is compared too.
        let (mut refuted, mut forced, mut undecided) = (0, 0, 0);
        for (cell, &(n, ell)) in [(6u32, 1usize), (9, 1), (8, 2), (7, 2)].iter().enumerate() {
            for degree in [3u32, 4] {
                let seed = 0xF4_6F2_00 + 16 * cell as u64 + u64::from(degree);
                let mut built = 0;
                for (polys, n_vars) in ladder_systems(n, ell, 3, 20, seed) {
                    assert!(agree(&polys, n_vars, degree), "oversize");
                    built += 1;
                    let d = decide(&polys, n_vars, degree, CAPS).0.unwrap();
                    refuted += usize::from(d.refuted);
                    forced += usize::from(!d.forced.is_empty());
                    undecided += usize::from(!d.is_decisive());
                }
                assert_eq!(built, 20);
            }
        }
        eprintln!("f4_gf2 ladder agreement: {refuted} refuted, {forced} forced, {undecided} undecided of 160");
        assert!(refuted > 0 && forced > 0 && undecided > 0);
    }

    #[test]
    fn f4_gf2_agrees_on_random_dense_systems() {
        for degree in [3u32, 4] {
            let (mut built, mut decisive) = (0, 0);
            for (polys, n_vars) in dense_systems(9, 8, 2, 20, 0xDE_5E00 + u64::from(degree)) {
                if polys.is_empty() {
                    continue;
                }
                if agree(&polys, n_vars, degree) {
                    built += 1;
                    decisive += usize::from(
                        decide(&polys, n_vars, degree, CAPS)
                            .0
                            .unwrap()
                            .is_decisive(),
                    );
                }
            }
            assert!(built >= 15, "degree {degree}: only {built} built");
            eprintln!(
                "f4_gf2 dense agreement: degree {degree}: {built} built, {decisive} decisive"
            );
        }
    }

    /// F5 pruning on or off gives the same decision, and on the cubic
    /// ladder at degree 4 it leaves rows out.
    #[test]
    fn f4_gf2_f5_on_and_off_agree() {
        let mut skipped = 0;
        for (polys, n_vars) in ladder_systems(7, 2, 3, 6, 0xF5_0FF) {
            for degree in [3u32, 4] {
                let (d_on, k_on) =
                    decide_with(&polys, n_vars, degree, CAPS, KernelOptions { f5: true });
                let (d_off, k_off) =
                    decide_with(&polys, n_vars, degree, CAPS, KernelOptions { f5: false });
                assert_eq!(d_on, d_off, "degree {degree}");
                assert_eq!(k_off.f5_skipped, 0);
                assert!(k_on.word_ops > 0 || k_on.eliminated_rows == 0);
                skipped += k_on.f5_skipped;
            }
        }
        eprintln!("f4_gf2 F5 pruning left out {skipped} rows");
    }

    #[test]
    fn f4_gf2_empty_and_trivial_inputs() {
        let (d, k) = decide(&[], 4, 3, CAPS);
        assert_eq!(d, Some(Decision::default()));
        assert!(!k.built);
        let one = F2BoolPoly::one(3);
        assert!(
            decide(std::slice::from_ref(&one), 3, 2, CAPS)
                .0
                .unwrap()
                .refuted
        );
        let v = F2BoolPoly::from_monos(vec![F2BoolMono::var(2), F2BoolMono::one()], 3);
        let (d, _) = decide(std::slice::from_ref(&v), 3, 2, CAPS);
        assert_eq!(d.unwrap().forced, vec![(2, true)]);
        assert!(f5_row_mask(&[], 3, 2).is_none());
    }

    /// Always-on, small: the same-binary comparison on three draws.  Both
    /// kernels give one decision, both report a nonzero XOR count, and the
    /// ported kernel's count is not above the crate's.
    #[test]
    fn f4_gf2_comparison_smoke() {
        for (polys, n_vars) in ladder_systems(7, 2, 3, 3, 0x5E0_4E) {
            let (rows, crate_xors) = matrix_f4_f2_counted(&polys, n_vars, 3).unwrap();
            let (d, k) = decide(&polys, n_vars, 3, CAPS);
            assert_eq!(d.unwrap(), decision_of(&rows));
            assert!(
                crate_xors > 0 && k.word_ops > 0,
                "{crate_xors} / {}",
                k.word_ops
            );
            assert!(k.word_ops <= crate_xors, "{} > {crate_xors}", k.word_ops);
        }
    }

    fn med(v: &mut [f64]) -> f64 {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = v.len();
        if n % 2 == 1 {
            v[n / 2]
        } else {
            (v[n / 2 - 1] + v[n / 2]) / 2.0
        }
    }

    fn envn<T: std::str::FromStr>(key: &str, default: T) -> T {
        std::env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Same-binary comparison, 20 draws.  Run:
    ///
    /// ```text
    /// CARGO_PROFILE_RELEASE_DEBUG=0 cargo test --release --lib -- \
    ///     f4_gf2_same_binary_comparison --ignored --nocapture --test-threads=1
    /// ```
    ///
    /// Cell and degree by `F4_GF2_CMP_N`, `_ELL`, `_M`, `_DEGREE`, `_DRAWS`.
    /// `--test-threads=1` is needed: the crate's XOR counter for the solver
    /// path is a process-wide atomic.
    #[test]
    #[ignore]
    fn f4_gf2_same_binary_comparison() {
        let n: u32 = envn("F4_GF2_CMP_N", 15);
        let ell: usize = envn("F4_GF2_CMP_ELL", 5);
        let m: usize = envn("F4_GF2_CMP_M", 3);
        let degree: u32 = envn("F4_GF2_CMP_DEGREE", 4);
        let draws: usize = envn("F4_GF2_CMP_DRAWS", 20);
        let systems = ladder_systems(n, ell, m, draws, 0xC0_4A5E);
        let n_vars = systems[0].1;
        let ms = |t: Instant| t.elapsed().as_secs_f64() * 1e3;
        // arms: 0 crate full (matrix_f4_f2_counted), 1 crate solver path,
        // 2 port F5 on, 3 port F5 off
        let names = ["crate full", "crate solver", "port F5 on", "port F5 off"];
        let mut t = [(); 4].map(|_| Vec::<f64>::new());
        let mut x = [(); 4].map(|_| Vec::<f64>::new());
        let (mut refuted, mut decisive) = (0usize, 0usize);
        println!("cell n={n} ell={ell} m={m} n_vars={n_vars} degree={degree} draws={draws}");
        println!(
            "draw  {:>12} {:>12} {:>12} {:>12}   (ms)",
            names[0], names[1], names[2], names[3]
        );
        // Warm-up: one untimed pass so thread-local scratch and caches exist.
        let _ = decide(&systems[0].0, n_vars, degree, CAPS);
        let _ = matrix_f4_f2_counted(&systems[0].0, n_vars, degree);
        for (i, (polys, nv)) in systems.iter().enumerate() {
            let mut row = [0f64; 4];
            let mut dec: [Option<Decision>; 4] = [None, None, None, None];
            // Alternate the order by draw to cancel position effects.
            let order: [usize; 4] = if i % 2 == 0 {
                [0, 1, 2, 3]
            } else {
                [3, 2, 1, 0]
            };
            for arm in order {
                match arm {
                    0 => {
                        let s = Instant::now();
                        let (rows, w) =
                            matrix_f4_f2_counted_with(polys, *nv, degree, RowCriterion::None)
                                .expect("within caps");
                        row[0] = ms(s);
                        x[0].push(w as f64);
                        dec[0] = Some(decision_of(&rows));
                    }
                    1 => {
                        f4_profile_reset();
                        let s = Instant::now();
                        let rows = matrix_f4_f2_solver_consequences(
                            polys,
                            *nv,
                            degree,
                            RowCriterion::None,
                        )
                        .expect("within caps");
                        row[1] = ms(s);
                        x[1].push(f4_profile().word_ops as f64);
                        dec[1] = Some(decision_of(&rows));
                    }
                    _ => {
                        let s = Instant::now();
                        let (d, k) =
                            decide_with(polys, *nv, degree, CAPS, KernelOptions { f5: arm == 2 });
                        row[arm] = ms(s);
                        x[arm].push(k.word_ops as f64);
                        dec[arm] = d;
                    }
                }
            }
            assert!(
                dec.iter().all(|d| d == &dec[0]),
                "draw {i}: decisions differ"
            );
            let d0 = dec[0].as_ref().expect("within caps");
            refuted += usize::from(d0.refuted);
            decisive += usize::from(d0.is_decisive());
            for a in 0..4 {
                t[a].push(row[a]);
            }
            println!(
                "{i:>4}  {:>12.3} {:>12.3} {:>12.3} {:>12.3}",
                row[0], row[1], row[2], row[3]
            );
        }
        println!("decisions: {refuted} refuted, {decisive} decisive of {draws}");
        println!(
            "\n{:<14} {:>10} {:>10} {:>10} {:>10} {:>14} {:>14}",
            "arm", "total ms", "median", "min", "max", "median xors", "speedup(med)"
        );
        let base = med(&mut t[0].clone());
        for a in 0..4 {
            let tot: f64 = t[a].iter().sum();
            let mut s = t[a].clone();
            let md = med(&mut s);
            println!(
                "{:<14} {:>10.1} {:>10.3} {:>10.3} {:>10.3} {:>14.0} {:>13.2}x",
                names[a],
                tot,
                md,
                s[0],
                s[s.len() - 1],
                med(&mut x[a].clone()),
                base / md
            );
        }
    }
}
