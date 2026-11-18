//! # F4 over the boolean ring `F_2[v_0, …, v_{n−1}] / (v_i² − v_i)`.
//!
//! Faugère's F4 (1999) is Buchberger's algorithm with the reductions
//! batched: at each step it takes **every** critical pair of the lowest
//! degree, writes the two halves of each S-polynomial as rows of one
//! matrix, adds a reducer row for every monomial some basis element's
//! leading monomial divides (*symbolic preprocessing*), and reduces the
//! whole matrix at once.  Rows whose pivot is a monomial no basis
//! element leads join the basis.  Over `F_2` the matrix is bit-packed,
//! sixty-four columns to a word, so a reduction step is a run of 64-bit
//! XORs — the unit this module counts.
//!
//! [`pq_groebner_f2`](crate::cryptanalysis::pq_groebner_f2) reduces the
//! same pairs one at a time on sorted monomial lists.  This module is the
//! batched version of the same computation, and a test holds it to that:
//! wherever the Buchberger output is certified to be a Gröbner basis of
//! the boolean ideal, the two reduced bases are identical.
//!
//! ## The field pairs, which the boolean ring needs
//!
//! In `F_2[v]/(v² − v)` a Gröbner basis must also be closed under the
//! products `v_i · g` for every variable `v_i` of `LM(g)`: those are the
//! S-polynomials of `g` against the field equations `v_i² + v_i`, and
//! multiplying by a variable already in the leading monomial is not
//! order-compatible there — `v_0·(v_0v_1 + v_0 + v_1) = v_0`, a smaller
//! leading monomial.  An engine that forms only the S-polynomials of its
//! own basis can stop early: on `{v_0v_1 + v_0 + v_1}` it returns the
//! generator itself, three standard monomials for an ideal with one
//! solution.  This engine queues a **field pair** `(g, v_i)` for every
//! basis element and every variable of its leading monomial, at sugar
//! degree `deg LM(g) + 1`, and reduces it in the same matrices as the
//! critical pairs.
//!
//! A field product is a single row, not half of a pair, so Faugère's
//! "the pivot is not a leading monomial of an input row" test for new
//! elements does not apply to it.  The test used here is the one that
//! holds for both: a reduced row is **new** iff its pivot is divisible by
//! no active basis leading monomial.  Symbolic preprocessing makes that
//! exact, because every such divisible monomial other than an S-pair's
//! common leading monomial receives a reducer row led by it.
//!
//! ## Pair criteria
//!
//! Critical pairs go through the Gebauer–Möller installation (Becker and
//! Weispfenning's `UPDATE`): the chain criterion on the new pairs and on
//! the queued ones, and Buchberger's product criterion.  These hold for
//! the ideal `I + ⟨v_i² + v_i⟩` of the polynomial ring, whose basis
//! elements other than the field equations are exactly the boolean
//! polynomials here, with square-free leading monomials; the field pairs
//! are never pruned and never used to prune.
//!
//! ## What is counted
//!
//! `word_xors`: logical 64-bit XORs in the selected elimination — the
//! forward pass of every step and of the initial echelon, and the backward
//! pass of the final inter-reduction.  On the full-width path it counts the
//! words between a pivot's lead and its last non-zero word.  The FL path
//! counts its compact `B'` and free-column XORs.  A large full-width step can
//! reduce by lookup tables instead (below), which performs fewer physical
//! XORs; `word_xors_performed` reports those actual table-path XORs,
//! construction included.  The
//! construction of the matrices (the products, the column map, the
//! packing) is **not** in that count; its wall time is reported beside
//! it as `build_ns`, so a reader can see what share of the run the unit
//! covers.  Solution extraction counts its monomial tests separately.
//!
//! ## Faugère–Lachartre reducer/free-column split
//!
//! Symbolic preprocessing puts reducers `[A | B]` first, with one distinct
//! leading column per row, followed by the critical-pair rows `[C | D]`.
//! On sufficiently large matrices [`echelon_fl`] reduces the triangular
//! reducer block once to `[I | B']`, forms `D + C B'`, and sends only that
//! free-column block to the packed echelon kernel.  Thus pair rows do not
//! carry reducer columns through the dense elimination and do not replay the
//! reducer cascade.  `F4_F2_FL=0` retains the full-width path as a control;
//! small shapes fall back automatically.  Both forms span the same row space
//! and return the same new pivots in the original column space.
//!
//! ## The leading block by tables
//!
//! Most of a symbolic-preprocessing matrix's elimination is reduction by
//! its leading block of reducers, whose leads are distinct and whose rows
//! never change.  [`BlockTables`] tabulates it four pivot columns at a
//! time, the method of four Russians (Arlazarov, Dinic, Kronrod and
//! Faradzev; Albrecht, Bard and Hart's M4RI): for each pattern a row can
//! carry on four consecutive pivot columns, the sum of those columns'
//! pivots the row-by-row loop would add for it.  A row whose lead falls
//! on those columns then takes one table row instead of up to four
//! pivots.  The loop's decisions there depend only on the row's bits on
//! those columns, all of which have pivots, so each row ends exactly as
//! it would have, pivot for pivot, and its count and end are the loop's.
//!
//! A pivot never changes once made, so the same holds for the pivots the
//! elimination makes itself: after each batch of rows, the groups of
//! columns its new pivots land in are tabled afresh, and later batches
//! reduce through them too.  The tables hold at most the matrix's words
//! for the leading block and twice them in all.
//!
//! ## Solutions, without enumeration
//!
//! The ideal of a boolean system is radical and zero-dimensional, so its
//! reduced basis has exactly as many standard monomials as the system
//! has solutions.  A variable is either the leading monomial of a linear
//! basis element or itself standard — *free*.  Every linear element
//! reads `v_p + (free variables) + c`, so an assignment of the free
//! variables determines the point, and there are at most
//! `#solutions − 1` of them.  [`solutions_from_reduced_basis`]
//! enumerates those assignments and keeps the points every basis element
//! vanishes at; the cost is `2^{#free}`, not `2^n`.
//!
//! ## References
//!
//! - J.-C. Faugère, *A new efficient algorithm for computing Gröbner
//!   bases (F4)*, J. Pure Appl. Algebra 139 (1999).
//! - T. Becker, V. Weispfenning, *Gröbner Bases*, Springer 1993 —
//!   `UPDATE`, p. 230.
//! - V. Arlazarov, E. Dinic, M. Kronrod, I. Faradzev, *On economical
//!   construction of the transitive closure of a directed graph*, Soviet
//!   Math. Dokl. 11 (1970).
//! - M. Albrecht, G. Bard, W. Hart, *Algorithm 898: Efficient
//!   multiplication of dense matrices over GF(2)*, ACM TOMS 37 (2010).
//! - J.-C. Faugère, S. Lachartre, *Parallel Gaussian Elimination for
//!   Gröbner Bases Computations in Finite Fields*, PASCO 2010.
//! - M. Brickenstein, *Boolean Gröbner bases*, PhD thesis, TU
//!   Kaiserslautern 2010 — the boolean ring and its field pairs.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::cryptanalysis::bitop_tally as tally;
use crate::cryptanalysis::fx_hash::{FxMap, FxSet};
use crate::cryptanalysis::pq_groebner_f2::{
    bit, cmp_mono, mono_key, sort_masks_descending, F2BoolMono, F2BoolPoly, Mask, MASK_BITS,
};

/// What one F4 run cost, and how far it got.
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct F4Stats {
    /// F4 steps: one matrix each, after the initial echelon.
    pub steps: u64,
    /// Critical pairs reduced, and field pairs reduced.
    pub pairs_reduced: u64,
    pub field_pairs_reduced: u64,
    /// Pairs Buchberger's product criterion dropped.
    pub pairs_product_skipped: u64,
    /// Pairs the Gebauer–Möller chain criterion dropped.
    pub pairs_chain_skipped: u64,
    /// New-pair selector use and exact dense-domain work.
    pub pair_dense_select_calls: u64,
    pub pair_quadratic_select_calls: u64,
    pub pair_candidate_visits: u64,
    pub pair_lcm_groups: u64,
    pub pair_cover_lookups: u64,
    pub pair_dense_scratch_bytes_max: u64,
    /// One when this F4 call serializes only product, symbolic and packing
    /// work while retaining the selected parallel elimination path.
    pub inner_build_parallel_disabled_calls: u64,
    /// Optional Stage 196 matrix-shape profile. Bins are `<256`, `256..512`,
    /// `512..1024`, `1024..2048`, `2048..4096`, and `>=4096` rows.
    pub matrix_profile_counts: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_rows_sum: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_cols_sum: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_eliminate_ns: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_logical_xors: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_performed_xors: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_full_m4ri_counts: [u64; MATRIX_PROFILE_BINS],
    pub matrix_profile_unbinned_eliminate_ns: u64,
    pub matrix_profile_unbinned_logical_xors: u64,
    pub matrix_profile_unbinned_performed_xors: u64,
    /// Rows symbolic preprocessing added as reducers.
    pub reducer_rows: u64,
    /// Largest matrix built, and the sum of rows over every matrix.
    pub matrix_rows_max: u64,
    pub matrix_cols_max: u64,
    pub matrix_rows_sum: u64,
    /// The unit: logical 64-bit XORs in the selected elimination schedule.
    pub word_xors: u64,
    /// 64-bit XORs actually performed, reducing by the leading block
    /// through tables where a step is large enough (their construction
    /// included); equal to `word_xors` where no table was built.
    pub word_xors_performed: u64,
    /// Divisibility tests made by symbolic preprocessing, one word
    /// operation each; reported, not in `word_xors`.
    pub divisor_tests: u64,
    /// Elements added to the basis after the initial echelon.
    pub new_elements: u64,
    /// Highest step degree processed.
    pub degree_reached: u32,
    /// Highest step degree at which the basis gained an element: the
    /// solving degree, comparable to `GbStats::solving_degree`.
    pub solving_degree: u32,
    /// Highest degree of any row in any matrix.
    pub max_poly_degree: u32,
    /// Elements of the reduced basis returned.
    pub basis_len: u64,
    /// Largest matrix held, in bytes of packed words.
    pub peak_matrix_bytes: u64,
    /// Largest live lookup-table allocation, in bytes of packed words.
    pub peak_table_bytes: u64,
    /// Matrices and pivot blocks routed through the opt-in full-matrix M4RI
    /// schedule, plus its actual table-preparation XORs.
    pub full_m4ri_matrices: u64,
    pub full_m4ri_blocks: u64,
    pub full_m4ri_table_word_xors: u64,
    /// Matrices reduced in Faugère–Lachartre form, with their reducer,
    /// pair-row and free-column totals.
    pub fl_matrices: u64,
    pub fl_reducer_rows: u64,
    pub fl_pair_rows: u64,
    pub fl_free_cols: u64,
    /// Wall time spent building matrices (products, columns, packing)
    /// and eliminating them.
    pub build_ns: u64,
    pub eliminate_ns: u64,
    pub wall_ns: u64,
    /// The budget ran out; the returned set is not a Gröbner basis.
    pub timed_out: bool,
    /// A matrix would have exceeded [`MAX_MATRIX_WORDS`].
    pub oversize: bool,
    pub pairs_left: u64,
    /// [`groebner_basis_f4_until`] stopped once the basis's degree-1
    /// elements left at most its `max_free` variables unpinned; the
    /// returned set is then not a Gröbner basis.
    pub early_stopped: bool,
    /// Variables no degree-1 element pinned when the run ended.
    pub free_at_stop: u64,
    /// [`F4Skeleton`] counters (all zero without a skeleton).  `1` when
    /// this call found the skeleton built for another shape and cleared it.
    pub skeleton_shape_rebuilds: u64,
    /// Steps whose key (degree and active leading monomials) was new to the
    /// skeleton and was stored.
    pub skeleton_steps_built: u64,
    /// Steps whose key the skeleton already held (the stored divisor
    /// candidates and columns were consulted).
    pub skeleton_steps_replayed: u64,
    /// Monomials whose reducer came from a stored candidate list.
    pub skeleton_candidate_hits: u64,
    /// Monomials looked up in a stored step and not found (scanned in full,
    /// and stored).
    pub skeleton_candidate_misses: u64,
    /// Divisor tests made on stored candidate lists (part of
    /// `divisor_tests`).
    pub skeleton_candidate_tests: u64,
    /// Steps whose stored column map matched the step's monomial set and
    /// was used instead of a new sort and index.
    pub skeleton_columns_reused: u64,
    /// Steps of a stored key whose column map did not match (rebuilt).
    pub skeleton_columns_rebuilt: u64,
    /// Steps not stored because the skeleton was at its byte cap.
    pub skeleton_store_refused: u64,
    /// Bytes the skeleton holds after this call (charged as memory).
    pub skeleton_bytes: u64,
}

/// **Symbolic preprocessing shared across the systems of one run**
/// (`docs/ic/GROEBNER_PERF_LEVERS_20261111.md`, lever 2a).
///
/// The symbolic phase of an F4 step reads the basis only through two
/// things: which active element's leading monomial divides each monomial
/// met (the *divisor candidates*; the reducer is the shortest of them),
/// and the sorted set of monomials the step's rows touch (the *columns*).
/// The first is a function of the active leading monomials alone, so the
/// systems of one cell, which mostly walk the same leading-monomial
/// staircase with different tails, look the same monomials up against the
/// same divisors again and again.  The skeleton stores, per step key
/// `(degree, active leading monomials in basis order)`:
///
/// - for each monomial looked up, the positions of its divisors among the
///   active elements, so a later step of the same key tests only those
///   (the shortest-first, first-on-tie choice is remade on the current
///   term counts, so the reducer is exactly the one a full scan picks);
/// - the step's sorted column list and its index, reused only when the
///   new step's monomial set equals it (verified: equal size and every
///   stored monomial present), otherwise rebuilt and replaced.
///
/// Results are therefore identical with or without the skeleton.  What a
/// replay still pays is charged where it happens: forming and comparing
/// the key, a hash probe per lookup, the candidate tests, the column
/// verification; misses pay the full scan plus the store.  The skeleton
/// lives on one solver instance (one run, one target) and is never
/// written to disk.
#[derive(Default)]
pub struct F4Skeleton {
    shape: Option<(usize, u32)>,
    steps: FxMap<(u32, Box<[Mask]>), SkeletonStep>,
    bytes: u64,
    cap_bytes: u64,
}

#[derive(Default)]
struct SkeletonStep {
    candidates: FxMap<Mask, Box<[u32]>>,
    columns: Option<Columns>,
}

/// Default byte cap of an [`F4Skeleton`].
pub const SKELETON_DEFAULT_CAP_BYTES: u64 = 256 << 20;

impl F4Skeleton {
    /// An empty skeleton that stops storing new steps past `cap_bytes`.
    pub fn with_cap(cap_bytes: u64) -> Self {
        Self {
            cap_bytes,
            ..Self::default()
        }
    }

    /// Bytes held.
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Steps held.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    fn columns_bytes(c: &Columns) -> u64 {
        (c.monos.capacity() * std::mem::size_of::<Mask>()) as u64
            + match &c.index {
                ColumnIndex::Dense(at) => (at.capacity() * 4) as u64,
                ColumnIndex::Hashed(map) => (map.capacity() * 24) as u64,
            }
    }
}

/// The largest matrix a step may build, in 64-bit words (1 GiB), unless
/// `F4_F2_MAX_MATRIX_WORDS` raises or lowers it for one run.
pub const MAX_MATRIX_WORDS: u64 = 1 << 27;

fn max_matrix_words() -> u64 {
    static CAP: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *CAP.get_or_init(|| {
        std::env::var("F4_F2_MAX_MATRIX_WORDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(MAX_MATRIX_WORDS)
    })
}

const NONE: u32 = u32::MAX;
pub const MATRIX_PROFILE_BINS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PairKind {
    /// The S-polynomial of basis elements `i` and `j`.
    Critical(usize, usize),
    /// `v · g_i` for a variable `v` of `LM(g_i)`: the S-polynomial of
    /// `g_i` against the field equation `v² + v`.
    Field(usize, u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pair {
    kind: PairKind,
    /// The lcm of the two leading monomials (critical pairs only).
    lcm: Mask,
    /// Selection degree: `deg lcm`, or `deg LM(g) + 1` for a field pair.
    deg: u32,
}

/// A bit-packed row.  Words outside `[start, end)` are zero.
#[derive(Clone, Default)]
struct Row {
    bits: Vec<u64>,
    start: usize,
    end: usize,
}

impl Row {
    fn lead(&mut self) -> Option<usize> {
        while self.start < self.end {
            let w = self.bits[self.start];
            if w != 0 {
                return Some(self.start * 64 + w.trailing_zeros() as usize);
            }
            self.start += 1;
        }
        None
    }

    /// [`Row::lead`], adding to `ops` the words it tests: one, plus one
    /// for every zero word it skips (`bitop/v1` word operations; each word
    /// is skipped at most once over the row's life).
    #[inline]
    fn lead_counted(&mut self, ops: &mut u64) -> Option<usize> {
        let from = self.start;
        let lead = self.lead();
        *ops += 1 + (self.start - from) as u64;
        lead
    }
}

/// The columns of one matrix: monomials in descending order, so the
/// first set bit of a row is its leading monomial.
struct Columns {
    monos: Vec<Mask>,
    index: ColumnIndex,
}

/// A monomial's column: an array indexed by the mask itself while the
/// masks span at most [`DENSE_INDEX_VARS`] variables, a hash map beyond.
enum ColumnIndex {
    Dense(Vec<u32>),
    Hashed(FxMap<Mask, usize>),
}

/// Variables up to which [`Columns`] indexes by array: `4 · 2^22` bytes at
/// most, against a hash lookup per term packed.
const DENSE_INDEX_VARS: u32 = 22;

/// Set membership during symbolic preprocessing.  For up to 22 variables
/// one bit per possible mask is smaller than the column index and avoids
/// hashing every repeated term.  Values are retained in insertion order
/// only to enumerate the final column set; `Columns` sorts them itself.
enum MonomialSeen {
    Bitmap {
        bits: Vec<u64>,
        limit: Mask,
        overflow: FxSet<Mask>,
        values: Vec<Mask>,
    },
    Hashed(FxSet<Mask>),
}

impl MonomialSeen {
    fn new(n_vars: usize) -> Self {
        static BITMAP: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let enabled =
            *BITMAP.get_or_init(|| std::env::var("F4_F2_BITMAP_SEEN").as_deref() != Ok("0"));
        Self::with_bitmap(n_vars, enabled)
    }

    fn with_bitmap(n_vars: usize, enabled: bool) -> Self {
        if enabled && n_vars <= DENSE_INDEX_VARS as usize {
            let limit: Mask = 1 << n_vars;
            // clearing the bitmap
            tally::word_ops(limit.div_ceil(64) as u64);
            Self::Bitmap {
                bits: vec![0; (limit as usize).div_ceil(64)],
                limit,
                overflow: FxSet::default(),
                values: Vec::new(),
            }
        } else {
            Self::Hashed(FxSet::default())
        }
    }

    #[inline]
    fn insert(&mut self, mask: Mask) -> bool {
        match self {
            Self::Bitmap {
                bits,
                limit,
                overflow,
                values,
            } => {
                let fresh = if mask < *limit {
                    let slot = &mut bits[mask as usize / 64];
                    let bit = 1u64 << (mask % 64);
                    let fresh = *slot & bit == 0;
                    *slot |= bit;
                    fresh
                } else {
                    overflow.insert(mask)
                };
                if fresh {
                    values.push(mask);
                }
                fresh
            }
            Self::Hashed(set) => set.insert(mask),
        }
    }

    #[inline]
    fn contains(&self, mask: &Mask) -> bool {
        match self {
            Self::Bitmap {
                bits,
                limit,
                overflow,
                ..
            } => {
                if *mask < *limit {
                    bits[*mask as usize / 64] & (1u64 << (*mask % 64)) != 0
                } else {
                    overflow.contains(mask)
                }
            }
            Self::Hashed(set) => set.contains(mask),
        }
    }

    fn extend(&mut self, values: impl IntoIterator<Item = Mask>) {
        let mut n = 0u64;
        for mask in values {
            self.insert(mask);
            n += 1;
        }
        self.charge(n);
    }

    /// Charge `n` insertions or membership tests: a word operation each
    /// on the bitmap, a hash probe each on the hashed set.
    fn charge(&self, n: u64) {
        match self {
            Self::Bitmap { .. } => tally::word_ops(n),
            Self::Hashed(_) => tally::hash_probes(n),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Bitmap { values, .. } => values.len(),
            Self::Hashed(set) => set.len(),
        }
    }

    fn into_values(self) -> Vec<Mask> {
        match self {
            Self::Bitmap { values, .. } => values,
            Self::Hashed(set) => set.into_iter().collect(),
        }
    }
}

impl Columns {
    fn from_monomials(set: impl IntoIterator<Item = Mask>) -> Self {
        let mut monos: Vec<Mask> = set.into_iter().collect();
        // Charged: the sort, the dedup scan and the span fold, one word
        // operation a key each, then the index (below).
        tally::word_ops(tally::sort_cost(monos.len()) + monos.len() as u64);
        sort_masks_descending(&mut monos);
        monos.dedup();
        tally::word_ops(monos.len() as u64);
        let span = monos.iter().fold(0 as Mask, |acc, &m| acc | m);
        let bits = Mask::BITS - span.leading_zeros();
        let index = if bits <= DENSE_INDEX_VARS {
            // clearing the array (two `u32` slots a word), one write a column
            tally::word_ops((1u64 << bits).div_ceil(2) + monos.len() as u64);
            let mut at = vec![NONE; 1usize << bits];
            for (i, &m) in monos.iter().enumerate() {
                at[m as usize] = i as u32;
            }
            ColumnIndex::Dense(at)
        } else {
            tally::hash_probes(monos.len() as u64);
            ColumnIndex::Hashed(monos.iter().enumerate().map(|(i, &m)| (m, i)).collect())
        };
        Self { monos, index }
    }

    #[inline]
    fn column(&self, m: Mask) -> usize {
        match &self.index {
            ColumnIndex::Dense(at) => {
                let c = at[m as usize];
                debug_assert_ne!(c, NONE, "a monomial outside the columns");
                c as usize
            }
            ColumnIndex::Hashed(index) => index[&m],
        }
    }

    fn words(&self) -> usize {
        self.monos.len().div_ceil(64).max(1)
    }

    /// Charge packing `polys`: a zeroed row of [`Columns::words`] words,
    /// then per term one column lookup (a word operation on the dense
    /// index, a hash probe on the hashed one) and one XOR into the row.
    /// Charged by the caller's thread, whichever thread packs.
    fn charge_pack<'p>(&self, polys: impl IntoIterator<Item = &'p F2BoolPoly>) {
        let (mut rows, mut terms) = (0u64, 0u64);
        for p in polys {
            rows += 1;
            terms += p.terms.len() as u64;
        }
        let hashed = matches!(self.index, ColumnIndex::Hashed(_));
        tally::word_ops(rows * self.words() as u64 + terms * (2 - u64::from(hashed)));
        if hashed {
            tally::hash_probes(terms);
        }
    }

    fn pack(&self, p: &F2BoolPoly) -> Row {
        let mut bits = vec![0u64; self.words()];
        let (mut start, mut end) = (usize::MAX, 0usize);
        for t in &p.terms {
            let c = self.column(t.mask);
            bits[c / 64] ^= 1u64 << (c % 64);
            start = start.min(c / 64);
            end = end.max(c / 64 + 1);
        }
        if start == usize::MAX {
            start = 0;
        }
        Row { bits, start, end }
    }

    fn unpack(&self, row: &Row, n_vars: usize) -> F2BoolPoly {
        // one test a word, one extraction a set bit, then the canonical
        // rebuild of the term list
        let set: u32 = row.bits[row.start..row.end]
            .iter()
            .map(|w| w.count_ones())
            .sum();
        tally::word_ops(
            (row.end - row.start) as u64 + u64::from(set) + tally::poly::from_monos(set as usize),
        );
        // exactly `set` terms: one allocation, no regrowth
        let mut monos = Vec::with_capacity(set as usize);
        for w in row.start..row.end {
            let mut word = row.bits[w];
            while word != 0 {
                let c = w * 64 + word.trailing_zeros() as usize;
                word &= word - 1;
                monos.push(F2BoolMono::from_mask(self.monos[c]));
            }
        }
        F2BoolPoly::from_monos(monos, n_vars)
    }
}

/// Rows × words of a matrix's non-leading rows below which the reductions
/// by the leading block run on one thread.
const PAR_WORDS: usize = 1 << 16;

const FULL_M4RI_BLOCK: usize = 8;

#[derive(Default)]
struct FullM4riScratch {
    pivot_columns: Vec<usize>,
    pivot_ends: Vec<usize>,
    block_pivots: Vec<Vec<u64>>,
    table: Vec<u64>,
    table_counts: Vec<u64>,
}

thread_local! {
    static FULL_M4RI_SCRATCH: std::cell::RefCell<FullM4riScratch> =
        std::cell::RefCell::new(FullM4riScratch::default());
}

fn full_m4ri_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        let mode = std::env::var("F4_F2_FULL_M4RI").ok();
        full_m4ri_enabled_for(mode.as_deref())
    })
}

fn full_m4ri_shape(n_rows: usize, n_cols: usize) -> bool {
    full_m4ri_shape_for(n_rows, n_cols, full_m4ri_min_rows())
}

fn full_m4ri_min_rows() -> usize {
    static MIN_ROWS: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *MIN_ROWS.get_or_init(|| {
        let mode = std::env::var("F4_F2_FULL_M4RI").ok();
        let explicit = std::env::var("F4_F2_FULL_M4RI_MIN_ROWS").ok();
        full_m4ri_min_rows_for(mode.as_deref(), explicit.as_deref())
    })
}

fn full_m4ri_enabled_for(mode: Option<&str>) -> bool {
    mode != Some("0")
}

fn full_m4ri_min_rows_for(mode: Option<&str>, explicit: Option<&str>) -> usize {
    explicit
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| if mode == Some("1") { 128 } else { 4096 })
        .max(128)
}

fn full_m4ri_shape_for(n_rows: usize, n_cols: usize, min_rows: usize) -> bool {
    n_rows >= min_rows && n_cols >= 256 && n_cols <= n_rows.saturating_mul(4)
}

/// Full-matrix Method of Four Russians elimination.
///
/// Each block first finds up to eight pivots and triangularises those pivot
/// rows. It then materialises every block combination once and clears the
/// block from each remaining row with one suffix XOR. `word_xors` charges the
/// individual pivot reductions represented by a combination; the actual row
/// and table-preparation XORs go to `word_xors_performed`.
fn echelon_full_m4ri(
    rows: Vec<Row>,
    n_cols: usize,
    st: &mut F4Stats,
    deadline: Option<Instant>,
) -> Option<Vec<(usize, Row)>> {
    echelon_full_m4ri_with_expiry(rows, n_cols, st, || {
        deadline.is_some_and(|limit| Instant::now() >= limit)
    })
}

fn echelon_full_m4ri_with_expiry(
    rows: Vec<Row>,
    n_cols: usize,
    st: &mut F4Stats,
    mut deadline_expired: impl FnMut() -> bool,
) -> Option<Vec<(usize, Row)>> {
    let n_rows = rows.len();
    let words = n_cols.div_ceil(64).max(1);
    let mut row_ends: Vec<usize> = rows.iter().map(|row| row.end).collect();
    let mut matrix: Vec<Vec<u64>> = rows.into_iter().map(|row| row.bits).collect();
    let mut logical_xors = 0u64;
    let mut performed_xors = 0u64;
    let mut preparation_xors = 0u64;
    let mut blocks = 0u64;
    let mut aux = 0u64;
    let result = FULL_M4RI_SCRATCH.with(|scratch| {
        let mut scratch = scratch.borrow_mut();
        scratch.pivot_columns.resize(FULL_M4RI_BLOCK, 0);
        scratch.pivot_ends.resize(FULL_M4RI_BLOCK, 0);
        scratch.block_pivots.resize_with(FULL_M4RI_BLOCK, Vec::new);
        for pivot in &mut scratch.block_pivots {
            pivot.resize(words, 0);
        }
        scratch.table.resize((1usize << FULL_M4RI_BLOCK) * words, 0);
        scratch.table_counts.resize(1usize << FULL_M4RI_BLOCK, 0);
        let scratch_bytes = scratch.pivot_columns.capacity() * std::mem::size_of::<usize>()
            + scratch.pivot_ends.capacity() * std::mem::size_of::<usize>()
            + scratch
                .block_pivots
                .iter()
                .map(|row| row.capacity() * std::mem::size_of::<u64>())
                .sum::<usize>()
            + scratch.table.capacity() * std::mem::size_of::<u64>()
            + scratch.table_counts.capacity() * std::mem::size_of::<u64>();
        st.peak_table_bytes = st.peak_table_bytes.max(scratch_bytes as u64);
        let FullM4riScratch {
            pivot_columns,
            pivot_ends,
            block_pivots,
            table,
            table_counts,
        } = &mut *scratch;

        let mut all_pivots = Vec::with_capacity(n_rows.min(n_cols));
        let mut pivot_row = 0usize;
        let mut column = 0usize;
        // Word operations besides the XORs: every bit test of the pivot
        // search and of the pattern a row carries on a block, the copy of
        // each pivot into the block, one a pivot row read back.

        while pivot_row < n_rows && column < n_cols {
            if deadline_expired() {
                return None;
            }
            let block_start = pivot_row;
            let mut block_rows = 0usize;
            while block_rows < FULL_M4RI_BLOCK && pivot_row < n_rows && column < n_cols {
                let next_pivot = block_start + block_rows;
                let (word, bit) = (column / 64, 1u64 << (column % 64));
                let mut found = None;
                for row in next_pivot..n_rows {
                    if row > next_pivot
                        && (row - next_pivot).is_multiple_of(128)
                        && deadline_expired()
                    {
                        return None;
                    }
                    aux += block_rows as u64 + 1;
                    for index in 0..block_rows {
                        let pivot_column = pivot_columns[index];
                        let (from, pivot_bit) = (pivot_column / 64, 1u64 << (pivot_column % 64));
                        if matrix[row][from] & pivot_bit != 0 {
                            let to = pivot_ends[index];
                            for (target, &source) in matrix[row][from..to]
                                .iter_mut()
                                .zip(&block_pivots[index][from..to])
                            {
                                *target ^= source;
                            }
                            let cost = (to - from) as u64;
                            logical_xors += cost;
                            performed_xors += cost;
                            row_ends[row] = row_ends[row].max(to);
                        }
                    }
                    if matrix[row][word] & bit != 0 {
                        found = Some(row);
                        break;
                    }
                }
                if let Some(found) = found {
                    matrix.swap(next_pivot, found);
                    row_ends.swap(next_pivot, found);
                    block_pivots[block_rows].copy_from_slice(&matrix[next_pivot]);
                    aux += (words + next_pivot - block_start) as u64;
                    pivot_ends[block_rows] = row_ends[next_pivot];
                    let (previous_pivots, current_pivots) = block_pivots.split_at_mut(block_rows);
                    let pivot = &current_pivots[0];
                    let pivot_end = pivot_ends[block_rows];
                    for previous in block_start..next_pivot {
                        if matrix[previous][word] & bit != 0 {
                            let block_index = previous - block_start;
                            for (target, &source) in matrix[previous][word..pivot_end]
                                .iter_mut()
                                .zip(&pivot[word..pivot_end])
                            {
                                *target ^= source;
                            }
                            for (target, &source) in previous_pivots[block_index][word..pivot_end]
                                .iter_mut()
                                .zip(&pivot[word..pivot_end])
                            {
                                *target ^= source;
                            }
                            let cost = (2 * (pivot_end - word)) as u64;
                            performed_xors += cost;
                            preparation_xors += cost;
                            row_ends[previous] = row_ends[previous].max(pivot_end);
                            pivot_ends[block_index] = pivot_ends[block_index].max(pivot_end);
                        }
                    }
                    pivot_columns[block_rows] = column;
                    block_rows += 1;
                }
                column += 1;
            }
            if block_rows == 0 {
                break;
            }
            blocks += 1;

            let first_word = pivot_columns[0] / 64;
            let block_end = pivot_ends[..block_rows]
                .iter()
                .copied()
                .max()
                .unwrap_or(first_word + 1);
            let suffix_words = block_end - first_word;
            let combinations = 1usize << block_rows;
            table[..suffix_words].fill(0);
            table_counts[0] = 0;
            let mut filled = 1usize;
            for index in 0..block_rows {
                let pivot = &block_pivots[index][first_word..block_end];
                let split = filled * suffix_words;
                let (source_tables, target_tables) =
                    table[..combinations * suffix_words].split_at_mut(split);
                let pivot_cost = (pivot_ends[index] - pivot_columns[index] / 64) as u64;
                for mask in 0..filled {
                    let offset = mask * suffix_words;
                    for word_index in 0..suffix_words {
                        target_tables[offset + word_index] =
                            source_tables[offset + word_index] ^ pivot[word_index];
                    }
                    table_counts[filled + mask] = table_counts[mask] + pivot_cost;
                }
                let cost = (filled * suffix_words) as u64;
                performed_xors += cost;
                preparation_xors += cost;
                filled *= 2;
            }

            let consecutive = pivot_columns[..block_rows]
                .windows(2)
                .all(|pair| pair[1] == pair[0] + 1);
            for row in block_start + block_rows..n_rows {
                if row > block_start + block_rows
                    && (row - block_start - block_rows).is_multiple_of(128)
                    && deadline_expired()
                {
                    return None;
                }
                aux += if consecutive {
                    3
                } else {
                    block_rows as u64 + 1
                };
                let pattern = if consecutive {
                    let first_column = pivot_columns[0];
                    let (word, offset) = (first_column / 64, first_column % 64);
                    let mut packed = matrix[row][word] >> offset;
                    if offset + block_rows > 64 {
                        packed |= matrix[row][word + 1] << (64 - offset);
                    }
                    packed as usize & ((1usize << block_rows) - 1)
                } else {
                    pivot_columns[..block_rows].iter().enumerate().fold(
                        0usize,
                        |pattern, (index, &pivot_column)| {
                            if matrix[row][pivot_column / 64] & (1u64 << (pivot_column % 64)) != 0 {
                                pattern | 1usize << index
                            } else {
                                pattern
                            }
                        },
                    )
                };
                if pattern != 0 {
                    let offset = pattern * suffix_words;
                    for (target, &source) in matrix[row][first_word..block_end]
                        .iter_mut()
                        .zip(&table[offset..offset + suffix_words])
                    {
                        *target ^= source;
                    }
                    logical_xors += table_counts[pattern];
                    performed_xors += suffix_words as u64;
                    row_ends[row] = row_ends[row].max(block_end);
                }
            }
            all_pivots.extend_from_slice(&pivot_columns[..block_rows]);
            pivot_row += block_rows;
        }

        let mut pivots = Vec::with_capacity(all_pivots.len());
        for ((bits, end), lead) in matrix.into_iter().zip(row_ends).zip(all_pivots) {
            pivots.push((
                lead,
                Row {
                    bits,
                    start: lead / 64,
                    end,
                },
            ));
        }
        aux += pivots.len() as u64;
        Some(pivots)
    });
    st.word_xors += logical_xors;
    st.word_xors_performed += performed_xors;
    st.full_m4ri_matrices += u64::from(result.is_some());
    st.full_m4ri_blocks += blocks;
    st.full_m4ri_table_word_xors += preparation_xors;
    tally::word_ops(aux);
    result
}

/// Forward elimination: each row in turn is reduced by the pivots found
/// so far and becomes a pivot if anything survives.  Returns the pivot
/// rows with their lead columns, or `None` if the deadline passed.
///
/// The leading block of rows with pairwise distinct leads — the reducers
/// of symbolic preprocessing — become pivots untouched, and every later
/// row is first reduced by them alone for as long as its lead is one of
/// theirs.  Those reductions are independent across rows and run in
/// parallel; the serial pass then resumes each row exactly where the
/// one-row-at-a-time loop would stand at that point, so the pivots and
/// the XOR count are identical to it.
fn echelon_block_tables(
    mut rows: Vec<Row>,
    n_cols: usize,
    st: &mut F4Stats,
    deadline: Option<Instant>,
) -> Option<Vec<(usize, Row)>> {
    let mut aux = (n_cols as u64).div_ceil(2);
    let mut xors = 0u64;
    let mut performed = 0u64;
    let run = || {
        let mut pivot_of = vec![NONE; n_cols];
        let mut pivots: Vec<(usize, Row)> = Vec::with_capacity(rows.len());
        // Word operations besides the XORs: clearing the pivot index (two
        // `u32` slots a word), every lead scan (`Row::lead_counted`), one
        // pivot-index lookup a reduction step and one pattern extraction a
        // table reduction.  Charged to this thread at the end.
        // the leading block: rows until the first repeated lead (zero rows
        // are skipped by the serial loop too)
        let mut fixed = 0usize;
        while fixed < rows.len() {
            match rows[fixed].lead_counted(&mut aux) {
                Some(lead) if pivot_of[lead] != NONE => break,
                Some(lead) => pivot_of[lead] = 0,
                None => {}
            }
            fixed += 1;
        }
        let rest = rows.split_off(fixed);
        for mut row in rows {
            if let Some(lead) = row.lead_counted(&mut aux) {
                pivot_of[lead] = pivots.len() as u32;
                pivots.push((lead, row));
            }
        }
        let words = n_cols.div_ceil(64).max(1);
        let expired = std::sync::atomic::AtomicBool::new(false);
        // The leading block's tables, where enough rows follow it to pay and
        // they take no more words than the matrix; later pivots join them
        // within twice that.
        let matrix_words = (pivots.len() + rest.len()) * words;
        let budget = matrix_words * TABLE_INITIAL_BUDGET_MULT;
        let mut tables = (rest.len() >= TABLE_ROWS && pivots.len() >= TABLE_ROWS)
            .then(|| BlockTables::build(&pivot_of, &pivots, budget))
            .flatten();
        if let Some(t) = &tables {
            st.peak_table_bytes = st.peak_table_bytes.max((t.held_words * 8) as u64);
        }
        if let Some(t) = &tables {
            performed += t.built_words;
            // the scan of the pivot index for runs
            aux += n_cols as u64;
        }
        // One reduction at `lead`, which has pivot `p`: by the table when the
        // lead is on a tabled block, else by the pivot.  Returns the words the
        // row-by-row loop would XOR and the words XORed.
        let step = |row: &mut Row,
                    lead: usize,
                    p: u32,
                    pivots: &[(usize, Row)],
                    tables: Option<&BlockTables>|
         -> (u64, u64) {
            if let Some((xors, done)) = tables.and_then(|t| t.reduce(row, lead)) {
                return (xors, done);
            }
            let pivot = &pivots[p as usize].1;
            let (from, to) = (lead / 64, pivot.end);
            for (a, b) in row.bits[from..to].iter_mut().zip(&pivot.bits[from..to]) {
                *a ^= *b;
            }
            row.end = row.end.max(to);
            ((to - from) as u64, (to - from) as u64)
        };
        // Reduce each of `rows` by the pivots already in the table for as long
        // as its lead has one; stop at the first lead without.  Pivot rows
        // never change once made, so these are exactly the reductions the
        // one-row-at-a-time loop would apply first, in the same order.  The
        // rows go a range of columns at a time: every row is taken as far as
        // the range allows before any goes further, so the tables and pivots
        // of a range are read from cache by all of them, not fetched again
        // for each.  The ranges are taken in column order and a row stops for
        // good at a lead without a pivot, so each row's reductions and their
        // order are unchanged.
        let by_known = |rows: &mut [Row],
                        pivot_of: &[u32],
                        pivots: &[(usize, Row)],
                        tables: Option<&BlockTables>|
         -> (u64, u64, u64) {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                expired.store(true, std::sync::atomic::Ordering::Relaxed);
                return (0, 0, 0);
            }
            let (mut xors, mut done, mut ops) = (0u64, 0u64, 0u64);
            let mut live: Vec<usize> = (0..rows.len()).collect();
            while !live.is_empty() {
                let from = live
                    .iter()
                    .filter_map(|&i| rows[i].lead_counted(&mut ops))
                    .min()
                    .unwrap_or(usize::MAX);
                let to = from.saturating_add(TILE_COLUMNS);
                live.retain(|&i| {
                    let row = &mut rows[i];
                    loop {
                        match row.lead_counted(&mut ops) {
                            Some(lead) if lead < to => {
                                let p = pivot_of[lead];
                                // the pivot lookup, and the table lookup (with
                                // the pattern it extracts) when tables exist
                                ops += 1 + 2 * u64::from(tables.is_some());
                                if p == NONE {
                                    return false;
                                }
                                let (x, d) = step(row, lead, p, pivots, tables);
                                xors += x;
                                done += d;
                            }
                            Some(_) => return true,
                            None => return false,
                        }
                    }
                });
            }
            (xors, done, ops)
        };
        let mut rest = rest;
        // The rows after the leading block go in batches.  Each batch is first
        // reduced — in parallel, a share of its rows to a thread — by the
        // pivots that exist before it — the leading block's, then every
        // earlier batch's — and then finished in order: a row resumes where it stopped, against the pivots its
        // predecessors in the batch have just made, and becomes a pivot if
        // anything survives.  Every row sees the reductions, the order and
        // the pivot table the serial loop would give it, so the pivots and
        // the XOR count are identical to it; only the first phase is shared
        // out.  The first batch is the whole remainder against the leading
        // block, whose reductions are the bulk of a symbolic-preprocessing
        // matrix, so the whole remainder takes that pass together before the
        // batches start.
        let mut done = 0usize;
        let mut first = true;
        // On one thread, or on a remainder too small to share out, without
        // tables, reading each batch twice only costs: finish it serially
        // instead, as the loop always did.  With tables the batches and their
        // first phase stay, for the cache reuse of `by_known` and so that each
        // batch's pivots can join the tables for the next.
        // (Size first: asking rayon its thread count starts its pool.)
        let batched = rest.len() * words > PAR_WORDS && rayon::current_num_threads() > 1;
        let batch_rows = (ELIMINATION_BATCH_WORDS / words).clamp(32, 1024);
        while done < rest.len() {
            if deadline.is_some_and(|d| Instant::now() >= d) {
                return None;
            }
            let end = if first || !(batched || tables.is_some()) {
                rest.len()
            } else {
                (done + batch_rows).min(rest.len())
            };
            let made = pivots.len();
            let chunk = &mut rest[done..end];
            if first || batched || tables.is_some() {
                let (pivot_of, known, tables) = (&pivot_of[..], &pivots[..], tables.as_ref());
                let threshold = if first { PAR_WORDS } else { PAR_BATCH_WORDS };
                let (x, d, o) = if chunk.len() > 1 && chunk.len() * words > threshold {
                    let share = chunk.len().div_ceil(rayon::current_num_threads()).max(1);
                    chunk
                        .par_chunks_mut(share)
                        .map(|rows| by_known(rows, pivot_of, known, tables))
                        .reduce(|| (0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
                } else {
                    by_known(chunk, pivot_of, known, tables)
                };
                xors += x;
                performed += d;
                aux += o;
            }
            if expired.load(std::sync::atomic::Ordering::Relaxed) {
                return None;
            }
            if first {
                // Only the leading block's reductions: every row has now
                // stopped at a lead no leading row covers.
                first = false;
                continue;
            }
            for (k, row) in chunk.iter_mut().enumerate() {
                if k > 0 && k % 128 == 0 && deadline.is_some_and(|d| Instant::now() >= d) {
                    return None;
                }
                let mut row = std::mem::take(row);
                while let Some(lead) = row.lead_counted(&mut aux) {
                    aux += 1 + 2 * u64::from(tables.is_some());
                    match pivot_of[lead] {
                        NONE => {
                            pivot_of[lead] = pivots.len() as u32;
                            pivots.push((lead, row));
                            break;
                        }
                        p => {
                            let (x, d) = step(&mut row, lead, p, &pivots, tables.as_ref());
                            xors += x;
                            performed += d;
                        }
                    }
                }
            }
            done = end;
            if let Some(t) = &mut tables {
                if rest.len() - done >= TABLE_ROWS && pivots.len() > made {
                    // the new pivots' groups, sorted, and their columns scanned
                    aux += tally::sort_cost(pivots.len() - made)
                        + (TABLE_BLOCK * (pivots.len() - made)) as u64;
                    performed += t.extend(
                        &pivot_of,
                        &pivots,
                        &pivots[made..],
                        matrix_words * TABLE_EXTENDED_BUDGET_MULT,
                    );
                    st.peak_table_bytes = st.peak_table_bytes.max((t.held_words * 8) as u64);
                }
            }
        }
        Some(pivots)
    };
    let result = run();
    st.word_xors += xors;
    st.word_xors_performed += performed;
    tally::word_ops(aux);
    result
}

fn echelon(
    rows: Vec<Row>,
    n_cols: usize,
    st: &mut F4Stats,
    deadline: Option<Instant>,
) -> Option<Vec<(usize, Row)>> {
    let use_full_m4ri = full_m4ri_enabled() && full_m4ri_shape(rows.len(), n_cols);
    if !matrix_profile_enabled() {
        return if use_full_m4ri {
            echelon_full_m4ri(rows, n_cols, st, deadline)
        } else {
            echelon_block_tables(rows, n_cols, st, deadline)
        };
    }
    let n_rows = rows.len();
    let bin = matrix_profile_bin(n_rows);
    let logical_before = st.word_xors;
    let performed_before = st.word_xors_performed;
    let started = Instant::now();
    let result = if use_full_m4ri {
        echelon_full_m4ri(rows, n_cols, st, deadline)
    } else {
        echelon_block_tables(rows, n_cols, st, deadline)
    };
    st.matrix_profile_counts[bin] += 1;
    st.matrix_profile_rows_sum[bin] += n_rows as u64;
    st.matrix_profile_cols_sum[bin] += n_cols as u64;
    st.matrix_profile_eliminate_ns[bin] =
        st.matrix_profile_eliminate_ns[bin].saturating_add(started.elapsed().as_nanos() as u64);
    st.matrix_profile_logical_xors[bin] = st.matrix_profile_logical_xors[bin]
        .saturating_add(st.word_xors.saturating_sub(logical_before));
    st.matrix_profile_performed_xors[bin] = st.matrix_profile_performed_xors[bin]
        .saturating_add(st.word_xors_performed.saturating_sub(performed_before));
    st.matrix_profile_full_m4ri_counts[bin] += u64::from(use_full_m4ri);
    result
}

/// Minimum full-matrix width for the Boolean Faugère–Lachartre split.
const FL_MIN_COLS: usize = 256;
/// The reducer block and pair block must both be large enough to amortise
/// splitting and compressing the rows.
const FL_MIN_ROWS: usize = 32;

/// `F4_F2_FL=0` retains the full-width reducer cascade as a same-binary
/// control.
fn fl_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("F4_F2_FL").as_deref() != Ok("0"))
}

fn fl_shape(reducers: usize, pair_rows: usize, n_cols: usize) -> bool {
    if !fl_enabled()
        || n_cols < FL_MIN_COLS
        || reducers < FL_MIN_ROWS
        || pair_rows < FL_MIN_ROWS
        || reducers >= n_cols
    {
        return false;
    }
    let free_words = (n_cols - reducers).div_ceil(64);
    // Bound the compact reducer and pair blocks by the same allocation cap
    // as a normal F4 matrix.
    (reducers + pair_rows).saturating_mul(free_words) <= max_matrix_words() as usize
}

/// Split one packed row into its reducer-column coefficients and its packed
/// free-column part.  `pivot_of[c] == NONE` marks a free column.
fn split_fl_row(
    row: &Row,
    pivot_of: &[u32],
    free_index: &[u32],
    free_words: usize,
) -> (Row, Vec<usize>, u64) {
    let mut bits = vec![0u64; free_words];
    let mut dependencies = Vec::new();
    let mut inspected = 0u64;
    let mut end = 0usize;
    for (word_index, &packed) in row.bits[..row.end].iter().enumerate() {
        let mut word = packed;
        while word != 0 {
            let column = word_index * 64 + word.trailing_zeros() as usize;
            word &= word - 1;
            inspected += 1;
            let pivot = pivot_of[column];
            if pivot == NONE {
                let free = free_index[column] as usize;
                bits[free / 64] |= 1u64 << (free % 64);
                end = end.max(free / 64 + 1);
            } else {
                dependencies.push(pivot as usize);
            }
        }
    }
    (
        Row {
            bits,
            start: 0,
            end,
        },
        dependencies,
        inspected,
    )
}

fn xor_fl_row(row: &mut Row, reducer: &Row) -> u64 {
    for (target, source) in row.bits[..reducer.end]
        .iter_mut()
        .zip(&reducer.bits[..reducer.end])
    {
        *target ^= *source;
    }
    row.end = row.end.max(reducer.end);
    while row.end > 0 && row.bits[row.end - 1] == 0 {
        row.end -= 1;
    }
    reducer.end as u64
}

/// Faugère–Lachartre elimination over `F_2`.
///
/// Symbolic preprocessing orders the rows as reducers `[A | B]` followed by
/// pair rows `[C | D]`; reducer leading columns are distinct and every tail
/// column is to the right of its lead.  Reduce the triangular reducer block
/// once to `[I | B']`, form the compact Schur complement `D + C B'`, and run
/// the existing packed echelon kernel only on free columns.  Returned rows are
/// expanded with zero reducer columns so the caller's polynomial readback is
/// unchanged.
fn echelon_fl(
    rows: Vec<Row>,
    n_cols: usize,
    reducer_count: usize,
    st: &mut F4Stats,
    deadline: Option<Instant>,
) -> Option<Vec<(usize, Row)>> {
    echelon_fl_with_expiry(rows, n_cols, reducer_count, st, deadline, || {
        deadline.is_some_and(|limit| Instant::now() >= limit)
    })
}

// The production wrapper supplies the original deadline predicate. A private
// stop control permits deterministic interrupted-prefix accounting tests.
fn echelon_fl_with_expiry(
    mut rows: Vec<Row>,
    n_cols: usize,
    reducer_count: usize,
    st: &mut F4Stats,
    deadline: Option<Instant>,
    mut deadline_expired: impl FnMut() -> bool,
) -> Option<Vec<(usize, Row)>> {
    let mut aux = (n_cols as u64).div_ceil(2);
    let mut xors = 0u64;
    let run = || {
        debug_assert!(reducer_count <= rows.len());
        let pair_rows = rows.split_off(reducer_count);
        let reducer_rows = rows;
        let mut pivot_of = vec![NONE; n_cols];
        let mut leads = Vec::with_capacity(reducer_count);
        for (index, mut row) in reducer_rows.iter().cloned().enumerate() {
            let lead = row.lead_counted(&mut aux)?;
            debug_assert_eq!(pivot_of[lead], NONE, "reducer leads are distinct");
            pivot_of[lead] = index as u32;
            leads.push(lead);
        }
        let free_cols: Vec<usize> = (0..n_cols).filter(|&c| pivot_of[c] == NONE).collect();
        let n_free = free_cols.len();
        let free_words = n_free.div_ceil(64);
        // While B' is built, the original packed matrix is still live.  Charge
        // both so the solver's peak-memory report does not hide the FL scratch
        // matrix behind the already-recorded full-width allocation.
        let full_words = n_cols.div_ceil(64) as u64;
        let original_words = (reducer_count + pair_rows.len()) as u64 * full_words;
        let bprime_words = reducer_count as u64 * free_words as u64;
        st.peak_matrix_bytes = st.peak_matrix_bytes.max(
            original_words
                .saturating_add(bprime_words)
                .saturating_mul(8),
        );
        let mut free_index = vec![NONE; n_cols];
        for (index, &column) in free_cols.iter().enumerate() {
            free_index[column] = index as u32;
        }

        let mut order: Vec<usize> = (0..reducer_count).collect();
        order.sort_unstable_by_key(|&index| std::cmp::Reverse(leads[index]));
        let mut bprime: Vec<Row> = (0..reducer_count)
            .map(|_| Row {
                bits: vec![0; free_words],
                start: 0,
                end: 0,
            })
            .collect();
        for (position, &index) in order.iter().enumerate() {
            if position.is_multiple_of(64) && deadline_expired() {
                return None;
            }
            let (mut free, dependencies, inspected) =
                split_fl_row(&reducer_rows[index], &pivot_of, &free_index, free_words);
            aux += inspected;
            for dependency in dependencies {
                if dependency == index {
                    continue;
                }
                debug_assert!(leads[dependency] > leads[index]);
                xors += xor_fl_row(&mut free, &bprime[dependency]);
            }
            bprime[index] = free;
        }
        drop(reducer_rows);

        let reduce_pair = |row: Row| {
            let (mut free, dependencies, inspected) =
                split_fl_row(&row, &pivot_of, &free_index, free_words);
            let mut row_xors = 0u64;
            for dependency in dependencies {
                row_xors += xor_fl_row(&mut free, &bprime[dependency]);
            }
            (free, row_xors, inspected)
        };
        let parallel = pair_rows.len() > 1
            && pair_rows.len().saturating_mul(free_words) > PAR_WORDS
            && inner_build_parallel_enabled()
            && rayon::current_num_threads() > 1;
        let reduced: Vec<(Row, u64, u64)> = if parallel {
            pair_rows.into_par_iter().map(reduce_pair).collect()
        } else {
            pair_rows.into_iter().map(reduce_pair).collect()
        };
        let mut compact = Vec::with_capacity(reduced.len());
        for (row, row_xors, inspected) in reduced {
            compact.push(row);
            xors += row_xors;
            aux += inspected;
        }
        st.fl_matrices += 1;
        st.fl_reducer_rows += reducer_count as u64;
        st.fl_pair_rows += compact.len() as u64;
        st.fl_free_cols += n_free as u64;

        if n_free == 0 {
            return Some(Vec::new());
        }
        let compact_pivots = echelon(compact, n_free, st, deadline)?;
        let full_words = n_cols.div_ceil(64).max(1);
        let mut expanded = Vec::with_capacity(compact_pivots.len());
        for (lead, row) in compact_pivots {
            let mut bits = vec![0u64; full_words];
            let mut end = 0usize;
            for word_index in row.start..row.end {
                let mut word = row.bits[word_index];
                while word != 0 {
                    let free = word_index * 64 + word.trailing_zeros() as usize;
                    word &= word - 1;
                    let column = free_cols[free];
                    bits[column / 64] |= 1u64 << (column % 64);
                    end = end.max(column / 64 + 1);
                }
            }
            let full_lead = free_cols[lead];
            expanded.push((
                full_lead,
                Row {
                    bits,
                    start: full_lead / 64,
                    end,
                },
            ));
        }
        Some(expanded)
    };
    let result = run();
    st.word_xors += xors;
    st.word_xors_performed += xors;
    tally::word_ops(aux);
    result
}

fn matrix_profile_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("F4_F2_MATRIX_PROFILE").as_deref() == Ok("1"))
}

fn matrix_profile_bin(rows: usize) -> usize {
    match rows {
        0..=255 => 0,
        256..=511 => 1,
        512..=1023 => 2,
        1024..=2047 => 3,
        2048..=4095 => 4,
        _ => 5,
    }
}

/// Columns per range in [`echelon`]'s shared-out pass: every row of a
/// share is taken through a range before any goes past it.
const TILE_COLUMNS: usize = 256;

/// Rows after the leading block, and rows in it, from which [`echelon`]
/// reduces by the leading block through [`BlockTables`].
const TABLE_ROWS: usize = 256;

/// Pivot columns per table: an aligned group of them never straddles a
/// word, so a row's pattern on it is one shift and mask.
const TABLE_BLOCK: usize = if cfg!(feature = "f4-wide-tables") {
    6
} else if cfg!(feature = "f4-four-tables") {
    4
} else {
    5
};
const TABLE_INITIAL_BUDGET_MULT: usize = if cfg!(feature = "f4-wide-tables") {
    4
} else if cfg!(feature = "f4-four-tables") {
    1
} else {
    2
};
const TABLE_EXTENDED_BUDGET_MULT: usize = if cfg!(feature = "f4-wide-tables") {
    4
} else if cfg!(feature = "f4-four-tables") {
    2
} else {
    3
};

/// One table: the leading block's pivots on up to [`TABLE_BLOCK`]
/// consecutive columns from `first`, all of which have one.
struct BlockTable {
    first: usize,
    k: usize,
    /// Row `b` (for `1 ≤ b < 2^k`) covers words `[first / 64, ends[b])`
    /// at `b * width`; row 0 is unused.
    width: usize,
    words: Vec<u64>,
    ends: [usize; 1 << TABLE_BLOCK],
    counts: [u64; 1 << TABLE_BLOCK],
}

/// The method of four Russians on an [`echelon`]'s pivots: a
/// [`BlockTable`] for every run of two or more pivot columns within an
/// aligned group of [`TABLE_BLOCK`] — the leading block's first, then each
/// group a later pivot lands in, re-tabled.
///
/// `table[b]`, for the pattern `b` a row carries on the block's columns
/// (bit `j` for column `first + j`), is the sum of the block's pivots the
/// row-by-row loop adds to such a row: by the lowest set bit `j`, pivot
/// `j` plus `table[b ^ pattern(pivot j)]`, whose lowest set bit is above
/// `j` — the loop's own recursion, so the count and the end are the
/// loop's too.
struct BlockTables {
    of_column: Vec<u32>,
    tables: Vec<BlockTable>,
    /// Words written building the leading block's, counted as performed
    /// XORs.
    built_words: u64,
    /// Words the live tables hold.
    held_words: usize,
}

impl BlockTables {
    /// `None` when the tables would take more words than `budget`.
    fn build(pivot_of: &[u32], pivots: &[(usize, Row)], budget: usize) -> Option<Self> {
        let n_cols = pivot_of.len();
        let mut runs: Vec<(usize, usize)> = Vec::new();
        let mut c = 0usize;
        while c < n_cols {
            if pivot_of[c] == NONE {
                c += 1;
                continue;
            }
            let in_word = c % 64;
            let group_end = (c - in_word % TABLE_BLOCK + TABLE_BLOCK)
                .min(c - in_word + 64)
                .min(n_cols);
            let mut e = c + 1;
            while e < group_end && pivot_of[e] != NONE {
                e += 1;
            }
            if e - c >= 2 {
                runs.push((c, e - c));
            }
            c = e;
        }
        let total: usize = runs
            .iter()
            .map(|r| Self::width(r, pivot_of, pivots) << r.1)
            .sum();
        if total > budget {
            return None;
        }
        let mut tables = Self {
            of_column: vec![NONE; n_cols],
            tables: Vec::new(),
            built_words: 0,
            held_words: 0,
        };
        tables.built_words = tables.add(&runs, pivot_of, pivots);
        Some(tables)
    }

    fn width(&(first, k): &(usize, usize), pivot_of: &[u32], pivots: &[(usize, Row)]) -> usize {
        (first..first + k)
            .map(|c| pivots[pivot_of[c] as usize].1.end)
            .max()
            .unwrap()
            - first / 64
    }

    /// The aligned group of up to [`TABLE_BLOCK`] columns, within one word,
    /// that column `c` belongs to.
    fn group(c: usize, n_cols: usize) -> (usize, usize) {
        let in_word = c % 64;
        let start = c - in_word % TABLE_BLOCK;
        (
            start,
            (start + TABLE_BLOCK).min(c - in_word + 64).min(n_cols),
        )
    }

    /// Bring the tables up to date with `new`, pivots just made: re-table
    /// every group one of them lands in, within `budget` words held.
    /// Returns the words written.
    fn extend(
        &mut self,
        pivot_of: &[u32],
        pivots: &[(usize, Row)],
        new: &[(usize, Row)],
        budget: usize,
    ) -> u64 {
        let n_cols = pivot_of.len();
        let mut groups: Vec<(usize, usize)> = new
            .iter()
            .map(|&(lead, _)| Self::group(lead, n_cols))
            .collect();
        groups.sort_unstable();
        groups.dedup();
        let mut runs: Vec<(usize, usize)> = Vec::new();
        for (start, end) in groups {
            // retire the group's old tables
            for c in start..end {
                let t = std::mem::replace(&mut self.of_column[c], NONE);
                if t != NONE {
                    let old = &mut self.tables[t as usize];
                    self.held_words -= old.words.len();
                    old.words = Vec::new();
                }
            }
            let mut c = start;
            while c < end {
                if pivot_of[c] == NONE {
                    c += 1;
                    continue;
                }
                let mut e = c + 1;
                while e < end && pivot_of[e] != NONE {
                    e += 1;
                }
                if e - c >= 2 {
                    runs.push((c, e - c));
                }
                c = e;
            }
        }
        let mut within = Vec::with_capacity(runs.len());
        let mut held = self.held_words;
        for r in runs {
            let words = Self::width(&r, pivot_of, pivots) << r.1;
            if held + words <= budget {
                held += words;
                within.push(r);
            }
        }
        self.add(&within, pivot_of, pivots)
    }

    /// Table `runs` and index them.  Returns the words written.
    fn add(&mut self, runs: &[(usize, usize)], pivot_of: &[u32], pivots: &[(usize, Row)]) -> u64 {
        let pivot = |c: usize| &pivots[pivot_of[c] as usize].1;
        let built: Vec<BlockTable> = runs
            .par_iter()
            .map(|r| {
                let (first, k) = *r;
                let (base, shift) = (first / 64, first % 64);
                let width = Self::width(r, pivot_of, pivots);
                let size = 1usize << k;
                let mut words = vec![0u64; size * width];
                let mut ends = [base; 1 << TABLE_BLOCK];
                let mut counts = [0u64; 1 << TABLE_BLOCK];
                for j in (0..k).rev() {
                    let p = pivot(first + j);
                    let pattern = ((p.bits[base] >> shift) & ((1u64 << k) - 1)) as usize;
                    let (pe, cost) = (p.end, (p.end - base) as u64);
                    for high in 0..size >> (j + 1) {
                        let b = (1 << j) | (high << (j + 1));
                        let dep = b ^ pattern;
                        let (de, end) = (ends[dep], ends[dep].max(pe));
                        let (entry, earlier) = if b < dep {
                            let (lo, hi) = words.split_at_mut(dep * width);
                            (&mut lo[b * width..(b + 1) * width], &hi[..width])
                        } else {
                            let (lo, hi) = words.split_at_mut(b * width);
                            (&mut hi[..width], &lo[dep * width..(dep + 1) * width])
                        };
                        entry[..pe - base].copy_from_slice(&p.bits[base..pe]);
                        for (a, x) in entry[..de - base].iter_mut().zip(&earlier[..de - base]) {
                            *a ^= *x;
                        }
                        ends[b] = end;
                        counts[b] = cost + counts[dep];
                    }
                }
                BlockTable {
                    first,
                    k,
                    width,
                    words,
                    ends,
                    counts,
                }
            })
            .collect();
        let mut written = 0u64;
        for table in built {
            let t = self.tables.len() as u32;
            self.of_column[table.first..table.first + table.k].fill(t);
            written += (1..1usize << table.k)
                .map(|b| (table.ends[b] - table.first / 64) as u64)
                .sum::<u64>();
            self.held_words += table.words.len();
            self.tables.push(table);
        }
        written
    }

    /// Reduce `row`, whose lead is `lead`, by the table covering it, if
    /// any: returns the words the row-by-row loop would have XORed and
    /// the words XORed.
    #[inline]
    fn reduce(&self, row: &mut Row, lead: usize) -> Option<(u64, u64)> {
        let t = self.of_column[lead];
        if t == NONE {
            return None;
        }
        let table = &self.tables[t as usize];
        let base = table.first / 64;
        let b = ((row.bits[base] >> (table.first % 64)) & ((1u64 << table.k) - 1)) as usize;
        let end = table.ends[b];
        let entry = &table.words[b * table.width..b * table.width + (end - base)];
        for (a, x) in row.bits[base..end].iter_mut().zip(entry) {
            *a ^= *x;
        }
        row.end = row.end.max(end);
        Some((table.counts[b], (end - base) as u64))
    }
}

/// Monomials on a symbolic-preprocessing level from which
/// [`groebner_basis_f4`] looks their reducers up in parallel.
const PAR_REDUCERS: usize = 512;

/// Terms, summed over a step's critical-pair (or field) products, from which
/// [`groebner_basis_f4`] forms them in parallel; below it rayon's dispatch
/// (and, the first time, its pool's start-up) costs more than it spreads.
const PAR_PRODUCT_TERMS: usize = 1 << 16;

/// Words per batch after the first in [`echelon`]'s shared-out pass: large
/// enough that sharing a batch out pays for itself and that a range's
/// tables serve many rows while cached, small enough that few of its rows
/// need a pivot another of its rows is about to make.
const ELIMINATION_BATCH_WORDS: usize = 1 << 16;

/// Words of work in one batch below which its first pass runs on one
/// thread.
const PAR_BATCH_WORDS: usize = 1 << 10;

fn inner_build_parallel_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED
        .get_or_init(|| std::env::var("F4_F2_DISABLE_INNER_BUILD_PARALLEL").as_deref() == Ok("0"))
}

fn is_one(p: &F2BoolPoly) -> bool {
    p.terms.len() == 1 && p.terms[0].mask == 0
}

fn degree(p: &F2BoolPoly) -> u32 {
    p.terms.iter().map(|t| t.degree()).max().unwrap_or(0)
}

struct State {
    n_vars: usize,
    polys: Vec<F2BoolPoly>,
    lm: Vec<Mask>,
    active: Vec<bool>,
    pairs: Vec<Pair>,
    pair_select_scratch: PairSelectScratch,
}

#[derive(Default)]
struct PairSelectScratch {
    dense: Option<DensePairSelectScratch>,
    /// The active basis indices of one `insert_core`, rebuilt per call
    /// and kept for its allocation (not part of the dense scratch's
    /// reported bytes).
    active: Vec<usize>,
}

struct DensePairSelectScratch {
    epoch: u32,
    stamp: Vec<u32>,
    first_noncoprime: Vec<u32>,
    noncoprime_count: Vec<u32>,
    has_coprime: Vec<u8>,
    survivor: Vec<u32>,
    touched: Vec<u32>,
}

impl DensePairSelectScratch {
    fn new(size: usize) -> Self {
        Self {
            epoch: 0,
            stamp: vec![0; size],
            first_noncoprime: vec![NONE; size],
            noncoprime_count: vec![0; size],
            has_coprime: vec![0; size],
            survivor: vec![NONE; size],
            touched: Vec::new(),
        }
    }

    fn begin(&mut self) {
        self.epoch = self.epoch.wrapping_add(1);
        if self.epoch == 0 {
            self.stamp.fill(0);
            self.epoch = 1;
        }
        self.touched.clear();
    }

    fn bytes(&self) -> u64 {
        (self.stamp.capacity() * std::mem::size_of::<u32>()
            + self.first_noncoprime.capacity() * std::mem::size_of::<u32>()
            + self.noncoprime_count.capacity() * std::mem::size_of::<u32>()
            + self.has_coprime.capacity() * std::mem::size_of::<u8>()
            + self.survivor.capacity() * std::mem::size_of::<u32>()
            + self.touched.capacity() * std::mem::size_of::<u32>()) as u64
    }
}

fn dense_pair_select_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("F4_F2_DENSE_PAIR_SELECT").as_deref() != Ok("0"))
}

/// The current direct Becker–Weispfenning UPDATE selection.
fn select_new_pairs_quadratic(
    lh: Mask,
    lm: &[Mask],
    active: &[usize],
    st: &mut F4Stats,
) -> Vec<(usize, Mask)> {
    st.pair_quadratic_select_calls += 1;
    st.pair_candidate_visits += active.len() as u64;
    // one word operation a candidate formed, a cover test made and a
    // product-criterion test
    let mut ops = 2 * active.len() as u64;
    let mut candidates: Vec<(usize, Mask)> = active.iter().map(|&g| (g, lh | lm[g])).collect();
    let mut kept = Vec::with_capacity(candidates.len());
    while let Some((g1, l1)) = candidates.pop() {
        let coprime = lh & lm[g1] == 0;
        let covered = candidates.iter().chain(kept.iter()).any(|&(_, l2)| {
            ops += 1;
            l2 & !l1 == 0
        });
        if coprime || !covered {
            kept.push((g1, l1));
        } else {
            st.pairs_chain_skipped += 1;
        }
    }
    tally::word_ops(ops);
    kept.into_iter()
        .filter(|&(g, _)| {
            let keep = lh & lm[g] != 0;
            st.pairs_product_skipped += u64::from(!keep);
            keep
        })
        .collect()
}

/// Exact dense-domain equivalent of [`select_new_pairs_quadratic`].
fn select_new_pairs_dense(
    n_vars: usize,
    lh: Mask,
    lm: &[Mask],
    active: &[usize],
    scratch: &mut PairSelectScratch,
    st: &mut F4Stats,
) -> Vec<(usize, Mask)> {
    debug_assert!(n_vars <= 20);
    st.pair_dense_select_calls += 1;
    st.pair_candidate_visits += active.len() as u64;
    if active.is_empty() {
        return Vec::new();
    }
    let lookups_before = st.pair_cover_lookups;
    let size = 1usize << n_vars;
    if scratch.dense.as_ref().map(|dense| dense.stamp.len()) != Some(size) {
        // five `u32`-or-narrower arrays of `size` cleared
        tally::word_ops((5 * size as u64).div_ceil(2));
        scratch.dense = Some(DensePairSelectScratch::new(size));
    }
    let dense = scratch.dense.as_mut().unwrap();
    dense.begin();
    dense.touched.reserve(active.len());
    for &g in active {
        debug_assert!(g < u32::MAX as usize);
        let lcm = (lh | lm[g]) as usize;
        debug_assert!(lcm < size);
        if dense.stamp[lcm] != dense.epoch {
            dense.stamp[lcm] = dense.epoch;
            dense.first_noncoprime[lcm] = NONE;
            dense.noncoprime_count[lcm] = 0;
            dense.has_coprime[lcm] = 0;
            dense.survivor[lcm] = NONE;
            dense.touched.push(lcm as u32);
        }
        if lh & lm[g] == 0 {
            dense.has_coprime[lcm] = 1;
        } else {
            dense.noncoprime_count[lcm] += 1;
            let first = dense.first_noncoprime[lcm];
            if first == NONE || g < first as usize {
                dense.first_noncoprime[lcm] = g as u32;
            }
        }
    }
    st.pair_lcm_groups += dense.touched.len() as u64;
    st.pair_dense_scratch_bytes_max = st.pair_dense_scratch_bytes_max.max(dense.bytes());
    for &lcm_u32 in &dense.touched {
        let lcm = lcm_u32 as usize;
        let remainder = lcm & !(lh as usize);
        let mut proper_cover = false;
        if remainder != 0 {
            let mut submask = (remainder - 1) & remainder;
            loop {
                st.pair_cover_lookups += 1;
                if dense.stamp[(lh as usize) | submask] == dense.epoch {
                    proper_cover = true;
                    break;
                }
                if submask == 0 {
                    break;
                }
                submask = (submask - 1) & remainder;
            }
        }
        let count = dense.noncoprime_count[lcm] as u64;
        if proper_cover || dense.has_coprime[lcm] != 0 {
            st.pairs_chain_skipped += count;
        } else {
            let first = dense.first_noncoprime[lcm];
            if first != NONE {
                dense.survivor[lcm] = first;
                st.pairs_chain_skipped += count.saturating_sub(1);
            }
        }
    }
    // the two passes over the candidates (an lcm and a test each), one
    // word operation a touched lcm group and one a cover lookup
    tally::word_ops(
        4 * active.len() as u64
            + 2 * dense.touched.len() as u64
            + (st.pair_cover_lookups - lookups_before),
    );
    let mut selected = Vec::new();
    for &g in active.iter().rev() {
        let lcm = (lh | lm[g]) as usize;
        if lh & lm[g] == 0 {
            st.pairs_product_skipped += 1;
        } else if dense.survivor[lcm] == g as u32 {
            selected.push((g, lcm as Mask));
        }
    }
    selected
}

impl State {
    /// Becker–Weispfenning `UPDATE` for the critical pairs, plus the
    /// field pairs of the new element.
    fn insert(&mut self, h_poly: F2BoolPoly, st: &mut F4Stats) {
        self.insert_core(h_poly, st, true);
    }

    /// Insert one F4 step's new elements in order, then scan queued pairs
    /// once for every new leading monomial that came after each pair.
    fn insert_batch(&mut self, polys: Vec<F2BoolPoly>, st: &mut F4Stats) {
        let first = self.polys.len();
        for poly in polys {
            self.insert_core(poly, st, false);
        }
        if self.polys.len() > first {
            self.filter_pairs_since(first, st);
        }
    }

    fn filter_pairs_since(&mut self, first: usize, st: &mut F4Stats) {
        let lm = &self.lm;
        let mut dropped = 0u64;
        // one word operation a pair visited and a criterion test made
        let mut ops = self.pairs.len() as u64;
        self.pairs.retain(|p| match p.kind {
            PairKind::Field(..) => true,
            PairKind::Critical(i, j) => {
                let keep = (first.max(i.max(j) + 1)..lm.len()).all(|h| {
                    ops += 1;
                    let lh = lm[h];
                    lh & !p.lcm != 0 || (lm[i] | lh) == p.lcm || (lm[j] | lh) == p.lcm
                });
                if !keep {
                    dropped += 1;
                }
                keep
            }
        });
        st.pairs_chain_skipped += dropped;
        tally::word_ops(ops);
    }

    fn insert_core(&mut self, h_poly: F2BoolPoly, st: &mut F4Stats, immediate_filter: bool) {
        let h = self.polys.len();
        let lh = h_poly.lt().expect("a new element is non-zero").mask;
        self.polys.push(h_poly);
        self.lm.push(lh);
        self.active.push(false);

        // the field pairs (one a variable of the lead), the scan of the
        // basis for active elements, the deactivation scan, and the new
        // critical pairs pushed
        tally::word_ops(u64::from(lh.count_ones()) + 2 * h as u64);
        let dh = lh.count_ones();
        let mut bits = lh;
        while bits != 0 {
            let v = bits.trailing_zeros();
            bits &= bits - 1;
            self.pairs.push(Pair {
                kind: PairKind::Field(h, v),
                lcm: lh,
                deg: dh + 1,
            });
        }

        // New pairs (h, g): keep one per minimal lcm (chain criterion).
        let mut active = std::mem::take(&mut self.pair_select_scratch.active);
        active.clear();
        active.extend((0..h).filter(|&g| self.active[g]));
        let selected = if dense_pair_select_enabled() && self.n_vars <= 20 {
            select_new_pairs_dense(
                self.n_vars,
                lh,
                &self.lm,
                &active,
                &mut self.pair_select_scratch,
                st,
            )
        } else {
            select_new_pairs_quadratic(lh, &self.lm, &active, st)
        };
        self.pair_select_scratch.active = active;
        // Old pairs whose lcm `h` divides strictly on both sides.
        if immediate_filter {
            self.filter_pairs_since(h, st);
        }
        tally::word_ops(selected.len() as u64);
        for (g, l) in selected {
            self.pairs.push(Pair {
                kind: PairKind::Critical(g, h),
                lcm: l,
                deg: l.count_ones(),
            });
        }
        for g in 0..h {
            if self.active[g] && lh & !self.lm[g] == 0 {
                self.active[g] = false;
            }
        }
        self.active[h] = true;
    }

    /// Active basis elements whose leading monomial divides `m`, the
    /// shortest first.
    /// The active element with the fewest terms whose leading monomial
    /// divides `m` (the first such on a tie).  Tests every active element,
    /// which the caller counts as `active.len()` divisor tests.
    fn reducer_among(&self, m: Mask, active: &[usize]) -> Option<usize> {
        let mut best: Option<usize> = None;
        for &g in active {
            if self.lm[g] & !m == 0
                && best.is_none_or(|b| self.polys[g].terms.len() < self.polys[b].terms.len())
            {
                best = Some(g);
            }
        }
        best
    }
}

/// **Compute the reduced Gröbner basis** of the ideal `initial` generates
/// in `F_2[v]/(v² − v)` by F4, within an optional wall-clock budget.
///
/// On a budget or size overrun the returned set generates the same ideal
/// but is not a Gröbner basis, and the stats say so.
pub fn groebner_basis_f4(
    initial: Vec<F2BoolPoly>,
    n_vars: usize,
    budget: Option<Duration>,
) -> (Vec<F2BoolPoly>, F4Stats) {
    groebner_basis_f4_until(initial, n_vars, budget, None)
}

/// [`groebner_basis_f4`] reading and extending a [`F4Skeleton`] shared
/// with the earlier systems of the run.  The basis and every decision are
/// those of [`groebner_basis_f4`]; only the symbolic work differs, and the
/// stats count what was done.
pub fn groebner_basis_f4_skeleton(
    initial: Vec<F2BoolPoly>,
    n_vars: usize,
    budget: Option<Duration>,
    skeleton: &mut F4Skeleton,
) -> (Vec<F2BoolPoly>, F4Stats) {
    let (basis, mut st) = groebner_basis_f4_inner(
        initial,
        n_vars,
        budget,
        None,
        max_matrix_words(),
        EARLY_GUARD,
        Some(&mut *skeleton),
    );
    st.skeleton_bytes = skeleton.bytes;
    (basis, st)
}

/// The variables the degree-1 elements of `polys` pin: one bit per
/// distinct leading variable.  Under a degree ordering an element whose
/// leading monomial is a variable is affine, so each such element fixes
/// its leading variable once the smaller ones are fixed.
fn pinned_variables(lm: &[Mask], active: &[bool]) -> Mask {
    lm.iter()
        .zip(active)
        .filter(|(m, a)| **a && m.count_ones() == 1)
        .fold(0 as Mask, |acc, (m, _)| acc | m)
}

/// [`groebner_basis_f4`], stopping after the first step at which the
/// degree-1 elements of the basis leave at most `max_free` variables
/// unpinned (`None` never stops early).  The ideal is unchanged, so its
/// solutions all lie on the affine space those elements cut out; a
/// caller enumerates `2^free` points there and checks each against the
/// original equations ([`solutions_from_linear_part`]).
pub fn groebner_basis_f4_until(
    initial: Vec<F2BoolPoly>,
    n_vars: usize,
    budget: Option<Duration>,
    max_free: Option<usize>,
) -> (Vec<F2BoolPoly>, F4Stats) {
    groebner_basis_f4_inner(
        initial,
        n_vars,
        budget,
        max_free,
        max_matrix_words(),
        EARLY_GUARD,
        None,
    )
}

/// Tuning of the early size guard; tests shrink it to reach the guarded
/// path on small systems.
#[derive(Clone, Copy)]
struct EarlyGuard {
    /// Source terms of a step (the sparse terms its S-rows start from)
    /// above which the dimension pre-pass runs before any row is stored.
    terms: usize,
    /// Rows or monomials handled between two checks of the deadline and
    /// of the size bound in the pre-pass.
    slab: usize,
    /// Deadline of the pre-pass (default: the budget's deadline).
    deadline: Option<Instant>,
}

/// 8 bytes a term: 64 MiB of sparse S-row terms.
const EARLY_GUARD: EarlyGuard = EarlyGuard {
    terms: 1 << 23,
    slab: 256,
    deadline: None,
};

/// Dense words of a `rows × cols` bit matrix.
#[inline]
fn dense_words(rows: usize, cols: usize) -> u64 {
    cols.div_ceil(64).max(1) as u64 * rows as u64
}

/// [`groebner_basis_f4_until`] with the dense-matrix guard at `max_words`.
///
/// **Early refusal.**  The guard used to be consulted only after every
/// sparse row of a step existed (8 bytes a term, several times the dense
/// matrix), so an oversized step was OOM-killed before it could be
/// refused, and the budget was checked only in the elimination.  For a
/// step whose S-rows start from more than [`EarlyGuard::terms`] terms, a
/// *dimension pre-pass* now runs first: it forms the S-rows and the
/// reducers one slab at a time, keeps only the set of monomials examined
/// and a row count (memory proportional to the columns), checks the
/// deadline between slabs, stops as soon as the rows counted so far times
/// the columns so far (both lower bounds that only grow) exceed
/// `max_words`, and at its end holds the step's exact rows and columns,
/// the very ones the late check computes.  A step that fits is therefore
/// never refused, and it is then built exactly as before: the pre-pass is
/// uncounted (no `bitop` charge) and changes no result.  The late check
/// stays as the exact one for steps below the threshold.  A refusal sets
/// `oversize` (or `timed_out`), the flags callers already handle.
fn groebner_basis_f4_inner(
    initial: Vec<F2BoolPoly>,
    n_vars: usize,
    budget: Option<Duration>,
    max_free: Option<usize>,
    max_words: u64,
    guard: EarlyGuard,
    mut skel: Option<&mut F4Skeleton>,
) -> (Vec<F2BoolPoly>, F4Stats) {
    static BATCH_INSERTS: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let batch_inserts =
        *BATCH_INSERTS.get_or_init(|| std::env::var("F4_F2_BATCH_INSERTS").as_deref() != Ok("0"));
    let started = Instant::now();
    let deadline = budget.map(|b| started + b);
    let pre_deadline = guard.deadline.or(deadline);
    let mut st = F4Stats {
        inner_build_parallel_disabled_calls: u64::from(!inner_build_parallel_enabled()),
        ..F4Stats::default()
    };
    let one = || vec![F2BoolPoly::one(n_vars)];
    // `bitop/v1` word operations outside the elimination go to
    // `bitop_tally` as they happen; the divisor tests of symbolic
    // preprocessing and interreduction (one mask test each) are already
    // counted in `st` and charged here once.
    let finish = |basis: Vec<F2BoolPoly>, mut st: F4Stats| {
        tally::word_ops(st.divisor_tests);
        st.basis_len = basis.len() as u64;
        st.wall_ns = started.elapsed().as_nanos() as u64;
        if matrix_profile_enabled() {
            st.matrix_profile_unbinned_eliminate_ns = st
                .eliminate_ns
                .saturating_sub(st.matrix_profile_eliminate_ns.iter().copied().sum::<u64>());
            st.matrix_profile_unbinned_logical_xors = st
                .word_xors
                .saturating_sub(st.matrix_profile_logical_xors.iter().copied().sum::<u64>());
            st.matrix_profile_unbinned_performed_xors = st.word_xors_performed.saturating_sub(
                st.matrix_profile_performed_xors
                    .iter()
                    .copied()
                    .sum::<u64>(),
            );
        }
        (basis, st)
    };

    let inputs: Vec<F2BoolPoly> = initial.into_iter().filter(|p| !p.is_zero()).collect();
    // the scans for the constant and the degree, one a term
    tally::word_ops(inputs.iter().map(|p| 1 + p.terms.len() as u64).sum());
    if inputs.iter().any(is_one) {
        return finish(one(), st);
    }
    for p in &inputs {
        st.max_poly_degree = st.max_poly_degree.max(degree(p));
    }
    if let Some(k) = skel.as_deref_mut().filter(|_| !inputs.is_empty()) {
        // the shape key: the unknowns and the inputs' largest degree (the
        // degree scan is the one above), compared with the stored one
        let shape = (n_vars, inputs.iter().map(degree).max().unwrap_or(0));
        tally::word_ops(2);
        if k.shape != Some(shape) {
            st.skeleton_shape_rebuilds = u64::from(k.shape.is_some());
            k.steps.clear();
            k.bytes = 0;
            k.shape = Some(shape);
        }
    }

    // The initial echelon: distinct leading monomials to start from.
    let t = Instant::now();
    let cols = Columns::from_monomials(inputs.iter().flat_map(|p| p.terms.iter().map(|t| t.mask)));
    let rows: Vec<Row> = inputs.iter().map(|p| cols.pack(p)).collect();
    cols.charge_pack(&inputs);
    st.build_ns += t.elapsed().as_nanos() as u64;
    let t = Instant::now();
    let Some(pivots) = echelon(rows, cols.monos.len(), &mut st, deadline) else {
        st.timed_out = true;
        st.eliminate_ns += t.elapsed().as_nanos() as u64;
        return finish(inputs, st);
    };
    st.eliminate_ns += t.elapsed().as_nanos() as u64;
    let mut start: Vec<F2BoolPoly> = pivots.iter().map(|(_, r)| cols.unpack(r, n_vars)).collect();
    if start.iter().any(is_one) {
        return finish(one(), st);
    }
    tally::word_ops(start.len() as u64 + tally::sort_cost(start.len()));
    start.sort_by(|a, b| cmp_mono(a.lt().unwrap(), b.lt().unwrap()));

    let mut s = State {
        n_vars,
        polys: Vec::new(),
        lm: Vec::new(),
        active: Vec::new(),
        pairs: Vec::new(),
        pair_select_scratch: PairSelectScratch::default(),
    };
    if batch_inserts {
        s.insert_batch(start, &mut st);
    } else {
        for p in start {
            s.insert(p, &mut st);
        }
    }

    // Per-step buffers, emptied at the top of every step and reused, so a
    // step allocates them only when it outgrows an earlier one.  Their
    // contents are each step's own; `lcm_columns` is read only by
    // membership and through `examined`, whose values `Columns` sorts, so
    // a reused set's iteration order changes nothing.
    let mut selected: Vec<Pair> = Vec::new();
    let mut seen_rows: FxSet<(Mask, usize)> = FxSet::default();
    let mut half_keys: Vec<(Mask, usize)> = Vec::new();
    let mut field_keys: Vec<(Mask, usize)> = Vec::new();
    let mut lcm_columns: FxSet<Mask> = FxSet::default();
    let mut active: Vec<usize> = Vec::new();
    'steps: while !s.pairs.is_empty() {
        if deadline.is_some_and(|d| Instant::now() >= d) {
            st.timed_out = true;
            break;
        }
        // the scan for the lowest degree and the partition, one a pair
        tally::word_ops(2 * s.pairs.len() as u64);
        let d = s.pairs.iter().map(|p| p.deg).min().unwrap();
        // the partition in place: both sides keep their order
        selected.clear();
        s.pairs.retain(|p| {
            if p.deg == d {
                selected.push(*p);
                false
            } else {
                true
            }
        });
        st.steps += 1;
        st.degree_reached = st.degree_reached.max(d);

        let t = Instant::now();
        // The rows the step reduces: both halves of every critical pair
        // (their common leading monomial is the lcm, which one half will
        // pivot and the other lose), and every field product.
        seen_rows.clear();
        half_keys.clear();
        field_keys.clear();
        lcm_columns.clear();
        // the multipliers (an AND-NOT each) and the hashed row and lcm sets
        for p in &selected {
            match p.kind {
                PairKind::Critical(..) => {
                    tally::word_ops(2);
                    tally::hash_probes(3);
                }
                PairKind::Field(..) => {
                    tally::word_ops(1);
                    tally::hash_probes(1);
                }
            }
        }
        for p in &selected {
            match p.kind {
                PairKind::Critical(i, j) => {
                    st.pairs_reduced += 1;
                    lcm_columns.insert(p.lcm);
                    for g in [i, j] {
                        let mult = p.lcm & !s.lm[g];
                        if seen_rows.insert((mult, g)) {
                            half_keys.push((mult, g));
                        }
                    }
                }
                PairKind::Field(g, v) => {
                    st.field_pairs_reduced += 1;
                    // A field multiplier lies inside `LM(g)` and a half's is
                    // disjoint from it, so the two kinds of key never meet.
                    let mult: Mask = bit(v);
                    if seen_rows.insert((mult, g)) {
                        field_keys.push((mult, g));
                    }
                }
            }
        }
        // The products are independent of one another; form them in
        // parallel when there are enough, in the order they were listed.
        let product = |&(mult, g): &(Mask, usize)| s.polys[g].mul_mono(F2BoolMono::from_mask(mult));
        let products = |keys: &[(Mask, usize)]| -> Vec<F2BoolPoly> {
            let terms: usize = keys.iter().map(|&(_, g)| s.polys[g].terms.len()).sum();
            if keys.len() > 1
                && terms >= PAR_PRODUCT_TERMS
                && inner_build_parallel_enabled()
                && rayon::current_num_threads() > 1
            {
                keys.par_iter().map(product).collect()
            } else {
                keys.iter().map(product).collect()
            }
        };
        // The dimension pre-pass (see `groebner_basis_f4_inner`).
        let src_terms: usize = half_keys
            .iter()
            .chain(&field_keys)
            .map(|&(_, g)| s.polys[g].terms.len())
            .sum();
        if src_terms > guard.terms {
            let act: Vec<usize> = (0..s.polys.len()).filter(|&g| s.active[g]).collect();
            // 1 = oversize, 2 = timed out
            let verdict: u8 = 'pre: {
                let mut rows_n = 0usize;
                let mut srow_monos: FxSet<Mask> = FxSet::default();
                for keys in [&half_keys, &field_keys] {
                    for slab in keys.chunks(guard.slab) {
                        for p in products(slab) {
                            if !p.is_zero() {
                                rows_n += 1;
                                srow_monos.extend(p.terms.iter().map(|t| t.mask));
                            }
                        }
                        if dense_words(rows_n, srow_monos.len()) > max_words {
                            break 'pre 1;
                        }
                        if pre_deadline.is_some_and(|d| Instant::now() >= d) {
                            break 'pre 2;
                        }
                    }
                }
                let mut examined: FxSet<Mask> = lcm_columns.iter().copied().collect();
                let mut frontier: Vec<Mask> = srow_monos
                    .into_iter()
                    .filter(|m| !lcm_columns.contains(m))
                    .collect();
                examined.extend(frontier.iter().copied());
                while !frontier.is_empty() {
                    let mut next: Vec<Mask> = Vec::new();
                    for slab in frontier.chunks(guard.slab) {
                        // a reducer's monomials after its lead (its lead is `m`)
                        let tails = |&m: &Mask| -> Option<Vec<Mask>> {
                            s.reducer_among(m, &act).map(|g| {
                                let r = s.polys[g].mul_mono(F2BoolMono::from_mask(m & !s.lm[g]));
                                r.terms[1..].iter().map(|t| t.mask).collect()
                            })
                        };
                        let found: Vec<Option<Vec<Mask>>> = if slab.len() >= PAR_REDUCERS
                            && inner_build_parallel_enabled()
                            && rayon::current_num_threads() > 1
                        {
                            slab.par_iter().map(tails).collect()
                        } else {
                            slab.iter().map(tails).collect()
                        };
                        for tail in found.into_iter().flatten() {
                            rows_n += 1;
                            for m in tail {
                                if examined.insert(m) {
                                    next.push(m);
                                }
                            }
                        }
                        if dense_words(rows_n, examined.len()) > max_words {
                            break 'pre 1;
                        }
                        if pre_deadline.is_some_and(|d| Instant::now() >= d) {
                            break 'pre 2;
                        }
                    }
                    frontier = next;
                }
                0
            };
            if verdict != 0 {
                if verdict == 1 {
                    st.oversize = true;
                } else {
                    st.timed_out = true;
                }
                st.build_ns += t.elapsed().as_nanos() as u64;
                s.pairs.extend_from_slice(&selected);
                break 'steps;
            }
        }
        let half_rows = products(&half_keys);
        let mut field_rows = products(&field_keys);
        tally::word_ops(
            half_keys
                .iter()
                .chain(&field_keys)
                .map(|&(_, g)| tally::poly::mul_mono(s.polys[g].terms.len()))
                .sum::<u64>()
                + field_rows.len() as u64,
        );
        field_rows.retain(|p| !p.is_zero());

        // Symbolic preprocessing.  Every monomial other than an S-pair's
        // lcm is examined once; a divisible one gets a reducer led by it.
        active.clear();
        active.extend((0..s.polys.len()).filter(|&g| s.active[g]));
        tally::word_ops(s.polys.len() as u64);
        // The skeleton's entry for this step: the key is formed (a word
        // operation a leading monomial), hashed (one more each) and, on a
        // hit, compared (one more each).
        let mut skel_added = 0u64;
        type StepRefs<'k> = (Option<&'k mut SkeletonStep>, Option<&'k mut u64>, bool);
        let (mut entry, mut skel_bytes, recording): StepRefs<'_> = match skel.as_deref_mut() {
            None => (None, None, false),
            Some(k) => {
                let F4Skeleton {
                    steps,
                    bytes,
                    cap_bytes,
                    ..
                } = k;
                let key: (u32, Box<[Mask]>) = (d, active.iter().map(|&g| s.lm[g]).collect());
                tally::word_ops(2 * active.len() as u64 + 1);
                tally::hash_probes(1);
                let recording = *bytes < *cap_bytes;
                if steps.contains_key(&key) {
                    tally::word_ops(active.len() as u64 + 1);
                    st.skeleton_steps_replayed += 1;
                    (steps.get_mut(&key), Some(bytes), recording)
                } else if recording {
                    st.skeleton_steps_built += 1;
                    tally::hash_probes(1);
                    skel_added += 32 + 8 * active.len() as u64;
                    (Some(steps.entry(key).or_default()), Some(bytes), true)
                } else {
                    st.skeleton_store_refused += 1;
                    (None, None, false)
                }
            }
        };
        let replaying = entry
            .as_deref()
            .is_some_and(|e| !e.candidates.is_empty() || e.columns.is_some());
        let mut no_divisor = MonomialSeen::new(n_vars);
        // The first level: every monomial of the S-rows but the lcms.
        let s_terms: usize = half_rows
            .iter()
            .chain(&field_rows)
            .map(|p| p.terms.len())
            .sum();
        let par_terms = s_terms >= PAR_PRODUCT_TERMS
            && inner_build_parallel_enabled()
            && rayon::current_num_threads() > 1;
        // (The S-rows carry many times more terms than distinct monomials,
        // so each share of them is deduplicated into a set of its own.)
        if par_terms {
            // every term into a hashed set (the non-parallel path charges
            // its own inserts), and the merge of the shares
            tally::hash_probes(2 * s_terms as u64);
        }
        let mut queue: Vec<Mask> = if par_terms {
            half_rows
                .par_iter()
                .chain(field_rows.par_iter())
                .fold(FxSet::default, |mut seen: FxSet<Mask>, p| {
                    seen.extend(p.terms.iter().map(|t| t.mask));
                    seen
                })
                .reduce(FxSet::default, |a, b| {
                    let (mut big, small) = if a.len() >= b.len() { (a, b) } else { (b, a) };
                    big.extend(small);
                    big
                })
                .into_iter()
                .collect()
        } else {
            let mut seen = MonomialSeen::new(n_vars);
            seen.extend(
                half_rows
                    .iter()
                    .chain(&field_rows)
                    .flat_map(|p| p.terms.iter().map(|t| t.mask)),
            );
            seen.into_values()
        };
        // a fixed order, so the reducers come out in one
        tally::word_ops(tally::sort_cost(queue.len()));
        tally::hash_probes(queue.len() as u64);
        queue.sort_unstable();
        queue.retain(|m| !lcm_columns.contains(m));
        let mut examined = MonomialSeen::new(n_vars);
        examined.extend(lcm_columns.iter().copied());
        examined.extend(queue.iter().copied());
        let mut reducers: Vec<F2BoolPoly> = Vec::new();
        // Level by level: the reducers of every monomial on the frontier,
        // then the monomials they carry that nobody has examined.  The
        // monomials examined, the reducers and the count are those of any
        // order of examination — a monomial's reducer depends only on the
        // monomial — and a large level's lookups, products and membership
        // tests are independent of one another, so they run in parallel.
        let product_of =
            |m: Mask, g: usize| s.polys[g].mul_mono(F2BoolMono::from_mask(m & !s.lm[g]));
        // the terms a reducer row carries, on average, to price a level
        let mean_terms = active
            .iter()
            .map(|&g| s.polys[g].terms.len())
            .sum::<usize>()
            / active.len().max(1);
        tally::word_ops(active.len() as u64);
        let mut frontier = queue;
        while !frontier.is_empty() {
            // (sizes first: asking rayon its thread count starts its pool)
            let parallel = frontier.len() >= PAR_REDUCERS
                && frontier.len() * mean_terms >= PAR_PRODUCT_TERMS
                && inner_build_parallel_enabled()
                && rayon::current_num_threads() > 1;
            // A monomial's reducer: from the stored divisor positions when
            // the skeleton has them (only those tested), else by a scan of
            // every active element (recording the divisors when storing).
            // Either way the shortest divisor, the first on a tie, in basis
            // order: the reducer [`State::reducer_among`] picks.
            let candidates = entry.as_deref().map(|e| &e.candidates);
            let lookup = |m: Mask| -> (Option<F2BoolPoly>, Option<Box<[u32]>>, bool) {
                let shortest = |positions: &mut dyn Iterator<Item = usize>| {
                    let mut best: Option<usize> = None;
                    for p in positions {
                        let g = active[p];
                        if best.is_none_or(|b| s.polys[g].terms.len() < s.polys[b].terms.len()) {
                            best = Some(g);
                        }
                    }
                    best
                };
                if let Some(c) = candidates.and_then(|c| c.get(&m)) {
                    let g = shortest(&mut c.iter().map(|&p| p as usize));
                    (g.map(|g| product_of(m, g)), None, true)
                } else if recording {
                    let found: Box<[u32]> = (0..active.len())
                        .filter(|&p| s.lm[active[p]] & !m == 0)
                        .map(|p| p as u32)
                        .collect();
                    let g = shortest(&mut found.iter().map(|&p| p as usize));
                    (g.map(|g| product_of(m, g)), Some(found), false)
                } else {
                    let g = s.reducer_among(m, &active);
                    (g.map(|g| product_of(m, g)), None, false)
                }
            };
            let looked: Vec<(Option<F2BoolPoly>, Option<Box<[u32]>>, bool)> = if parallel {
                frontier.par_iter().map(|&m| lookup(m)).collect()
            } else {
                frontier.iter().map(|&m| lookup(m)).collect()
            };
            let mut rows: Vec<Option<F2BoolPoly>> = Vec::with_capacity(looked.len());
            let mut fresh_candidates: Vec<(Mask, Box<[u32]>)> = Vec::new();
            for (&m, (row, found, hit)) in frontier.iter().zip(looked) {
                if hit {
                    let c = candidates.and_then(|c| c.get(&m)).map_or(0, |c| c.len());
                    st.skeleton_candidate_hits += 1;
                    st.skeleton_candidate_tests += c as u64;
                    st.divisor_tests += c as u64;
                    // the lookup
                    tally::hash_probes(1);
                } else {
                    st.divisor_tests += active.len() as u64;
                    if let Some(found) = found {
                        if replaying {
                            st.skeleton_candidate_misses += 1;
                        }
                        // the failed lookup (when there was a stored map to
                        // look in), the store, and the positions copied
                        tally::hash_probes(1 + u64::from(candidates.is_some()));
                        tally::word_ops(found.len() as u64);
                        skel_added += 24 + 4 * found.len() as u64;
                        fresh_candidates.push((m, found));
                    }
                }
                rows.push(row);
            }
            if let Some(e) = entry.as_deref_mut() {
                e.candidates.extend(fresh_candidates);
            }
            // a reducer leads with its monomial, already examined
            let unseen = |r: &F2BoolPoly| -> Vec<Mask> {
                debug_assert!(r.lt().is_some_and(|t| examined.contains(&t.mask)));
                r.terms[1..]
                    .iter()
                    .map(|t| t.mask)
                    .filter(|t| !examined.contains(t))
                    .collect()
            };
            let mut next: Vec<Mask> = if parallel {
                rows.par_iter().flatten().flat_map_iter(unseen).collect()
            } else {
                rows.iter().flatten().flat_map(unseen).collect()
            };
            // each reducer's product, the membership test of each of its
            // terms after the lead, the sort and dedup of the next level
            let reducer_terms: u64 = rows
                .iter()
                .flatten()
                .map(|r| tally::poly::mul_mono(r.terms.len()))
                .sum();
            let tested: usize = rows
                .iter()
                .flatten()
                .map(|r| r.terms.len().saturating_sub(1))
                .sum();
            tally::word_ops(reducer_terms + tally::sort_cost(next.len()) + next.len() as u64);
            examined.charge(tested as u64);
            if parallel {
                next.par_sort_unstable();
            } else {
                next.sort_unstable();
            }
            next.dedup();
            examined.extend(next.iter().copied());
            for (m, row) in frontier.into_iter().zip(rows) {
                match row {
                    Some(r) => {
                        debug_assert_eq!(
                            r.lt().map(|t| t.mask),
                            Some(m),
                            "a reducer must lead with its monomial"
                        );
                        reducers.push(r);
                    }
                    None => {
                        no_divisor.insert(m);
                        no_divisor.charge(1);
                    }
                }
            }
            frontier = next;
        }
        st.reducer_rows += reducers.len() as u64;

        if let Some(b) = skel_bytes.as_deref_mut() {
            *b = b.saturating_add(std::mem::take(&mut skel_added));
        }
        let n_rows = reducers.len() + half_rows.len() + field_rows.len();
        // The column map: the stored one when the step's monomial set is
        // exactly its set (equal size, every stored monomial examined:
        // a membership test each), else a new sort and index.
        let stored = entry.as_deref_mut().and_then(|e| e.columns.take());
        let stored_bytes = stored.as_ref().map_or(0, F4Skeleton::columns_bytes);
        let cols = match stored {
            Some(c) => {
                let same = c.monos.len() == examined.len() && {
                    examined.charge(c.monos.len() as u64);
                    c.monos.iter().all(|m| examined.contains(m))
                };
                tally::word_ops(1);
                if same {
                    st.skeleton_columns_reused += 1;
                    c
                } else {
                    st.skeleton_columns_rebuilt += 1;
                    Columns::from_monomials(examined.into_values())
                }
            }
            None => Columns::from_monomials(examined.into_values()),
        };
        let words = cols.words() as u64;
        if words * n_rows as u64 > max_words {
            st.oversize = true;
            st.build_ns += t.elapsed().as_nanos() as u64;
            s.pairs.extend_from_slice(&selected);
            break;
        }
        st.matrix_rows_max = st.matrix_rows_max.max(n_rows as u64);
        st.matrix_cols_max = st.matrix_cols_max.max(cols.monos.len() as u64);
        st.matrix_rows_sum += n_rows as u64;
        st.peak_matrix_bytes = st.peak_matrix_bytes.max(words * n_rows as u64 * 8);
        // Reducers first, each on a column of its own; then the S-rows by
        // leading monomial, so the second half of a pair meets the first.
        let mut s_rows: Vec<&F2BoolPoly> = half_rows.iter().chain(field_rows.iter()).collect();
        // the keys, the sort, and the degree scan of every row below
        tally::word_ops(s_rows.len() as u64 + tally::sort_cost(s_rows.len()) + n_rows as u64);
        // stable: rows sharing a leading monomial keep their order
        s_rows.sort_by_cached_key(|p| std::cmp::Reverse(mono_key(p.lt().unwrap())));
        // Every row is a monomial multiple, canonical, so its degree is its
        // leading term's.
        for p in reducers.iter().chain(s_rows.iter().copied()) {
            debug_assert!(p.is_canonical());
            let lead = p.terms.first().map_or(0, |t| t.degree());
            st.max_poly_degree = st.max_poly_degree.max(lead);
        }
        let all: Vec<&F2BoolPoly> = reducers.iter().chain(s_rows).collect();
        // Packing is one lookup per term and independent across rows.
        let rows: Vec<Row> = if all.len() * cols.words() > PAR_WORDS
            && inner_build_parallel_enabled()
            && rayon::current_num_threads() > 1
        {
            all.par_iter().map(|p| cols.pack(p)).collect()
        } else {
            all.iter().map(|p| cols.pack(p)).collect()
        };
        cols.charge_pack(all.iter().copied());
        st.build_ns += t.elapsed().as_nanos() as u64;

        let t = Instant::now();
        let pair_row_count = all.len() - reducers.len();
        let Some(pivots) = (if fl_shape(reducers.len(), pair_row_count, cols.monos.len()) {
            echelon_fl(rows, cols.monos.len(), reducers.len(), &mut st, deadline)
        } else {
            echelon(rows, cols.monos.len(), &mut st, deadline)
        }) else {
            st.timed_out = true;
            st.eliminate_ns += t.elapsed().as_nanos() as u64;
            break;
        };
        st.eliminate_ns += t.elapsed().as_nanos() as u64;

        no_divisor.charge(pivots.len() as u64);
        let mut fresh: Vec<F2BoolPoly> = pivots
            .iter()
            .filter(|(lead, _)| no_divisor.contains(&cols.monos[*lead]))
            .map(|(_, r)| cols.unpack(r, n_vars))
            .collect();
        // Keep the column map for the next system's step of this key (a
        // move, no copy), and account the skeleton's growth.
        let mut skel_removed = 0u64;
        if let Some(e) = entry.as_deref_mut() {
            skel_added += F4Skeleton::columns_bytes(&cols);
            skel_removed = stored_bytes;
            e.columns = Some(cols);
        }
        if let Some(b) = skel_bytes {
            *b = b.saturating_add(skel_added).saturating_sub(skel_removed);
        }
        tally::word_ops(fresh.len() as u64 + tally::sort_cost(fresh.len()));
        if fresh.iter().any(is_one) {
            return finish(one(), st);
        }
        if !fresh.is_empty() {
            st.solving_degree = st.solving_degree.max(d);
        }
        fresh.sort_by(|a, b| cmp_mono(a.lt().unwrap(), b.lt().unwrap()));
        st.new_elements += fresh.len() as u64;
        if batch_inserts {
            s.insert_batch(fresh, &mut st);
        } else {
            for p in fresh {
                s.insert(p, &mut st);
            }
        }
        if let Some(k) = max_free {
            let pinned = pinned_variables(&s.lm, &s.active);
            let free = n_vars as u64 - u64::from(pinned.count_ones());
            if free <= k as u64 {
                st.early_stopped = true;
                st.free_at_stop = free;
                break;
            }
        }
    }
    st.pairs_left = s.pairs.len() as u64;
    // the scan of the active elements and the copy of their terms
    tally::word_ops(
        (0..s.polys.len())
            .map(|g| {
                1 + if s.active[g] {
                    s.polys[g].terms.len() as u64
                } else {
                    0
                }
            })
            .sum(),
    );
    if st.early_stopped && !st.timed_out && !st.oversize {
        let basis: Vec<F2BoolPoly> = (0..s.polys.len())
            .filter(|&g| s.active[g])
            .map(|g| s.polys[g].clone())
            .collect();
        return finish(basis, st);
    }
    if st.timed_out || st.oversize {
        let basis: Vec<F2BoolPoly> = (0..s.polys.len())
            .filter(|&g| s.active[g])
            .map(|g| s.polys[g].clone())
            .collect();
        return finish(basis, st);
    }
    let active: Vec<F2BoolPoly> = (0..s.polys.len())
        .filter(|&g| s.active[g])
        .map(|g| s.polys[g].clone())
        .collect();
    let reduced = interreduce(active, s.n_vars, &mut st);
    finish(reduced, st)
}

/// Keep one element per minimal leading monomial, then reduce every tail
/// by Gauss–Jordan on the symbolic-preprocessing matrix of that minimal
/// basis.  Returns the reduced Gröbner basis, sorted by leading monomial,
/// largest first.
fn interreduce(mut elements: Vec<F2BoolPoly>, n_vars: usize, st: &mut F4Stats) -> Vec<F2BoolPoly> {
    let t = Instant::now();
    // word operations: the sort, every divisibility test of the minimality
    // scan, the membership tests, products and sorts below, and one test a
    // row above each pivot of the backward pass
    let mut ops = tally::sort_cost(elements.len());
    elements.sort_by(|a, b| cmp_mono(a.lt().unwrap(), b.lt().unwrap()));
    let mut minimal: Vec<F2BoolPoly> = Vec::new();
    for p in elements {
        let l = p.lt().unwrap().mask;
        if !minimal.iter().any(|q| {
            ops += 1;
            q.lt().unwrap().mask & !l == 0
        }) {
            minimal.push(p);
        }
    }
    let lms: Vec<Mask> = minimal.iter().map(|p| p.lt().unwrap().mask).collect();
    let mut examined = MonomialSeen::new(n_vars);
    examined.extend(lms.iter().copied());
    let mut queue: Vec<Mask> = Vec::new();
    let mut inserts = 0u64;
    for p in &minimal {
        inserts += (p.terms.len() - 1) as u64;
        for t in &p.terms[1..] {
            if examined.insert(t.mask) {
                queue.push(t.mask);
            }
        }
    }
    let mut reducers: Vec<F2BoolPoly> = Vec::new();
    while let Some(m) = queue.pop() {
        let mut best: Option<usize> = None;
        for (k, &l) in lms.iter().enumerate() {
            st.divisor_tests += 1;
            if l & !m == 0 && best.is_none_or(|b| minimal[k].terms.len() < minimal[b].terms.len()) {
                best = Some(k);
            }
        }
        if let Some(k) = best {
            let r = minimal[k].mul_mono(F2BoolMono::from_mask(m & !lms[k]));
            ops += tally::poly::mul_mono(minimal[k].terms.len());
            inserts += r.terms.len() as u64;
            for t in &r.terms {
                if examined.insert(t.mask) {
                    queue.push(t.mask);
                }
            }
            reducers.push(r);
        }
    }
    examined.charge(inserts);
    let cols = Columns::from_monomials(examined.into_values());
    cols.charge_pack(minimal.iter().chain(reducers.iter()));
    let mut rows: Vec<(usize, Row)> = minimal
        .iter()
        .chain(reducers.iter())
        .map(|p| {
            let mut r = cols.pack(p);
            let lead = r.lead_counted(&mut ops).expect("non-zero");
            (lead, r)
        })
        .collect();
    let n_rows = rows.len() as u64;
    st.matrix_rows_max = st.matrix_rows_max.max(n_rows);
    st.matrix_cols_max = st.matrix_cols_max.max(cols.monos.len() as u64);
    st.matrix_rows_sum += n_rows;
    st.peak_matrix_bytes = st.peak_matrix_bytes.max(cols.words() as u64 * n_rows * 8);
    st.build_ns += t.elapsed().as_nanos() as u64;

    // Every row already leads on a column of its own, so the forward pass
    // is empty; the backward pass clears each pivot column from every row
    // above it, rightmost pivot first.
    let t = Instant::now();
    ops += tally::sort_cost(rows.len()) + (rows.len() * rows.len().saturating_sub(1) / 2) as u64;
    rows.sort_by_key(|(lead, _)| *lead);
    for k in (0..rows.len()).rev() {
        let (lead, from) = (rows[k].0, rows[k].0 / 64);
        let (bit_word, bit) = (lead / 64, 1u64 << (lead % 64));
        let (above, rest) = rows.split_at_mut(k);
        let pivot = &rest[0].1;
        for (_, row) in above.iter_mut() {
            if row.bits[bit_word] & bit != 0 {
                let to = pivot.end;
                for (a, b) in row.bits[from..to].iter_mut().zip(&pivot.bits[from..to]) {
                    *a ^= *b;
                }
                st.word_xors += (to - from) as u64;
                st.word_xors_performed += (to - from) as u64;
                row.end = row.end.max(to);
            }
        }
    }
    st.eliminate_ns += t.elapsed().as_nanos() as u64;
    let lm_set: HashSet<Mask> = lms.iter().copied().collect();
    tally::hash_probes((lms.len() + rows.len()) as u64);
    let mut out: Vec<F2BoolPoly> = rows
        .iter()
        .filter(|(lead, _)| lm_set.contains(&cols.monos[*lead]))
        .map(|(_, r)| cols.unpack(r, n_vars))
        .collect();
    ops += tally::sort_cost(out.len());
    tally::word_ops(ops);
    out.sort_by(|a, b| cmp_mono(b.lt().unwrap(), a.lt().unwrap()));
    out
}

/// **The solutions of a reduced boolean Gröbner basis**, without
/// enumerating `{0,1}^n`.
///
/// Every variable is either the leading monomial of a linear element
/// `v_p + Σ c_q v_q + c` (with every `v_q` free) or free itself, so an
/// assignment of the free variables fixes the point.  The number of free
/// variables is at most the number of solutions less one, since the
/// basis of a radical zero-dimensional ideal has as many standard
/// monomials as the ideal has points.  Returns `None` above `max_free`
/// free variables, which on a reduced basis means the ideal has more than
/// `2^max_free` solutions, or the input was not a reduced basis.
///
/// `tests` counts monomial tests, one per term evaluated.
pub fn solutions_from_reduced_basis(
    gb: &[F2BoolPoly],
    n_vars: usize,
    max_free: usize,
    tests: &mut u64,
) -> Option<Vec<Mask>> {
    if gb.iter().any(is_one) {
        return Some(Vec::new());
    }
    let mut linear_of = vec![None; n_vars];
    for (k, g) in gb.iter().enumerate() {
        let l = g.lt()?.mask;
        if l.count_ones() == 1 {
            linear_of[l.trailing_zeros() as usize] = Some(k);
        }
    }
    let free: Vec<usize> = (0..n_vars).filter(|&v| linear_of[v].is_none()).collect();
    if free.len() > max_free {
        return None;
    }
    let pivots: Vec<(usize, usize)> = (0..n_vars)
        .filter_map(|v| linear_of[v].map(|k| (v, k)))
        .collect();
    let mut out = Vec::new();
    for a in 0u64..(1u64 << free.len()) {
        let mut point: Mask = 0;
        for (i, &v) in free.iter().enumerate() {
            if (a >> i) & 1 == 1 {
                point |= bit(v as u32);
            }
        }
        for &(v, k) in &pivots {
            // The tail involves free variables and the constant only.
            *tests += gb[k].terms.len() as u64;
            if gb[k].eval(point) == 1 {
                point |= bit(v as u32);
            }
        }
        let mut ok = true;
        for g in gb {
            *tests += g.terms.len() as u64;
            if g.eval(point) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(point);
        }
    }
    Some(out)
}

/// Every solution of `equations` on the affine space the degree-1
/// elements of `basis` cut out, `basis` any generating set of the same
/// ideal (an early-stopped F4 run): Gauss–Jordan on the affine elements,
/// then each of the `2^free` points checked against `equations`.
/// Returns `None` above `max_free` free variables.  Sound and complete:
/// every zero of the ideal satisfies the affine elements, and every point
/// returned satisfies `equations`.
///
/// `tests` counts monomial tests, one per term evaluated; `xors` counts
/// the word XORs of the elimination.
pub fn solutions_from_linear_part(
    basis: &[F2BoolPoly],
    equations: &[F2BoolPoly],
    n_vars: usize,
    max_free: usize,
    tests: &mut u64,
    xors: &mut u64,
) -> Option<Vec<Mask>> {
    if basis.iter().any(is_one) {
        return Some(Vec::new());
    }
    if n_vars > MASK_BITS {
        return None;
    }
    // Rows: (variable mask, constant).  Reduce to one pivot per variable.
    let mut rows: Vec<(Mask, u64)> = Vec::new();
    for g in basis {
        if degree(g) > 1 {
            continue;
        }
        let mut mask = 0 as Mask;
        let mut c = 0u64;
        for t in &g.terms {
            if t.mask == 0 {
                c ^= 1;
            } else {
                mask ^= t.mask;
            }
        }
        rows.push((mask, c));
    }
    let mut pivots: Vec<(usize, Mask, u64)> = Vec::new();
    for (mut mask, mut c) in rows {
        for &(v, pm, pc) in &pivots {
            if (mask >> v) & 1 == 1 {
                mask ^= pm;
                c ^= pc;
                *xors += 1;
            }
        }
        if mask == 0 {
            if c == 1 {
                return Some(Vec::new());
            }
            continue;
        }
        let v = mask.trailing_zeros() as usize;
        for p in pivots.iter_mut() {
            if (p.1 >> v) & 1 == 1 {
                p.1 ^= mask;
                p.2 ^= c;
                *xors += 1;
            }
        }
        pivots.push((v, mask, c));
    }
    let pinned = pivots
        .iter()
        .fold(0 as Mask, |acc, p| acc | bit(p.0 as u32));
    let free: Vec<usize> = (0..n_vars).filter(|&v| (pinned >> v) & 1 == 0).collect();
    if free.len() > max_free {
        return None;
    }
    let mut out = Vec::new();
    let assignments = 1u64.checked_shl(free.len().try_into().ok()?)?;
    for a in 0u64..assignments {
        let mut point = 0 as Mask;
        for (offset, &v) in free.iter().enumerate() {
            if (a >> offset) & 1 == 1 {
                point |= bit(v as u32);
            }
        }
        // Each pivot row is v + (free variables) + c, fully reduced.
        for &(v, m, c) in &pivots {
            *tests += 1;
            let rest = m & !bit(v as u32);
            if ((point & rest).count_ones() as u64 + c) & 1 == 1 {
                point |= bit(v as u32);
            }
        }
        let mut ok = true;
        for e in equations {
            *tests += e.terms.len() as u64;
            if e.eval(point) != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            out.push(point);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_enumeration_refuses_an_unrepresentable_assignment_count() {
        let mut tests = 0;
        let mut xors = 0;
        assert!(solutions_from_linear_part(&[], &[], 64, 64, &mut tests, &mut xors).is_none());
    }

    #[cfg(feature = "wide-vars")]
    #[test]
    fn affine_roots_and_pinned_variables_preserve_coordinates_above_sixty_four() {
        let n = 96;
        let free = 80;
        let basis: Vec<_> = (0..n)
            .filter(|&v| v != free)
            .map(|v| {
                let mut terms = vec![F2BoolMono::from_mask(bit(v as u32))];
                if v == 95 {
                    terms.push(F2BoolMono::from_mask(0));
                }
                F2BoolPoly::from_monos(terms, n)
            })
            .collect();
        let (mut tests, mut xors) = (0, 0);
        assert_eq!(
            solutions_from_linear_part(&basis, &basis, n, 1, &mut tests, &mut xors),
            Some(vec![bit(95), bit(95) | bit(free as u32)])
        );
        assert_eq!(
            pinned_variables(&[bit(95), bit(1)], &[true, true]),
            bit(95) | bit(1)
        );
    }

    #[allow(unused_imports)]
    use crate::cryptanalysis::pq_groebner_f2::{
        bit, low_bits, mask_from64, mask_low64, masks_low64, Mask, MASK_BITS,
    };
    use crate::cryptanalysis::pq_groebner_f2::{groebner_basis_f2, reduce, spoly};
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// **Stopping early and enumerating the affine part finds the full
    /// run's solutions**: on random dense quadratic systems with planted
    /// and without planted solutions, for every stopping threshold.
    #[test]
    fn early_stopped_f4_finds_the_full_runs_solutions() {
        let mut rng = StdRng::seed_from_u64(29);
        for case in 0..40 {
            let n = 8 + case % 7;
            let planted = (case % 3 != 0).then(|| rng.gen::<u64>() & ((1u64 << n) - 1));
            let m = n + 2 + case % 4;
            let eqs: Vec<F2BoolPoly> = (0..m)
                .map(|_| {
                    let mut masks = Vec::new();
                    for a in 0..n {
                        for b in a..n {
                            if rng.gen_bool(0.5) {
                                masks.push((1u64 << a) | (1u64 << b));
                            }
                        }
                    }
                    if rng.gen_bool(0.5) {
                        masks.push(0);
                    }
                    let mut p = F2BoolPoly::from_monos(
                        masks
                            .iter()
                            .map(|&m| F2BoolMono::from_mask(mask_from64(m)))
                            .collect(),
                        n,
                    );
                    if let Some(x) = planted {
                        if p.eval(mask_from64(x)) == 1 {
                            p = p.add(&F2BoolPoly::one(n));
                        }
                    }
                    p
                })
                .collect();
            let (gb, _) = groebner_basis_f4(eqs.clone(), n, None);
            let mut t = 0;
            let mut full = solutions_from_reduced_basis(&gb, n, 24, &mut t).unwrap();
            full.retain(|&v| eqs.iter().all(|e| e.eval(v) == 0));
            full.sort_unstable();
            if let Some(x) = planted {
                assert!(
                    full.contains(&mask_from64(x)),
                    "case {case}: the planted point solves"
                );
            }
            for k in [0usize, 2, 5, 8] {
                let (part, st) = groebner_basis_f4_until(eqs.clone(), n, None, Some(k));
                let (mut tests, mut xors) = (0, 0);
                let mut got = solutions_from_linear_part(&part, &eqs, n, 24, &mut tests, &mut xors)
                    .expect("at most 24 free variables");
                got.sort_unstable();
                assert_eq!(
                    got, full,
                    "case {case}, free {k}, early {}",
                    st.early_stopped
                );
            }
        }
    }

    #[test]
    fn monomial_bitmap_matches_hash_set() {
        let mut rng = StdRng::seed_from_u64(0x5eed_2026);
        for n_vars in [0usize, 1, 6, 12, 20, 22, 23] {
            let mut bitmap = MonomialSeen::with_bitmap(n_vars, true);
            let mut hashed = MonomialSeen::with_bitmap(n_vars, false);
            let within = (1u64 << n_vars) - 1;
            let masks = (0..2_000)
                .map(|i| {
                    if i % 17 == 0 {
                        1u64 << 40 | rng.gen::<u64>() & within
                    } else {
                        rng.gen::<u64>() & within
                    }
                })
                .collect::<Vec<_>>();
            for &m in &masks {
                assert_eq!(
                    bitmap.insert(mask_from64(m)),
                    hashed.insert(mask_from64(m)),
                    "n_vars={n_vars}"
                );
                assert_eq!(
                    bitmap.contains(&mask_from64(m)),
                    hashed.contains(&mask_from64(m))
                );
            }
            let mut actual = bitmap.into_values();
            let mut expected = hashed.into_values();
            actual.sort_unstable();
            expected.sort_unstable();
            assert_eq!(actual, expected, "n_vars={n_vars}");
        }
    }

    /// The row-by-row forward elimination `echelon` must match: each row
    /// reduced by the pivots before it while its lead has one.
    fn echelon_reference(rows: &[Row], n_cols: usize) -> (Vec<(usize, Vec<u64>, usize)>, u64) {
        let mut pivot_of = vec![NONE; n_cols];
        let mut pivots: Vec<(usize, Vec<u64>, usize)> = Vec::new();
        let mut xors = 0u64;
        for r in rows {
            let mut row = Row {
                bits: r.bits.clone(),
                start: r.start,
                end: r.end,
            };
            while let Some(lead) = row.lead() {
                match pivot_of[lead] {
                    NONE => {
                        pivot_of[lead] = pivots.len() as u32;
                        pivots.push((lead, row.bits.clone(), row.end));
                        break;
                    }
                    p => {
                        let (_, bits, end) = &pivots[p as usize];
                        for w in lead / 64..*end {
                            row.bits[w] ^= bits[w];
                        }
                        xors += (*end - lead / 64) as u64;
                        row.end = row.end.max(*end);
                    }
                }
            }
        }
        (pivots, xors)
    }

    fn canonical_rref(mut matrix: Vec<Vec<u64>>, n_cols: usize) -> Vec<Vec<u64>> {
        let words = n_cols.div_ceil(64);
        let mut pivot_row = 0usize;
        for column in 0..n_cols {
            let (word, bit) = (column / 64, 1u64 << (column % 64));
            let Some(found) = (pivot_row..matrix.len()).find(|&row| matrix[row][word] & bit != 0)
            else {
                continue;
            };
            matrix.swap(pivot_row, found);
            let pivot = matrix[pivot_row].clone();
            for row in 0..matrix.len() {
                if row != pivot_row && matrix[row][word] & bit != 0 {
                    for index in word..words {
                        matrix[row][index] ^= pivot[index];
                    }
                }
            }
            pivot_row += 1;
            if pivot_row == matrix.len() {
                break;
            }
        }
        matrix.truncate(pivot_row);
        matrix
    }

    #[test]
    fn full_m4ri_matches_gaussian_rank_pivots_and_row_space() {
        let mut rng = StdRng::seed_from_u64(0xf411_4d52_1202_6101);
        for (n_rows, n_cols, density) in [
            (128usize, 257usize, 2u32),
            (173, 319, 4),
            (211, 385, 8),
            (257, 511, 16),
        ] {
            let words = n_cols.div_ceil(64);
            let mut raw = Vec::with_capacity(n_rows);
            for _ in 0..n_rows {
                let mut bits = vec![0u64; words];
                for column in 0..n_cols {
                    if rng.gen_ratio(1, density) {
                        bits[column / 64] |= 1u64 << (column % 64);
                    }
                }
                raw.push(bits);
            }
            let make_rows = || {
                raw.iter()
                    .cloned()
                    .map(|bits| Row {
                        bits,
                        start: 0,
                        end: words,
                    })
                    .collect::<Vec<_>>()
            };
            let (reference, _) = echelon_reference(&make_rows(), n_cols);
            let mut stats = F4Stats::default();
            let candidate = echelon_full_m4ri(make_rows(), n_cols, &mut stats, None).unwrap();
            assert_eq!(candidate.len(), reference.len(), "shape {n_rows}x{n_cols}");
            let mut candidate_columns = candidate.iter().map(|(lead, _)| *lead).collect::<Vec<_>>();
            let mut reference_columns = reference
                .iter()
                .map(|(lead, _, _)| *lead)
                .collect::<Vec<_>>();
            candidate_columns.sort_unstable();
            reference_columns.sort_unstable();
            assert_eq!(
                candidate_columns, reference_columns,
                "pivot columns at shape {n_rows}x{n_cols}"
            );
            let expected_space = canonical_rref(raw.clone(), n_cols);
            let actual_space = canonical_rref(
                candidate.iter().map(|(_, row)| row.bits.clone()).collect(),
                n_cols,
            );
            assert_eq!(
                actual_space, expected_space,
                "row space at shape {n_rows}x{n_cols}"
            );
            assert_eq!(stats.full_m4ri_matrices, 1);
            assert!(stats.full_m4ri_blocks > 0);
            assert!(stats.full_m4ri_table_word_xors > 0);
            assert!(stats.word_xors > 0);
            assert!(stats.word_xors_performed > 0);
        }
    }

    #[test]
    fn faugere_lachartre_split_matches_the_full_width_cascade() {
        let mut rng = StdRng::seed_from_u64(0xf4a6_1ac4_2026_1117);
        for &(n_cols, reducer_count, pair_count) in &[(159usize, 47usize, 83usize), (257, 79, 131)]
        {
            let words = n_cols.div_ceil(64);
            let make_row = |bits: Vec<u64>| {
                let start = bits.iter().position(|&word| word != 0).unwrap_or(0);
                let end = bits
                    .iter()
                    .rposition(|&word| word != 0)
                    .map_or(0, |index| index + 1);
                Row { bits, start, end }
            };
            let mut rows = Vec::new();
            // A triangular reducer block: every tail column lies strictly
            // right of its distinct leading column, exactly as symbolic
            // preprocessing orders Boolean F4 reducers.
            // Interleave pivot and free columns instead of making the
            // reducer block a contiguous prefix.  Symbolic preprocessing
            // only guarantees distinct leads and rightward tails.
            let reducer_leads: Vec<usize> = (0..reducer_count).map(|index| index * 2).collect();
            for &lead in &reducer_leads {
                let mut bits = vec![0u64; words];
                bits[lead / 64] |= 1u64 << (lead % 64);
                for column in lead + 1..n_cols {
                    if rng.gen_ratio(1, 5) {
                        bits[column / 64] |= 1u64 << (column % 64);
                    }
                }
                rows.push(make_row(bits));
            }
            for _ in 0..pair_count {
                let mut bits = vec![0u64; words];
                for column in 0..n_cols {
                    if rng.gen_ratio(1, 3) {
                        bits[column / 64] |= 1u64 << (column % 64);
                    }
                }
                rows.push(make_row(bits));
            }

            let reducer_space: Vec<Vec<u64>> = rows[..reducer_count]
                .iter()
                .map(|row| row.bits.clone())
                .collect();
            let mut reference_stats = F4Stats::default();
            let reference =
                echelon_block_tables(rows.clone(), n_cols, &mut reference_stats, None).unwrap();
            let reference_new_leads: Vec<usize> = reference
                .iter()
                .filter(|(lead, _)| reducer_leads.binary_search(lead).is_err())
                .map(|(lead, _)| *lead)
                .collect();
            let reference_space: Vec<Vec<u64>> =
                reference.into_iter().map(|(_, row)| row.bits).collect();

            let mut candidate_stats = F4Stats::default();
            let candidate =
                echelon_fl(rows, n_cols, reducer_count, &mut candidate_stats, None).unwrap();
            let candidate_new_leads: Vec<usize> = candidate.iter().map(|(lead, _)| *lead).collect();
            let mut candidate_space = reducer_space;
            candidate_space.extend(candidate.into_iter().map(|(_, row)| row.bits));
            assert_eq!(
                candidate_new_leads, reference_new_leads,
                "new pivots for shape reducers={reducer_count} pairs={pair_count} cols={n_cols}"
            );
            assert_eq!(
                canonical_rref(candidate_space, n_cols),
                canonical_rref(reference_space, n_cols),
                "row space for shape reducers={reducer_count} pairs={pair_count} cols={n_cols}"
            );
            assert_eq!(candidate_stats.fl_matrices, 1);
            assert_eq!(candidate_stats.fl_reducer_rows, reducer_count as u64);
            assert_eq!(candidate_stats.fl_pair_rows, pair_count as u64);
            assert_eq!(
                candidate_stats.fl_free_cols,
                (n_cols - reducer_count) as u64
            );
        }
    }

    /// Large enough for the leading block's tables, with blocks of every
    /// width from one to [`TABLE_BLOCK`] and blocks at word boundaries: the
    /// pivots, their ends and the counted XORs are the row-by-row loop's,
    /// and fewer XORs are performed.
    #[test]
    fn leading_block_tables_reproduce_the_row_by_row_elimination() {
        let mut rng = StdRng::seed_from_u64(0x4d34_7269);
        let n_cols: usize = 1500;
        let words = n_cols.div_ceil(64);
        let row = |lead: usize, density: u32, rng: &mut StdRng| {
            let mut bits = vec![0u64; words];
            bits[lead / 64] |= 1 << (lead % 64);
            let mut end = lead / 64 + 1;
            for c in lead + 1..n_cols {
                if rng.gen_ratio(1, density) {
                    bits[c / 64] |= 1 << (c % 64);
                    end = end.max(c / 64 + 1);
                }
            }
            Row {
                bits,
                start: lead / 64,
                end,
            }
        };
        // leading block: distinct leads on most columns, with gaps
        let mut rows: Vec<Row> = Vec::new();
        for c in 0..n_cols - 40 {
            if rng.gen_ratio(9, 10) {
                rows.push(row(c, 8, &mut rng));
            }
        }
        let fixed = rows.len();
        assert!(fixed >= TABLE_ROWS);
        for _ in 0..2000 {
            let lead = rng.gen_range(0..n_cols);
            rows.push(row(lead, 3, &mut rng));
        }
        assert!(rows.len() - fixed >= TABLE_ROWS);
        let (want, want_xors) = echelon_reference(&rows, n_cols);
        let mut st = F4Stats::default();
        let got = echelon_block_tables(rows, n_cols, &mut st, None).unwrap();
        assert_eq!(got.len(), want.len());
        for ((lead, row), (wlead, wbits, wend)) in got.iter().zip(&want) {
            assert_eq!(lead, wlead);
            assert_eq!(&row.bits, wbits);
            assert_eq!(row.end, *wend);
        }
        assert_eq!(st.word_xors, want_xors);
        assert!(st.word_xors_performed < st.word_xors, "{st:?}");
    }

    fn poly(masks: &[u64], n: usize) -> F2BoolPoly {
        F2BoolPoly::from_monos(
            masks
                .iter()
                .map(|&m| F2BoolMono::from_mask(mask_from64(m)))
                .collect(),
            n,
        )
    }

    #[test]
    fn dense_pair_selection_matches_quadratic_update() {
        let mut rng = StdRng::seed_from_u64(0xd3e5_e1ec_7102_6101);
        for n_vars in 3usize..=12 {
            let cap = (1u64 << n_vars) - 1;
            let mut scratch = PairSelectScratch::default();
            for round in 0..256 {
                let count = rng.gen_range(0..96usize);
                let lm: Vec<Mask> = (0..count)
                    .map(|index| {
                        mask_from64(if index > 0 && index % 7 == 0 {
                            // Duplicate LCM groups and leaders are deliberate.
                            rng.gen::<u64>() & cap
                        } else {
                            (rng.gen::<u64>() & cap).max(1)
                        })
                    })
                    .collect();
                let active: Vec<usize> = (0..count).filter(|_| rng.gen_ratio(3, 4)).collect();
                let lh = mask_from64((rng.gen::<u64>() & cap).max(1));
                let mut quadratic_stats = F4Stats::default();
                let mut dense_stats = F4Stats::default();
                let quadratic = select_new_pairs_quadratic(lh, &lm, &active, &mut quadratic_stats);
                let dense = select_new_pairs_dense(
                    n_vars,
                    lh,
                    &lm,
                    &active,
                    &mut scratch,
                    &mut dense_stats,
                );
                assert_eq!(dense, quadratic, "n={n_vars} round={round}");
                assert_eq!(
                    dense_stats.pairs_chain_skipped, quadratic_stats.pairs_chain_skipped,
                    "chain count n={n_vars} round={round}"
                );
                assert_eq!(
                    dense_stats.pairs_product_skipped, quadratic_stats.pairs_product_skipped,
                    "product count n={n_vars} round={round}"
                );
            }
        }
    }

    #[test]
    fn batched_insert_keeps_pairs_and_skip_counts() {
        let empty = || State {
            n_vars: 10,
            polys: Vec::new(),
            lm: Vec::new(),
            active: Vec::new(),
            pairs: Vec::new(),
            pair_select_scratch: PairSelectScratch::default(),
        };
        let (mut serial, mut batched) = (empty(), empty());
        let (mut serial_st, mut batched_st) = (F4Stats::default(), F4Stats::default());
        let mut rng = StdRng::seed_from_u64(0xba7c_2026);
        for _ in 0..30 {
            let mut group = Vec::new();
            for _ in 0..rng.gen_range(1..6) {
                let p = loop {
                    let masks: Vec<u64> = (0..rng.gen_range(1..20))
                        .map(|_| rng.gen_range(0..(1u64 << 10)))
                        .collect();
                    let p = poly(&masks, 10);
                    if !p.is_zero() {
                        break p;
                    }
                };
                group.push(p);
            }
            for p in &group {
                serial.insert(p.clone(), &mut serial_st);
            }
            batched.insert_batch(group, &mut batched_st);
            assert_eq!(batched.polys, serial.polys);
            assert_eq!(batched.lm, serial.lm);
            assert_eq!(batched.active, serial.active);
            assert_eq!(batched.pairs, serial.pairs);
            assert_eq!(
                batched_st.pairs_chain_skipped,
                serial_st.pairs_chain_skipped
            );
            assert_eq!(
                batched_st.pairs_product_skipped,
                serial_st.pairs_product_skipped
            );
        }
    }

    fn brute_force(eqs: &[F2BoolPoly], n: usize) -> Vec<u64> {
        (0u64..1 << n)
            .filter(|v| eqs.iter().all(|e| e.eval(mask_from64(*v)) == 0))
            .collect()
    }

    fn standard_monomials(gb: &[F2BoolPoly], n: usize) -> u64 {
        let lms: Vec<Mask> = gb.iter().filter_map(|p| p.lt()).map(|m| m.mask).collect();
        (0u64..1 << n)
            .map(mask_from64)
            .filter(|&m| !lms.iter().any(|&l| l & !m == 0))
            .count() as u64
    }

    /// The Gröbner-basis certificate, checked directly: every
    /// S-polynomial and every field product reduces to zero.
    fn is_boolean_groebner_basis(gb: &[F2BoolPoly]) -> bool {
        for (i, g) in gb.iter().enumerate() {
            let l = g.lt().unwrap().mask;
            let mut bits = l;
            while bits != 0 {
                let v = bits.trailing_zeros();
                bits &= bits - 1;
                if !reduce(&g.mul_mono(F2BoolMono::from_mask(1 << v)), gb).is_zero() {
                    return false;
                }
            }
            for h in &gb[i + 1..] {
                if !reduce(&spoly(g, h), gb).is_zero() {
                    return false;
                }
            }
        }
        true
    }

    fn random_quadratic(n: usize, m: usize, rng: &mut StdRng) -> Vec<F2BoolPoly> {
        (0..m)
            .map(|_| {
                let mut monos = Vec::new();
                for i in 0..n {
                    for j in 0..i {
                        if rng.gen_bool(0.5) {
                            monos.push((1u64 << i) | (1 << j));
                        }
                    }
                    if rng.gen_bool(0.5) {
                        monos.push(1u64 << i);
                    }
                }
                if rng.gen_bool(0.5) {
                    monos.push(0);
                }
                poly(&monos, n)
            })
            .collect()
    }

    /// **The field pairs are what makes it a boolean basis.**  On
    /// `v_0v_1 + v_0 + v_1`, whose only root is `(0, 0)`, the basis is
    /// `{v_0, v_1}`: `v_0·g = v_0` and `v_1·g = v_1`.  The repository's
    /// Buchberger used to return the generator itself — three standard
    /// monomials for one solution — and was closed under the field
    /// equations after this engine was written (34154ed9); the two now
    /// agree here, and the ledger's §14–§17 rows record which engine
    /// they measured.
    #[test]
    fn the_field_pairs_close_the_toy_ideal() {
        let g = poly(&[3, 1, 2], 2);
        let (gb, st) = groebner_basis_f4(vec![g.clone()], 2, None);
        let mut lts: Vec<Mask> = gb.iter().map(|p| p.lt().unwrap().mask).collect();
        lts.sort();
        assert_eq!(lts, vec![1, 2]);
        assert!(gb.iter().all(|p| p.terms.len() == 1), "{gb:?}");
        assert_eq!(standard_monomials(&gb, 2), 1);
        assert!(st.field_pairs_reduced > 0);
        let buchberger = groebner_basis_f2(vec![g], 2);
        assert_eq!(
            standard_monomials(&buchberger, 2),
            1,
            "Buchberger closes under the field equations too"
        );
    }

    /// **Certified on random systems**: the output is a Gröbner basis of
    /// the boolean ideal (every S-polynomial and field product reduces to
    /// zero), it has exactly as many standard monomials as the system has
    /// solutions, an inconsistent system gives `{1}`, and the extraction
    /// recovers every solution and nothing else.
    #[test]
    fn the_basis_is_a_certified_boolean_groebner_basis() {
        let mut rng = StdRng::seed_from_u64(20260922);
        let (mut consistent, mut inconsistent) = (0, 0);
        for trial in 0..60 {
            let n = 4 + trial % 7;
            let m = n - 2 + trial % 5;
            let eqs = random_quadratic(n, m, &mut rng);
            let (gb, st) = groebner_basis_f4(eqs.clone(), n, None);
            assert!(!st.timed_out && !st.oversize);
            let sols = brute_force(&eqs, n);
            assert!(is_boolean_groebner_basis(&gb), "trial {trial}: not closed");
            assert_eq!(
                standard_monomials(&gb, n),
                sols.len() as u64,
                "trial {trial}"
            );
            if sols.is_empty() {
                inconsistent += 1;
                assert_eq!(gb.len(), 1);
                assert!(is_one(&gb[0]));
            } else {
                consistent += 1;
            }
            let mut tests = 0;
            let mut got = solutions_from_reduced_basis(&gb, n, 24, &mut tests).unwrap();
            got.sort_unstable();
            assert_eq!(
                crate::cryptanalysis::pq_groebner_f2::masks_to_words_checked(&got)
                    .expect("bounded one-word fixture returned unexpected high mask bits"),
                sols,
                "trial {trial}: extraction"
            );
            // Every generator lies in the ideal the basis generates.
            for e in &eqs {
                assert!(
                    reduce(e, &gb).is_zero(),
                    "trial {trial}: a generator escaped"
                );
            }
        }
        assert!(
            consistent > 5 && inconsistent > 5,
            "{consistent} / {inconsistent}"
        );
    }

    /// **Batched Buchberger is Buchberger.**  Wherever the Buchberger
    /// output is certified a boolean basis — everywhere, since it closes
    /// under the field equations — the reduced bases must coincide
    /// polynomial for polynomial: the reduced Gröbner basis of an ideal
    /// is unique.
    #[test]
    fn f4_and_buchberger_agree_where_buchberger_is_certified() {
        use crate::cryptanalysis::ic_boundary::random_binary_instance;
        use crate::cryptanalysis::pq_descent_symbolic::descend;
        let mut rng = StdRng::seed_from_u64(5);
        let mut compared = 0;
        for &(n, np, seed) in &[(7u32, 4u32, 1u64), (9, 5, 2), (11, 6, 3)] {
            let inst = random_binary_instance(n, seed, 1 << 20).unwrap();
            let words: Vec<u64> = (0..np).map(|k| 1u64 << k).collect();
            for _ in 0..4 {
                let x_r = rng.gen::<u64>() & ((1u64 << n) - 1);
                let sys = descend(&inst.gf, inst.b, x_r, &words, 2).unwrap();
                let bb = groebner_basis_f2(sys.equations.clone(), sys.n_vars);
                if !is_boolean_groebner_basis(&bb) {
                    continue;
                }
                let (f4, _) = groebner_basis_f4(sys.equations.clone(), sys.n_vars, None);
                assert_eq!(f4, bb, "n = {n}, n' = {np}: the reduced bases differ");
                compared += 1;
            }
        }
        assert!(compared >= 8, "only {compared} systems were comparable");
    }

    /// The extraction enumerates free variables only.  The standard
    /// monomials include `1` and every free variable, and there are as
    /// many of them as solutions, so `1 + #free ≤ #solutions`.
    #[test]
    fn extraction_enumerates_only_the_free_variables() {
        let mut rng = StdRng::seed_from_u64(99);
        let mut consistent = 0;
        for _ in 0..20 {
            let eqs = random_quadratic(9, 9, &mut rng);
            let (gb, _) = groebner_basis_f4(eqs.clone(), 9, None);
            let sols = brute_force(&eqs, 9).len() as u64;
            let mut tests = 0;
            let got = solutions_from_reduced_basis(&gb, 9, 24, &mut tests).unwrap();
            assert_eq!(got.len() as u64, sols);
            if sols > 0 {
                consistent += 1;
                let free = (0..9)
                    .filter(|&v| {
                        !gb.iter()
                            .any(|g| g.lt().unwrap().mask == mask_from64(1u64 << v))
                    })
                    .count() as u64;
                assert!(free < sols, "{free} free variables for {sols} solutions");
            }
        }
        assert!(consistent > 3);
    }

    #[test]
    fn matrix_profile_row_bins_cover_boundaries() {
        for (rows, expected) in [
            (0usize, 0usize),
            (255, 0),
            (256, 1),
            (511, 1),
            (512, 2),
            (1023, 2),
            (1024, 3),
            (2047, 3),
            (2048, 4),
            (4095, 4),
            (4096, 5),
            (12000, 5),
        ] {
            assert_eq!(matrix_profile_bin(rows), expected);
        }
    }

    #[test]
    fn full_m4ri_minimum_rows_preserves_other_shape_gates() {
        assert!(!full_m4ri_shape_for(4095, 4096, 4096));
        assert!(full_m4ri_shape_for(4096, 4096, 4096));
        assert!(!full_m4ri_shape_for(4096, 255, 4096));
        assert!(!full_m4ri_shape_for(4096, 16_385, 4096));
        assert!(full_m4ri_shape_for(128, 256, 128));
    }

    #[test]
    fn full_m4ri_policy_resolves_selected_and_legacy_controls() {
        assert!(full_m4ri_enabled_for(None));
        assert!(full_m4ri_enabled_for(Some("1")));
        assert!(!full_m4ri_enabled_for(Some("0")));
        assert_eq!(full_m4ri_min_rows_for(None, None), 4096);
        assert_eq!(full_m4ri_min_rows_for(Some("0"), None), 4096);
        assert_eq!(full_m4ri_min_rows_for(Some("1"), None), 128);
        assert_eq!(full_m4ri_min_rows_for(None, Some("2048")), 2048);
        assert_eq!(full_m4ri_min_rows_for(Some("1"), Some("64")), 128);
    }

    /// A budget is honoured and reported, not silently turned into a
    /// wrong basis.
    #[test]
    fn the_budget_is_reported() {
        let mut rng = StdRng::seed_from_u64(3);
        let eqs = random_quadratic(18, 17, &mut rng);
        let (_, st) = groebner_basis_f4(eqs, 18, Some(Duration::from_nanos(1)));
        assert!(st.timed_out);
    }

    fn counted(
        eqs: Vec<F2BoolPoly>,
        n: usize,
        max_words: u64,
        guard: EarlyGuard,
    ) -> (Vec<F2BoolPoly>, F4Stats, tally::Tally) {
        let before = tally::snapshot();
        let (gb, st) = groebner_basis_f4_inner(eqs, n, None, None, max_words, guard, None);
        (gb, st, tally::snapshot().since(before))
    }

    const EAGER: EarlyGuard = EarlyGuard {
        terms: 0,
        slab: 1,
        deadline: None,
    };

    /// The early guard refuses exactly what the late guard refuses: the
    /// smallest cap at which a run is not oversize is the same with the
    /// pre-pass on every step (`EAGER`) as with it off, and at that cap the
    /// guarded run reproduces the basis, the stats and the counted
    /// operations; one word lower both refuse.
    #[test]
    fn the_early_guard_keeps_fitting_steps_identical() {
        const OFF: EarlyGuard = EarlyGuard {
            terms: usize::MAX,
            slab: 256,
            deadline: None,
        };
        for seed in 0..3u64 {
            let mut rng = StdRng::seed_from_u64(100 + seed);
            let eqs = random_quadratic(11, 10, &mut rng);
            let (gb0, st0, _) = counted(eqs.clone(), 11, MAX_MATRIX_WORDS, OFF);
            assert!(!st0.oversize && !st0.timed_out);
            // smallest fitting cap under the late guard alone
            let (mut lo, mut hi) = (0u64, st0.peak_matrix_bytes / 8);
            while lo + 1 < hi {
                let mid = (lo + hi) / 2;
                if counted(eqs.clone(), 11, mid, OFF).1.oversize {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let cap = hi;
            let (gb_l, st_l, t_l) = counted(eqs.clone(), 11, cap, OFF);
            let (gb_e, st_e, t_e) = counted(eqs.clone(), 11, cap, EAGER);
            assert!(!st_l.oversize && !st_e.oversize);
            assert_eq!(gb0, gb_e);
            assert_eq!(gb_l, gb_e);
            assert_eq!(t_l, t_e, "counted operations must not change");
            assert_eq!(st_l.word_xors, st_e.word_xors);
            assert_eq!(st_l.steps, st_e.steps);
            assert_eq!(st_l.reducer_rows, st_e.reducer_rows);
            assert_eq!(st_l.divisor_tests, st_e.divisor_tests);
            if cap > 1 {
                let (_, a, _) = counted(eqs.clone(), 11, cap - 1, OFF);
                let (_, b, _) = counted(eqs, 11, cap - 1, EAGER);
                assert!(a.oversize && b.oversize && !b.timed_out);
            }
        }
    }

    /// An oversized step is refused before any sparse row is stored: the refusal arrives with `oversize` set and the
    /// step's pairs put back, whatever the cap.
    #[test]
    fn an_oversized_step_is_refused_while_building() {
        let mut rng = StdRng::seed_from_u64(7);
        let eqs = random_quadratic(14, 14, &mut rng);
        for cap in [1u64, 8, 64] {
            let (_, st, _) = counted(eqs.clone(), 14, cap, EAGER);
            assert!(st.oversize, "cap {cap}");
            assert!(st.pairs_left > 0);
            // refused before any matrix of the step was recorded
            assert!(st.matrix_rows_max == 0 || st.peak_matrix_bytes / 8 <= cap);
        }
        // the public entry point keeps the 1 GiB guard and is unchanged
        let (_, st) = groebner_basis_f4(eqs, 14, None);
        assert!(!st.oversize);
    }

    /// The deadline is honoured while building rows, not only in the
    /// elimination: a build deadline already past stops the first step
    /// that reaches row building.
    #[test]
    fn the_budget_is_honoured_while_building_rows() {
        let mut rng = StdRng::seed_from_u64(9);
        let eqs = random_quadratic(14, 14, &mut rng);
        let guard = EarlyGuard {
            deadline: Some(Instant::now()),
            ..EAGER
        };
        let (_, st, _) = counted(eqs, 14, MAX_MATRIX_WORDS, guard);
        assert!(st.timed_out && !st.oversize);
        assert_eq!(st.matrix_rows_max, 0, "no matrix of the step was built");
        assert!(st.pairs_left > 0);
    }

    /// Same-shape systems: one template support, each system keeping a
    /// random subset of its terms (the target-dependent bits of a cell).
    fn same_shape_batch(
        n: usize,
        m: usize,
        count: usize,
        keep: f64,
        rng: &mut StdRng,
    ) -> Vec<Vec<F2BoolPoly>> {
        let template = random_quadratic(n, m, rng);
        (0..count)
            .map(|_| {
                template
                    .iter()
                    .map(|p| {
                        let kept: Vec<u64> = p
                            .terms
                            .iter()
                            .map(|t| mask_low64(t.mask))
                            .filter(|_| rng.gen_bool(keep))
                            .collect();
                        poly(&kept, n)
                    })
                    .collect()
            })
            .collect()
    }

    fn assert_same_run(a: &(Vec<F2BoolPoly>, F4Stats), b: &(Vec<F2BoolPoly>, F4Stats), what: &str) {
        assert_eq!(a.0, b.0, "{what}: bases differ");
        let (x, y) = (&a.1, &b.1);
        assert_eq!(
            (
                x.steps,
                x.new_elements,
                x.reducer_rows,
                x.matrix_rows_sum,
                x.matrix_cols_max,
                x.word_xors,
                x.solving_degree,
                x.degree_reached,
                x.pairs_reduced,
                x.field_pairs_reduced,
            ),
            (
                y.steps,
                y.new_elements,
                y.reducer_rows,
                y.matrix_rows_sum,
                y.matrix_cols_max,
                y.word_xors,
                y.solving_degree,
                y.degree_reached,
                y.pairs_reduced,
                y.field_pairs_reduced,
            ),
            "{what}: the runs differ"
        );
    }

    /// **The skeleton changes no basis and no decision.**  Over a batch of
    /// random same-shape systems, with degenerate members (an inconsistent
    /// one, a rank-one one, an all-zero one, a member of another shape
    /// that forces a rebuild, and a different template that misses the
    /// stored steps), the basis and every structural count of the skeleton
    /// path equal the plain path's, the skeleton is replayed, and its
    /// divisor tests are fewer.
    #[test]
    fn skeleton_matches_the_plain_path_on_same_shape_systems() {
        let mut rng = StdRng::seed_from_u64(0x5ce1_2026_1118);
        let n = 10;
        let mut batch = same_shape_batch(n, 10, 24, 0.9, &mut rng);
        // degenerate members, in the middle of the batch
        let mut inconsistent = batch[0].clone();
        inconsistent.push(poly(&[0], n));
        batch.insert(5, inconsistent);
        let rank_one = vec![batch[1][0].clone(); 10];
        batch.insert(9, rank_one);
        batch.insert(12, vec![F2BoolPoly::zero(n); 10]);
        // another template of the same shape: every stored step misses
        let other = same_shape_batch(n, 10, 3, 0.9, &mut rng);
        batch.splice(14..14, other);
        let mut skeleton = F4Skeleton::with_cap(SKELETON_DEFAULT_CAP_BYTES);
        let (mut plain_tests, mut skel_tests) = (0u64, 0u64);
        let mut total = F4Stats::default();
        for (k, eqs) in batch.iter().enumerate() {
            let plain = groebner_basis_f4(eqs.clone(), n, None);
            let skel = groebner_basis_f4_skeleton(eqs.clone(), n, None, &mut skeleton);
            assert_same_run(&plain, &skel, &format!("system {k}"));
            plain_tests += plain.1.divisor_tests;
            skel_tests += skel.1.divisor_tests;
            let s = &skel.1;
            total.skeleton_steps_built += s.skeleton_steps_built;
            total.skeleton_steps_replayed += s.skeleton_steps_replayed;
            total.skeleton_candidate_hits += s.skeleton_candidate_hits;
            total.skeleton_candidate_misses += s.skeleton_candidate_misses;
            total.skeleton_columns_reused += s.skeleton_columns_reused;
            total.skeleton_columns_rebuilt += s.skeleton_columns_rebuilt;
            total.skeleton_shape_rebuilds += s.skeleton_shape_rebuilds;
        }
        // a system of another shape (one more unknown) clears the skeleton
        let wider = same_shape_batch(n + 1, 10, 1, 0.9, &mut rng).remove(0);
        let plain = groebner_basis_f4(wider.clone(), n + 1, None);
        let skel = groebner_basis_f4_skeleton(wider, n + 1, None, &mut skeleton);
        assert_same_run(&plain, &skel, "wider system");
        assert_eq!(skel.1.skeleton_shape_rebuilds, 1);
        assert_eq!(skel.1.skeleton_steps_replayed, 0);
        // and the first system again, after the rebuild, is still exact
        let plain = groebner_basis_f4(batch[0].clone(), n, None);
        let skel = groebner_basis_f4_skeleton(batch[0].clone(), n, None, &mut skeleton);
        assert_same_run(&plain, &skel, "after rebuild");
        eprintln!(
            "skeleton: divisor tests {skel_tests} vs plain {plain_tests}; built {} replayed {} \
             hits {} misses {} columns reused {} rebuilt {} shape rebuilds {}; bytes {}",
            total.skeleton_steps_built,
            total.skeleton_steps_replayed,
            total.skeleton_candidate_hits,
            total.skeleton_candidate_misses,
            total.skeleton_columns_reused,
            total.skeleton_columns_rebuilt,
            total.skeleton_shape_rebuilds,
            skeleton.bytes(),
        );
        assert!(total.skeleton_steps_replayed > 0);
        assert!(total.skeleton_candidate_hits > 0);
        assert!(total.skeleton_candidate_misses + total.skeleton_columns_rebuilt > 0);
        assert!(skel_tests < plain_tests, "{skel_tests} >= {plain_tests}");
    }

    /// At a zero byte cap nothing is stored, and the run is the plain one.
    #[test]
    fn skeleton_at_zero_cap_stores_nothing() {
        let mut rng = StdRng::seed_from_u64(0x5ce1_0);
        let batch = same_shape_batch(8, 8, 4, 0.9, &mut rng);
        let mut skeleton = F4Skeleton::with_cap(0);
        for eqs in &batch {
            let plain = groebner_basis_f4(eqs.clone(), 8, None);
            let skel = groebner_basis_f4_skeleton(eqs.clone(), 8, None, &mut skeleton);
            assert_same_run(&plain, &skel, "cap 0");
            assert_eq!(skel.1.divisor_tests, plain.1.divisor_tests);
            assert_eq!(skel.1.skeleton_steps_replayed, 0);
        }
        assert!(skeleton.is_empty());
        assert_eq!(skeleton.bytes(), 0);
    }
}

#[cfg(test)]
mod interrupted_elimination_accounting {
    use super::*;

    #[test]
    fn fl_interruption_retains_the_completed_xor_prefix() {
        let n_cols = 257usize;
        let reducer_count = 128usize;
        let mut rows = Vec::new();
        // Distinct reducer leads and strictly rightward tails satisfy the
        // actual symbolic-preprocessing contract. Descending substitution
        // alternates three-word and one-word XORs before the stop.
        for lead in 0..reducer_count {
            let mut bits = vec![0u64; n_cols.div_ceil(64)];
            for column in [lead, lead + 1, 256] {
                bits[column / 64] |= 1u64 << (column % 64);
            }
            rows.push(Row {
                bits,
                start: lead / 64,
                end: 5,
            });
        }
        for free in 128..160 {
            let mut bits = vec![0u64; n_cols.div_ceil(64)];
            bits[free / 64] |= 1u64 << (free % 64);
            rows.push(Row {
                bits,
                start: free / 64,
                end: free / 64 + 1,
            });
        }
        let mut checks = 0usize;
        let mut stats = F4Stats::default();
        let result = echelon_fl_with_expiry(rows, n_cols, reducer_count, &mut stats, None, || {
            checks += 1;
            checks > 1
        });
        assert!(result.is_none());
        assert_eq!(checks, 2);
        // The 63 completed substitutions perform 32*3 + 31*1 word XORs.
        assert_eq!(stats.word_xors, 127);
        assert_eq!(stats.word_xors_performed, 127);
    }
}

#[cfg(test)]
mod interrupted_table_accounting {
    use super::*;

    #[test]
    fn full_m4ri_interruption_retains_completed_table_preparation() {
        let n_cols = 257usize;
        let mut rows = Vec::new();
        for lead in 0..2 * FULL_M4RI_BLOCK + 1 {
            let mut bits = vec![0u64; n_cols.div_ceil(64)];
            bits[lead / 64] |= 1u64 << (lead % 64);
            bits[4] |= 1;
            rows.push(Row {
                bits,
                start: lead / 64,
                end: 5,
            });
        }
        let mut checks = 0usize;
        let mut stats = F4Stats::default();
        let result = echelon_full_m4ri_with_expiry(rows, n_cols, &mut stats, || {
            checks += 1;
            checks > 1
        });
        assert!(result.is_none());
        assert_eq!(checks, 2);
        let prepared = ((1u64 << FULL_M4RI_BLOCK) - 1) * 5;
        assert_eq!(stats.word_xors, 0);
        assert_eq!(stats.word_xors_performed, prepared);
        assert_eq!(stats.full_m4ri_table_word_xors, prepared);
        assert_eq!(stats.full_m4ri_blocks, 1);
    }

    #[test]
    fn block_table_expiry_retains_prepared_lookup_words() {
        let n_cols = 512usize;
        let words = n_cols.div_ceil(64);
        let mut rows = Vec::new();
        let mut pivot_of = vec![NONE; n_cols];
        let mut pivots = Vec::new();
        for lead in 0..TABLE_ROWS {
            let mut bits = vec![0u64; words];
            bits[lead / 64] |= 1u64 << (lead % 64);
            bits[7] |= 1u64 << 63;
            let row = Row {
                bits,
                start: lead / 64,
                end: 8,
            };
            pivot_of[lead] = lead as u32;
            pivots.push((lead, row.clone()));
            rows.push(row);
        }
        for index in 0..4 * TABLE_ROWS {
            let mut bits = vec![0u64; words];
            bits[0] |= 1;
            let free = 256 + index % 255;
            bits[free / 64] |= 1u64 << (free % 64);
            rows.push(Row {
                bits,
                start: 0,
                end: free / 64 + 1,
            });
        }
        let expected = BlockTables::build(
            &pivot_of,
            &pivots,
            rows.len() * words * TABLE_INITIAL_BUDGET_MULT,
        )
        .unwrap()
        .built_words;
        assert!(expected > 0);
        let mut stats = F4Stats::default();
        let result = echelon_block_tables(rows, n_cols, &mut stats, Some(Instant::now()));
        assert!(result.is_none());
        assert_eq!(stats.word_xors, 0);
        assert_eq!(stats.word_xors_performed, expected);
    }
}
