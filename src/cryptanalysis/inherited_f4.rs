//! # Inherited matrix-F4: the reduced Macaulay basis specialised down the splitting tree.
//!
//! The Koblitz decomposition oracle
//! ([`super::koblitz_groebner::solve_boolean_system`]) is a DPLL loop
//! around a Macaulay reduction: at every node it takes the current system,
//! builds the degree-`D` Macaulay matrix from scratch, reduces it, reads
//! off the refutation or the forced variables, and recurses on
//! `v := 0` and `v := 1`.  Every child rebuilds and re-reduces a matrix
//! that is a *specialisation* of the one its parent has just reduced.
//!
//! F5's organising idea is to never redo a reduction the algorithm can
//! predict.  Here the prediction is exact.  Substitution `φ_c : v ↦ c` is a
//! ring homomorphism of the Boolean ring, and the Macaulay rows of the
//! specialised system are the images of the parent's rows:
//!
//! ```text
//!     V(S|_{v=c})  =  φ_c( V(S) )  =  span{ φ_c(ρ) : ρ a reduced row of the parent }
//! ```
//!
//! (up to multipliers that cannot reach the linear tail, and up to a
//! generator whose degree *drops* under `φ_c` gaining multipliers — see
//! [`ReducedBasis::specialise`] for both).  So the child's row space is
//! spanned by the `rank` reduced rows of its parent, specialised — and most
//! of them are already in echelon form:
//!
//! - under `v := 0` every monomial containing `v` is deleted; a row whose
//!   leading monomial does not contain `v` keeps it;
//! - under `v := 1` every monomial `m ∋ v` folds into `m ∖ v`, which is
//!   *smaller* in any degree-compatible order, so again a leading monomial
//!   without `v` survives and cannot be cancelled.
//!
//! Only the rows whose **pivot contains `v`** lose their place and are
//! re-reduced against the rest — about `D/n` of them.  No Macaulay matrix
//! is built below the root, and no reduction is repeated.  Because the
//! row space is the same one the from-scratch step would compute, the
//! linear tail — hence every refutation, forced variable, split and
//! verdict — is identical; only the work per node changes.
//!
//! ## Accounting
//!
//! The stage's unit is 64-bit word XORs in the elimination.  Everything
//! this module does to a packed row is charged in that unit: the root
//! reduction through the shared kernel, every XOR of the re-reduction, and
//! the specialisation itself at one word operation per word read and per
//! word written — a charge the from-scratch path never pays, since its
//! matrix build is not counted.  Rows are stored from their first nonzero
//! word, so a rewrite reads the words from the row's leading word to the
//! end of its layout and writes those from its image's first word to the
//! end of the new one; an XOR has always started at the pivot's leading
//! word.  Wall time is reported beside it.
//!
//! ## What decides the cost: which variable the solver splits on
//!
//! Re-reducing a displaced row costs one XOR per pivot it crosses, so the
//! method's cost is the number of displaced rows times the density of the
//! basis.  Both depend on the **split variable**.  A reduced row's pivot
//! is its largest monomial under DegRevLex, so pivots are biased toward
//! the large variables; splitting on the lowest-indexed free variable (the
//! historical `LowestFree` rule) picks exactly those, and deep in the tree
//! half the basis is displaced per level.  Splitting on the smallest free
//! variable ([`super::koblitz_groebner::SplitRule::HighestFree`]) displaces
//! the fewest rows: the engine's work halves on every quadratic rung and
//! its loss on the chained cubic `m = 3` systems — whose 40%-rank-deficient
//! matrices make the reduced rows dense — turns into a `1.9×` win on the
//! deep `K_0/2^15` cell (`RESEARCH_INHERITED_F4.md` §3.5).  Restoring
//! reduced form after each level, cost-triggered fallbacks and
//! drift-triggered rebuilds were all tried and rejected; each cost more
//! than it saved.
//!
//! ## What this does not change
//!
//! Nothing about the algebra: same degree, same row space, same splitting
//! rule, same node budget.  Nothing about the decomposition oracle's cost
//! at `n = 131`, which is bounded by how many candidate tuples must be
//! ruled out and not by how one node's matrix is reduced.

use crate::cryptanalysis::bitop_tally;
use crate::cryptanalysis::fx_hash::{FxMap as HashMap, MaskMap};
use crate::cryptanalysis::koblitz_groebner::{
    all_variable_mask, batched_rewrite, build_inherited_macaulay,
    build_inherited_macaulay_support_local, echelon_f2_counted, macaulay_columns,
    macaulay_rows_monos_with_mask, monomials_up_to_mask, next_kernels, pack_rows, rref_f2_counted,
    rref_uses_four_russians,
};
use crate::cryptanalysis::pq_groebner_f2::{
    bit, cmp_mono, mask_fold64, sort_masks_descending, F2BoolMono, F2BoolPoly, Mask,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

mod prefix;
pub use prefix::{
    clear_prefix_cache, prefix_hint, prefix_root_counts, prefix_root_enabled, set_prefix_hint,
    set_prefix_root,
};

static COMPACT_REFUTED_BASIS: AtomicBool = AtomicBool::new(false);

/// Keep only the decisive constant row after a basis proves `1`.
pub fn set_compact_refuted_basis(enabled: bool) {
    COMPACT_REFUTED_BASIS.store(enabled, Ordering::Relaxed);
}

thread_local! {
    /// Macaulay rows generated on this thread (root builds and completion
    /// rows) and the packed words they were written into: the row
    /// generation that the stage unit `word_xors` does not charge.
    static ROWS_GENERATED: std::cell::Cell<(u64, u64)> = const { std::cell::Cell::new((0, 0)) };
}

fn note_rows_generated(rows: u64, words: u64) {
    ROWS_GENERATED.with(|c| {
        let (r, w) = c.get();
        c.set((r.wrapping_add(rows), w.wrapping_add(words)));
    });
}

/// Macaulay rows generated by inherited-F4 bases on the calling thread, and
/// the words they were packed into: every row of a from-scratch build (the
/// root of a tree, or a node rebuilt after linear elimination) and every
/// completion row.  A deterministic counter beside the stage unit, which
/// charges elimination and specialisation but not row generation.
pub fn f4_rows_generated_thread() -> (u64, u64) {
    ROWS_GENERATED.with(|c| c.get())
}

/// Credit rows generated on another thread to this one (a parallel solve
/// hands its workers' counts back to the thread that asked).
pub(crate) fn credit_rows_generated(rows: u64, words: u64) {
    note_rows_generated(rows, words);
}

/// Shape of the degree-`degree` Macaulay matrix an inherited-F4 root builds
/// for `system`: `(rows, columns, rank)`, with multipliers over the
/// occurring variables, or, with `support_local`, over each generator's own
/// support (the production policy).  `None` past the F4 size caps.  A
/// diagnostic for the per-degree cost model (lit2026 W5); nothing is charged.
pub fn macaulay_shape(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    support_local: bool,
) -> Option<(usize, usize, usize)> {
    let system: Vec<F2BoolPoly> = system.iter().filter(|p| !p.is_zero()).cloned().collect();
    let quadratic = system.iter().all(|p| poly_degree(p) <= 2);
    let occurring = system
        .iter()
        .flat_map(|p| p.terms.iter())
        .fold(0 as Mask, |acc, t| acc | t.mask)
        & all_variable_mask(n_vars);
    let (columns, mut matrix) = if support_local {
        build_inherited_macaulay_support_local(&system, n_vars, degree, occurring, quadratic)?
    } else {
        build_inherited_macaulay(&system, n_vars, degree, occurring, quadratic)?
    };
    let rows = matrix.len();
    let mut ignored = 0u64;
    let rank = if matrix.is_empty() {
        0
    } else {
        echelon_f2_counted(&mut matrix, columns.len(), &mut ignored)
    };
    Some((rows, columns.len(), rank))
}

thread_local! {
    /// Word operations charged to root eliminations on this thread.
    static ROOT_ELIMINATION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn note_root_elimination(word_ops: u64) {
    ROOT_ELIMINATION.with(|c| c.set(c.get().wrapping_add(word_ops)));
}

/// The part of the stage unit charged to root eliminations (from-scratch
/// builds, sequential or lane-batched) on the calling thread.  The rest of
/// `word_xors` is the splitting tree's, so two arms whose roots differ
/// only in how they were reduced can be compared tree for tree.
pub fn f4_root_elimination_thread() -> u64 {
    ROOT_ELIMINATION.with(|c| c.get())
}

/// Credit root eliminations done on another thread to this one.
pub(crate) fn credit_root_elimination(word_ops: u64) {
    note_root_elimination(word_ops);
}

// ── Root shape replay (`KIC_F4_ROOT_REPLAY=1`, off by default) ─────────
//
// F4Remake's idea applied to the roots: the rows of a root that are
// dependent on earlier rows (in build order) are recorded on the first
// root of a given shape (column layout, row count, degree, multiplier
// policy) and dropped from later roots of the same shape before the
// elimination.  On another target the dropped rows may be independent, so
// a replayed root spans a *subspace* of the from-scratch row space: every
// row it yields is still in the ideal, so every refutation and forced
// variable it reads is sound, and the splitting tree still decides every
// target exactly (as with `support_local`).  It may read a weaker tail and
// split more; that cost, and the recording's (an incremental basis in
// build order, charged in the stage unit), is in the counts.

fn root_replay() -> bool {
    static ROOT_REPLAY: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ROOT_REPLAY.get_or_init(|| std::env::var("KIC_F4_ROOT_REPLAY").as_deref() == Ok("1"))
}

thread_local! {
    /// Recorded dependent rows by root shape.
    static ROOT_SHAPES: std::cell::RefCell<std::collections::HashMap<u64, std::rc::Rc<Vec<u32>>>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Roots recorded, roots replayed and rows dropped by root shape replay,
/// over all threads (a diagnostic; all zero unless `KIC_F4_ROOT_REPLAY=1`).
static ROOT_REPLAY_COUNTS: [std::sync::atomic::AtomicU64; 3] = [
    std::sync::atomic::AtomicU64::new(0),
    std::sync::atomic::AtomicU64::new(0),
    std::sync::atomic::AtomicU64::new(0),
];

/// `(roots recorded, roots replayed, rows dropped)` since the start.
pub fn root_replay_counts() -> (u64, u64, u64) {
    use std::sync::atomic::Ordering::Relaxed;
    (
        ROOT_REPLAY_COUNTS[0].load(Relaxed),
        ROOT_REPLAY_COUNTS[1].load(Relaxed),
        ROOT_REPLAY_COUNTS[2].load(Relaxed),
    )
}

fn root_shape_key(columns: &[Mask], rows: usize, degree: u32, support_local: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    columns.hash(&mut h);
    rows.hash(&mut h);
    degree.hash(&mut h);
    support_local.hash(&mut h);
    h.finish()
}

/// The rows dependent on earlier rows, in build order, by an incremental
/// basis keyed by leading bit; one word operation per word XORed.
fn dependent_rows(matrix: &[Vec<u64>], word_ops: &mut u64) -> Vec<u32> {
    let mut basis: std::collections::HashMap<usize, Vec<u64>> = std::collections::HashMap::new();
    let mut dependent = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut v = row.clone();
        loop {
            let Some(w) = v.iter().position(|&x| x != 0) else {
                dependent.push(i as u32);
                break;
            };
            let lead = w * 64 + v[w].trailing_zeros() as usize;
            match basis.get(&lead) {
                Some(b) => {
                    for k in w..v.len() {
                        v[k] ^= b[k];
                    }
                    *word_ops += (v.len() - w) as u64;
                }
                None => {
                    basis.insert(lead, v);
                    break;
                }
            }
        }
    }
    dependent
}

/// Apply root shape replay to a freshly built root: record its dependent
/// rows, or drop the recorded ones.
fn replay_root(
    columns: &[Mask],
    matrix: &mut Vec<Vec<u64>>,
    degree: u32,
    support_local: bool,
    word_ops: &mut u64,
) {
    let key = root_shape_key(columns, matrix.len(), degree, support_local);
    let known = ROOT_SHAPES.with(|m| m.borrow().get(&key).cloned());
    match known {
        Some(drop) => {
            let mut keep = vec![true; matrix.len()];
            for &i in drop.iter() {
                keep[i as usize] = false;
            }
            let mut i = 0;
            matrix.retain(|_| {
                let k = keep[i];
                i += 1;
                k
            });
            ROOT_REPLAY_COUNTS[1].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            ROOT_REPLAY_COUNTS[2]
                .fetch_add(drop.len() as u64, std::sync::atomic::Ordering::Relaxed);
        }
        None => {
            let dep = dependent_rows(matrix, word_ops);
            ROOT_SHAPES.with(|m| m.borrow_mut().insert(key, std::rc::Rc::new(dep)));
            ROOT_REPLAY_COUNTS[0].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

/// Whether a root is reduced fully (`KIC_F4_INHERIT_ROOT=rref`) rather
/// than to echelon form; read once.
fn root_rref() -> bool {
    static ROOT_POLICY: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ROOT_POLICY.get_or_init(|| std::env::var("KIC_F4_INHERIT_ROOT").as_deref() == Ok("rref"))
}

/// A root's Macaulay matrix before elimination, with the system it
/// belongs to (zeros dropped) and the generator degrees.
struct RootMatrix {
    system: Rc<Vec<F2BoolPoly>>,
    generator_degrees: Rc<Vec<u32>>,
    columns: Vec<Mask>,
    matrix: Vec<Vec<u64>>,
}

impl RootMatrix {
    /// Generate the rows exactly as a from-scratch root does; `None` past
    /// the F4 size caps.
    fn build(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
    ) -> Option<Self> {
        // `bitop/v1`: the copy, the degree scan and the support fold, a
        // word operation a term each (the build itself charges its own)
        bitop_tally::word_ops(3 * system.iter().map(|p| 1 + p.terms.len() as u64).sum::<u64>());
        let system: Rc<Vec<F2BoolPoly>> =
            Rc::new(system.iter().filter(|p| !p.is_zero()).cloned().collect());
        let generator_degrees: Rc<Vec<u32>> = Rc::new(system.iter().map(poly_degree).collect());
        let quadratic_generators = generator_degrees.iter().all(|&degree| degree <= 2);
        let occurring = system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0 as Mask, |acc, t| acc | t.mask)
            & all_variable_mask(n_vars);
        let (columns, matrix) = if support_local {
            build_inherited_macaulay_support_local(
                system.as_slice(),
                n_vars,
                degree,
                occurring,
                quadratic_generators,
            )?
        } else {
            build_inherited_macaulay(
                system.as_slice(),
                n_vars,
                degree,
                occurring,
                quadratic_generators,
            )?
        };
        note_rows_generated(
            matrix.len() as u64,
            (matrix.len() * columns.len().div_ceil(64)) as u64,
        );
        Some(Self {
            system,
            generator_degrees,
            columns,
            matrix,
        })
    }
}

/// What one lane-batched root elimination did
/// ([`ReducedBasis::from_systems_lanes`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LaneStats {
    /// Instances in the batch.
    pub lanes: u32,
    /// Bits per lane word (8, 16, 32 or 64: the narrowest that holds the
    /// active lanes); `0` if nothing was eliminated.
    pub lane_bits: u32,
    /// Rows and columns of the bit-sliced matrix (the longest lane's row
    /// count; the union of the lanes' columns).
    pub rows: u64,
    pub columns: u64,
    /// Columns at which some lane found a pivot.
    pub pivot_columns: u64,
    /// Pivot rows used: one per column when the lanes share their pivot
    /// pattern, more where they diverge.
    pub pivot_passes: u64,
    /// Columns pivoted in every active lane by one row (fully shared).
    pub shared_columns: u64,
    /// Lane-word XORs (each moves one column of every lane in the mask).
    pub lane_word_ops: u64,
    /// `lane_word_ops` in 64-bit word equivalents.
    pub word_equivalent: u64,
    /// The batch exceeded the memory cap and was not reduced.
    pub oversize: bool,
}

/// The bit-sliced Gauss–Jordan kernel of [`ReducedBasis::from_systems_lanes`].
mod lanes {
    use super::{LaneStats, RootMatrix};
    use crate::cryptanalysis::pq_groebner_f2::Mask;

    /// A lane word: one bit per instance.
    pub(super) trait Lane:
        Copy
        + Default
        + Eq
        + std::ops::BitAnd<Output = Self>
        + std::ops::BitOr<Output = Self>
        + std::ops::BitXor<Output = Self>
        + std::ops::BitXorAssign
        + std::ops::BitOrAssign
        + std::ops::Not<Output = Self>
    {
        const ZERO: Self;
        fn bit(lane: usize) -> Self;
        fn has(self, lane: usize) -> bool;
    }

    macro_rules! lane {
        ($t:ty) => {
            impl Lane for $t {
                const ZERO: Self = 0;
                #[inline(always)]
                fn bit(lane: usize) -> Self {
                    1 << lane
                }
                #[inline(always)]
                fn has(self, lane: usize) -> bool {
                    (self >> lane) & 1 == 1
                }
            }
        };
    }
    lane!(u8);
    lane!(u16);
    lane!(u32);
    lane!(u64);

    /// Memory cap on the bit-sliced matrix, in bytes.
    const MAX_BYTES: usize = 1 << 30;

    /// `dst[j] ^= src[j] & mask` over the common length.
    #[inline(always)]
    fn masked_xor<L: Lane>(dst: &mut [L], src: &[L], mask: L) {
        for (d, &s) in dst.iter_mut().zip(src) {
            *d ^= s & mask;
        }
    }

    /// Reduce the active lanes' matrices together; per active lane, its
    /// reduced rows in its own column layout, by ascending pivot column.
    pub(super) fn reduce<L: Lane>(
        roots: &[Option<RootMatrix>],
        active: &[usize],
        stats: &mut LaneStats,
    ) -> Option<Vec<Vec<Vec<u64>>>> {
        // Union layout: every lane's columns are in descending monomial
        // order, so their merge is too; a lane's leading column stays its
        // leading column.
        let mut union: Vec<Mask> = active
            .iter()
            .flat_map(|&l| roots[l].as_ref().expect("active").columns.iter().copied())
            .collect();
        crate::cryptanalysis::pq_groebner_f2::sort_masks_descending(&mut union);
        union.dedup();
        let position: crate::cryptanalysis::fx_hash::MaskMap<usize> =
            union.iter().enumerate().map(|(i, &m)| (m, i)).collect();
        let ncols = union.len();
        let nrows = active
            .iter()
            .map(|&l| roots[l].as_ref().expect("active").matrix.len())
            .max()
            .unwrap_or(0);
        stats.rows = nrows as u64;
        stats.columns = ncols as u64;
        if nrows
            .saturating_mul(ncols)
            .saturating_mul(std::mem::size_of::<L>())
            > MAX_BYTES
        {
            stats.oversize = true;
            return None;
        }
        let mut m: Vec<L> = vec![L::ZERO; nrows * ncols];
        let mut maps: Vec<Vec<usize>> = Vec::with_capacity(active.len());
        for (i, &l) in active.iter().enumerate() {
            let root = roots[l].as_ref().expect("active");
            let map: Vec<usize> = root.columns.iter().map(|c| position[c]).collect();
            let bit = L::bit(i);
            for (r, row) in root.matrix.iter().enumerate() {
                for (w, &word) in row.iter().enumerate() {
                    let mut bits = word;
                    while bits != 0 {
                        let b = bits.trailing_zeros() as usize;
                        bits &= bits - 1;
                        m[r * ncols + map[w * 64 + b]] |= bit;
                    }
                }
            }
            maps.push(map);
        }
        let all = (0..active.len()).fold(L::ZERO, |acc, i| acc | L::bit(i));
        // `used[r]`: the lanes in which row `r` already pivots.
        let mut used: Vec<L> = vec![L::ZERO; nrows];
        let mut pivots: Vec<(usize, usize, L)> = Vec::new();
        let mut ops = 0u64;
        for c in 0..ncols {
            let mut need = L::ZERO;
            for r in 0..nrows {
                need |= m[r * ncols + c] & !used[r];
            }
            if need == L::ZERO {
                continue;
            }
            stats.pivot_columns += 1;
            let mut first = true;
            while need != L::ZERO {
                let r = (0..nrows)
                    .find(|&r| m[r * ncols + c] & !used[r] & need != L::ZERO)
                    .expect("a lane that needs a pivot has a candidate");
                let p = m[r * ncols + c] & !used[r] & need;
                if first && p == all {
                    stats.shared_columns += 1;
                }
                first = false;
                stats.pivot_passes += 1;
                used[r] |= p;
                need = need & !p;
                pivots.push((r, c, p));
                // Clear column `c` from every other row in the lanes `p`
                // (above and below: reduced form).  In those lanes the
                // pivot row is zero before `c`.
                let (pivot_tail, len) = {
                    let start = r * ncols + c;
                    (m[start..start + ncols - c].to_vec(), ncols - c)
                };
                for r2 in 0..nrows {
                    if r2 == r {
                        continue;
                    }
                    let mask = m[r2 * ncols + c] & p;
                    if mask != L::ZERO {
                        let start = r2 * ncols + c;
                        masked_xor(&mut m[start..start + len], &pivot_tail, mask);
                        ops += len as u64;
                    }
                }
            }
        }
        stats.lane_word_ops = ops;
        // Read each lane's rows back into its own layout.
        let mut out = Vec::with_capacity(active.len());
        for (i, &l) in active.iter().enumerate() {
            let root = roots[l].as_ref().expect("active");
            let words = root.columns.len().div_ceil(64).max(1);
            let mut back = vec![usize::MAX; ncols];
            for (own, &u) in maps[i].iter().enumerate() {
                back[u] = own;
            }
            let mut rows: Vec<(usize, Vec<u64>)> = Vec::new();
            for &(r, c, p) in &pivots {
                if !p.has(i) {
                    continue;
                }
                let mut row = vec![0u64; words];
                for (j, &x) in m[r * ncols + c..(r + 1) * ncols].iter().enumerate() {
                    if x.has(i) {
                        let own = back[c + j];
                        debug_assert!(own != usize::MAX, "a lane's rows stay in its layout");
                        row[own / 64] |= 1u64 << (own % 64);
                    }
                }
                rows.push((back[c], row));
            }
            rows.sort_unstable_by_key(|&(c, _)| c);
            out.push(rows.into_iter().map(|(_, row)| row).collect());
        }
        Some(out)
    }
}

/// Degree of a Boolean polynomial (`0` for a constant or zero).
fn poly_degree(p: &F2BoolPoly) -> u32 {
    p.terms
        .iter()
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(0)
}

/// Leading (lowest-index) set column of a packed row.
fn leading_column(row: &[u64]) -> Option<usize> {
    row.iter()
        .enumerate()
        .find(|(_, &w)| w != 0)
        .map(|(i, &w)| i * 64 + w.trailing_zeros() as usize)
}

/// The version of the word-operation accounting below, recorded in every new
/// ecbench session (`accounting_versions`) so that an audit names a
/// cross-version replay instead of reporting bare count differences.
///
/// - **1** (never recorded; what a session without the field was made
///   under, up to `#2043`, 2026-10-17): specialisation was eager through
///   `pext`/`pdep` on a BMI2 x86-64 host and lazy everywhere else, and the
///   two charge different counts by construction.
/// - **2** (since `#2043`): specialisation is eager on every host (`pext` /
///   `pdep`, or shift runs without BMI2), and every eager path charges
///   identical counts
///   (`rewrite_paths_charge_identical_work_and_build_identical_rows`).
///
/// Bump this whenever a change alters any [`InheritCost`] field for any
/// input; `inherited_f4_accounting_version_pins_the_charge` fails then, on
/// purpose, so the bump cannot be forgotten.  The lazy rewrite
/// (`KIC_F4_EAGER=0`, `KIC_F4_REWRITE=scatter`) stays reachable only as a
/// same-binary control and charges a different count; `ecbench` refuses to
/// run with those overrides set.
pub const INHERITED_F4_ACCOUNTING_VERSION: u32 = 2;

/// Work counters for one basis operation, all in 64-bit word operations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InheritCost {
    /// XORs in the root reduction, in re-reducing displaced rows, and in
    /// restoring reduced form ([`ReducedBasis::reduce_fully`]).
    pub reduce_word_ops: u64,
    /// The part of `reduce_word_ops` spent restoring reduced form.
    pub rref_word_ops: u64,
    /// Word reads and writes performed by specialisation (deleting or
    /// folding the columns of the assigned variable).
    pub specialise_word_ops: u64,
    /// Rows whose pivot contained the assigned variable and were re-reduced.
    pub displaced_rows: u64,
    /// Completion rows added because a generator's degree dropped.
    pub completion_rows: u64,
    /// Products of degree falls inserted by [`ReducedBasis::close`].
    pub closure_products: u64,
}

impl InheritCost {
    /// Everything charged, in word operations.
    pub fn word_ops(&self) -> u64 {
        self.reduce_word_ops + self.specialise_word_ops
    }
    fn add(&mut self, other: InheritCost) {
        self.reduce_word_ops += other.reduce_word_ops;
        self.rref_word_ops += other.rref_word_ops;
        self.specialise_word_ops += other.specialise_word_ops;
        self.displaced_rows += other.displaced_rows;
        self.completion_rows += other.completion_rows;
        self.closure_products += other.closure_products;
    }
}

/// What a [`ClosureSink`] asks of the closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloseFlow {
    Continue,
    Stop,
}

/// Where [`ReducedBasis::close_with`] offers its resumable states: between
/// rounds, and at every word boundary of a round's elimination.  A sink that
/// answers [`CloseFlow::Stop`] ends the closure at that state, which it has
/// saved if it wants it back.
pub trait ClosureSink {
    /// A round has been absorbed and the closure has more to do; `done`
    /// rounds are finished.  The basis is readable by
    /// [`ReducedBasis::closure_view`].
    fn after_round(&mut self, basis: &ReducedBasis, cost: &InheritCost, done: u32) -> CloseFlow;
    /// A word boundary of the elimination inside round `done + 1`.
    fn in_round(&mut self, view: &RoundView<'_>, cost: &InheritCost, done: u32) -> CloseFlow;
}

/// The sink that never stops and keeps nothing: plain `close`.
struct NoSink;
impl ClosureSink for NoSink {
    fn after_round(&mut self, _: &ReducedBasis, _: &InheritCost, _: u32) -> CloseFlow {
        CloseFlow::Continue
    }
    fn in_round(&mut self, _: &RoundView<'_>, _: &InheritCost, _: u32) -> CloseFlow {
        CloseFlow::Continue
    }
}

/// One closure round caught in the middle of its elimination: the matrix of
/// the basis rows and the round's products, partly reduced.  Together with
/// the layout, the pivot bits the basis had, and the elimination's position
/// it is everything the round still needs; the rest of the basis is rebuilt
/// from the finished matrix.
#[derive(Debug)]
pub struct RoundView<'a> {
    pub degree: u32,
    pub n_vars: usize,
    pub support_local: bool,
    pub min_fall_degree: u32,
    pub closure_rounds: u32,
    pub columns: &'a [Mask],
    /// Whether each column led a row of the basis before this round.
    pub was_pivot: &'a [bool],
    pub matrix: &'a [Vec<u64>],
    /// Pivot rows found so far, and the packed word the elimination is at.
    pub pivot_row: usize,
    pub word: usize,
}

/// A round to continue: the owned form of a [`RoundView`].
#[derive(Debug)]
pub struct RoundJob {
    matrix: Vec<Vec<u64>>,
    was_pivot: Vec<bool>,
    /// Where the elimination continues; `None` for a round not yet started.
    resume: Option<(usize, usize)>,
}

/// How [`ReducedBasis::close_with`] ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloseEnd {
    /// Rounds finished.
    pub rounds: u32,
    /// The sink stopped the closure (mid-round, or between rounds).
    pub stopped: bool,
}

/// The state of a closure between two rounds, as a checkpoint holds it:
/// everything [`ReducedBasis::from_closure_state`] needs besides the system
/// itself, which the caller rebuilds from its inputs.  Rows are in the
/// current layout, `words` words each, in the order `pending` indexes.
#[derive(Debug)]
pub struct ClosureView<'a> {
    pub degree: u32,
    pub n_vars: usize,
    pub support_local: bool,
    pub min_fall_degree: u32,
    pub closure_rounds: u32,
    pub words: usize,
    pub columns: &'a [Mask],
    pub pending: &'a [u32],
    rows: &'a [LazyRow],
}

impl ClosureView<'_> {
    pub fn n_rows(&self) -> usize {
        self.rows.len()
    }
    /// Row `r`, all `words` words of it.
    pub fn row(&self, r: usize) -> &[u64] {
        &self.rows[r].data
    }
}

/// A column a layout step deletes (`v := 0` on a monomial containing `v`).
const DELETED: u32 = u32::MAX;

/// One change of column layout: how the columns of one epoch map to the
/// columns of the next.  [`DELETED`] marks a deleted column (`v := 0`);
/// two columns may map to the same target (`v := 1` folds `m ∋ v` onto
/// `m ∖ v`), and a materialisation XORs them together.
#[derive(Debug)]
struct LayoutStep {
    map: Rc<Vec<u32>>,
    /// Composed maps from earlier epochs to the epoch this step maps *to*,
    /// built on demand.  Shared with every basis below this step, so a
    /// child composes one step on top of what its parent already built.
    to_here: RefCell<HashMap<u32, Rc<Vec<u32>>>>,
    /// The word-wise rewrite plans of those composed maps (and of this
    /// step's own map, under key `to − 1`), built on demand from the map.
    #[allow(dead_code)] // rewrite-plan cache reserved for a later path
    plans: RefCell<HashMap<u32, Rc<RewritePlan>>>,
    /// Under the batched rewrite ([`batched_rewrite`]): the eager kernel's
    /// flat plans of the composed maps from earlier epochs to the epoch
    /// this step maps *to* (this step's own plan under key `to − 1`),
    /// built on demand and shared like `to_here`.
    step_plans: RefCell<HashMap<u32, Rc<StepPlan>>>,
    /// The plan of this step's own map (`from = to − 1`), kept apart from
    /// `step_plans` so the commonest lookup is a cell read, not a hash
    /// probe, and a specialisation allocates no table to leave it.
    own_plan: std::cell::OnceCell<Rc<StepPlan>>,
}

/// One `pext`/`pdep` move of a rewrite: the source bits `src_mask` of one
/// source word go, in order, to the bits `dst_mask` of word `dst_word`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Segment {
    src_mask: u64,
    dst_mask: u64,
    dst_word: u32,
}

/// A column map as a list of segments per source word, so a row is
/// rewritten a word at a time instead of a bit at a time.
///
/// Every layout step is a merge of monotone pieces (the kept columns keep
/// their order, the folded ones keep theirs), so under a composed map the
/// columns of one source word fall into a few runs on which the map is
/// increasing and stays inside one destination word.  Each such run is a
/// [`Segment`]: `pext` gathers its source bits, `pdep` spreads them onto
/// its destination bits, and the XOR into the destination word performs
/// the folds, exactly as the bit scatter did.  Two sources folding onto one
/// destination are never in one segment (the destination would not be
/// strictly increasing), so they meet in the XOR as before.
///
/// A word's segments are built the first time a rewrite reads that word:
/// rows are trimmed to their leading word and few rows of a given epoch
/// are rewritten at a node, so building every word of the layout up front
/// cost as much as the rewrites it served.
#[derive(Debug)]
struct RewritePlan {
    map: Rc<Vec<u32>>,
    words: Vec<std::cell::OnceCell<Box<[Segment]>>>,
}

impl RewritePlan {
    /// The plan of `map`, with nothing built yet.
    #[allow(dead_code)]
    fn new(map: Rc<Vec<u32>>) -> Self {
        let n_src_words = map.len().div_ceil(64);
        Self {
            map,
            words: (0..n_src_words)
                .map(|_| std::cell::OnceCell::new())
                .collect(),
        }
    }

    /// The segments of source word `w` of `map`.
    fn segments_of(map: &[u32], w: usize) -> Box<[Segment]> {
        let mut segments = Vec::new();
        Self::segments_into(map, w, &mut segments);
        segments.into_boxed_slice()
    }

    /// [`RewritePlan::segments_of`], appended to `segments`.
    fn segments_into(map: &[u32], w: usize, segments: &mut Vec<Segment>) {
        let base = w * 64;
        let end = (base + 64).min(map.len());
        // `bitop/v1`: one lookup and test a column of the source word
        bitop_tally::word_ops(end.saturating_sub(base) as u64);
        let mut current: Option<(Segment, u32)> = None;
        for c in base..end {
            let d = map[c];
            if d == DELETED {
                continue;
            }
            let (dw, db) = (d / 64, d % 64);
            let extend = match &current {
                Some((seg, last_bit)) => seg.dst_word == dw && db > *last_bit,
                None => false,
            };
            if !extend {
                if let Some((seg, _)) = current.take() {
                    segments.push(seg);
                }
                current = Some((
                    Segment {
                        src_mask: 0,
                        dst_mask: 0,
                        dst_word: dw,
                    },
                    0,
                ));
            }
            let (seg, last_bit) = current.as_mut().expect("just set");
            seg.src_mask |= 1u64 << (c - base);
            seg.dst_mask |= 1u64 << db;
            *last_bit = db;
        }
        if let Some((seg, _)) = current {
            segments.push(seg);
        }
    }

    /// [`RewritePlan::segments_into`], the same segments in the same
    /// order, with the same tally, built with fewer unpredictable branches:
    /// the kept columns of the word are found in one branch-free pass and
    /// then visited by their set bits, so a deleted column costs no branch.
    /// The batched rewrite builds its plans with it; the eager path keeps
    /// the original.
    fn segments_into_kept(map: &[u32], w: usize, segments: &mut Vec<Segment>) {
        let base = w * 64;
        let end = (base + 64).min(map.len());
        // `bitop/v1`: one lookup and test a column of the source word
        bitop_tally::word_ops(end.saturating_sub(base) as u64);
        if end <= base {
            return;
        }
        let chunk = &map[base..end];
        let mut kept = 0u64;
        for (i, &d) in chunk.iter().enumerate() {
            kept |= u64::from(d != DELETED) << i;
        }
        // `u32::MAX` is no destination word: the first kept column always
        // opens a segment, as `None` does above.
        let (mut word, mut last, mut src, mut dst) = (u32::MAX, 0u32, 0u64, 0u64);
        while kept != 0 {
            let i = kept.trailing_zeros();
            kept &= kept - 1;
            let d = chunk[i as usize];
            let (dw, db) = (d / 64, d % 64);
            if dw != word || db <= last {
                if word != u32::MAX {
                    segments.push(Segment {
                        src_mask: src,
                        dst_mask: dst,
                        dst_word: word,
                    });
                }
                (word, src, dst) = (dw, 0, 0);
            }
            src |= 1u64 << i;
            dst |= 1u64 << db;
            last = db;
        }
        if word != u32::MAX {
            segments.push(Segment {
                src_mask: src,
                dst_mask: dst,
                dst_word: word,
            });
        }
    }

    /// Segments of source word `w`, built on first use.
    #[inline(always)]
    fn of(&self, w: usize) -> &[Segment] {
        self.words[w].get_or_init(|| Self::segments_of(&self.map, w))
    }
}

/// One layout step's rewrite plan, built whole as flat arrays: under eager
/// specialisation every kept row of a node goes through it, so nothing is
/// gained by building it a word at a time.
#[derive(Debug)]
struct StepPlan {
    /// Source word `w`'s segments are `segments[first[w]..first[w + 1]]`.
    first: Vec<u32>,
    segments: Vec<Segment>,
    /// Segment `k`'s shift runs are `runs[run_first[k]..run_first[k + 1]]`
    /// (built only for [`EagerMode::Runs`]).
    run_first: Vec<u32>,
    runs: Vec<ShiftRun>,
    /// `low[w]`: the smallest destination word of any column in source
    /// words `w ..` (`u32::MAX` if every one is deleted).  A row read from
    /// its word `lead` writes nothing before `low[lead]`.
    low: Vec<u32>,
}

impl StepPlan {
    fn new(map: &[u32], with_runs: bool) -> Self {
        Self::build(map, with_runs, RewritePlan::segments_into)
    }

    /// [`StepPlan::new`] through [`RewritePlan::segments_into_kept`]: the
    /// same plan and tally, for the batched rewrite.
    fn new_kept(map: &[u32], with_runs: bool) -> Self {
        Self::build(map, with_runs, RewritePlan::segments_into_kept)
    }

    #[inline(always)]
    fn build(
        map: &[u32],
        with_runs: bool,
        segments_into: impl Fn(&[u32], usize, &mut Vec<Segment>),
    ) -> Self {
        let n = map.len().div_ceil(64);
        let mut first = Vec::with_capacity(n + 1);
        let mut segments = Vec::new();
        for w in 0..n {
            first.push(segments.len() as u32);
            segments_into(map, w, &mut segments);
        }
        first.push(segments.len() as u32);
        let mut low = vec![u32::MAX; n + 1];
        for w in (0..n).rev() {
            let here = segments[first[w] as usize..first[w + 1] as usize]
                .iter()
                .map(|s| s.dst_word)
                .min()
                .unwrap_or(u32::MAX);
            low[w] = here.min(low[w + 1]);
        }
        let mut run_first = Vec::new();
        let mut runs = Vec::new();
        if with_runs {
            run_first.reserve(segments.len() + 1);
            for seg in &segments {
                run_first.push(runs.len() as u32);
                ShiftRun::split_into(seg, &mut runs);
            }
            run_first.push(runs.len() as u32);
        }
        Self {
            first,
            segments,
            run_first,
            runs,
            low,
        }
    }
}

/// A maximal set of a segment's source bits that its map moves by one
/// common distance: `dst |= shift(src & mask)`.  A segment's map is
/// increasing, and a layout step deletes or folds whole blocks of columns,
/// so a segment is a few such runs; together they do what `pdep ∘ pext`
/// does, with shifts and masks only, on any CPU.
#[derive(Clone, Copy, Debug)]
struct ShiftRun {
    mask: u64,
    /// Destination bit minus source bit.
    shift: i8,
}

impl ShiftRun {
    /// The runs of `seg`, appended to `runs`: the `k`-th source bit maps to
    /// the `k`-th destination bit, as under `pdep(pext(x, src), dst)`.
    fn split_into(seg: &Segment, runs: &mut Vec<ShiftRun>) {
        let (mut src, mut dst) = (seg.src_mask, seg.dst_mask);
        let mut current: Option<ShiftRun> = None;
        while src != 0 {
            let (sb, db) = (src.trailing_zeros() as i8, dst.trailing_zeros() as i8);
            src &= src - 1;
            dst &= dst - 1;
            let shift = db - sb;
            match &mut current {
                Some(run) if run.shift == shift => run.mask |= 1u64 << sb,
                _ => {
                    if let Some(run) = current.take() {
                        runs.push(run);
                    }
                    current = Some(ShiftRun {
                        mask: 1u64 << sb,
                        shift,
                    });
                }
            }
        }
        if let Some(run) = current {
            runs.push(run);
        }
    }

    #[inline(always)]
    fn apply(self, x: u64) -> u64 {
        let v = x & self.mask;
        if self.shift >= 0 {
            v << self.shift
        } else {
            v >> -self.shift
        }
    }
}

/// [`apply_step_bmi2`] with each segment applied through its shift runs:
/// the same image, on CPUs without `pext`/`pdep`.
fn apply_step_runs(plan: &StepPlan, live: &[u64], lead: usize, out: &mut [u64], lo: usize) {
    for (i, &word) in live.iter().enumerate() {
        if word == 0 {
            continue;
        }
        let w = lead + i;
        for k in plan.first[w] as usize..plan.first[w + 1] as usize {
            let runs = &plan.runs[plan.run_first[k] as usize..plan.run_first[k + 1] as usize];
            let mut image = 0u64;
            for run in runs {
                image |= run.apply(word);
            }
            out[plan.segments[k].dst_word as usize - lo] ^= image;
        }
    }
}

/// [`apply_step_bmi2`] or [`apply_step_runs`], as `mode` says.
#[inline]
fn apply_step(
    mode: EagerMode,
    plan: &StepPlan,
    live: &[u64],
    lead: usize,
    out: &mut [u64],
    lo: usize,
) {
    match mode {
        #[cfg(target_arch = "x86_64")]
        // SAFETY: `eager_mode` returns `Bmi2` only after checking BMI2.
        EagerMode::Bmi2 => unsafe { apply_step_bmi2(plan, live, lead, out, lo) },
        _ => apply_step_runs(plan, live, lead, out, lo),
    }
}

/// Apply a [`StepPlan`] to the live words of a row (`live[i]` is source
/// word `lead + i`), XORing the image into `out`, which holds destination
/// words `lo ..` with `lo ≤ plan.low[lead]`.
///
/// # Safety
/// BMI2 must be available on this CPU.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn apply_step_bmi2(plan: &StepPlan, live: &[u64], lead: usize, out: &mut [u64], lo: usize) {
    use std::arch::x86_64::{_pdep_u64, _pext_u64};
    for (i, &word) in live.iter().enumerate() {
        if word == 0 {
            continue;
        }
        let w = lead + i;
        let segments = &plan.segments[plan.first[w] as usize..plan.first[w + 1] as usize];
        for seg in segments {
            out[seg.dst_word as usize - lo] ^=
                _pdep_u64(_pext_u64(word, seg.src_mask), seg.dst_mask);
        }
    }
}

/// [`apply_step`] for the next kernels: the same image, and returns the
/// number of segments applied — the `3 ×` count the `bitop/v1` tally
/// charges — so the caller need not make a second pass over the row to
/// count them.
#[inline]
fn apply_step_counted(
    mode: EagerMode,
    plan: &StepPlan,
    live: &[u64],
    lead: usize,
    out: &mut [u64],
    lo: usize,
) -> u64 {
    match mode {
        #[cfg(target_arch = "x86_64")]
        // SAFETY: `eager_mode` returns `Bmi2` only after checking BMI2.
        EagerMode::Bmi2 => unsafe { apply_step_bmi2_counted(plan, live, lead, out, lo) },
        _ => {
            let segments = live
                .iter()
                .enumerate()
                .filter(|(_, &w)| w != 0)
                .map(|(i, _)| (plan.first[lead + i + 1] - plan.first[lead + i]) as u64)
                .sum();
            apply_step_runs(plan, live, lead, out, lo);
            segments
        }
    }
}

/// [`apply_step_bmi2`], counting the segments it applies, with the plan's
/// word bounds read pairwise.
///
/// # Safety
/// BMI2 must be available on this CPU.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn apply_step_bmi2_counted(
    plan: &StepPlan,
    live: &[u64],
    lead: usize,
    out: &mut [u64],
    lo: usize,
) -> u64 {
    use std::arch::x86_64::{_pdep_u64, _pext_u64};
    let mut applied = 0u64;
    let bounds = &plan.first[lead..lead + live.len() + 1];
    for (&word, pair) in live.iter().zip(bounds.windows(2)) {
        if word == 0 {
            continue;
        }
        let segments = &plan.segments[pair[0] as usize..pair[1] as usize];
        applied += segments.len() as u64;
        for seg in segments {
            out[seg.dst_word as usize - lo] ^=
                _pdep_u64(_pext_u64(word, seg.src_mask), seg.dst_mask);
        }
    }
    applied
}

/// Test-only, per-thread breakdown of the specialisation's rewrites, by
/// kind: `(rows, charged words)` for the bulk rewrite of kept rows
/// ([`ReducedBasis::rewrite_kept`]), the one-row materialisations and the
/// displaced rows' rewrites.  Compiled out of every non-test build.
mod rewrite_tally {
    pub(super) const BULK: usize = 0;
    pub(super) const SINGLE: usize = 1;
    pub(super) const DISPLACED: usize = 2;
    #[cfg(test)]
    thread_local! {
        static TALLY: std::cell::Cell<[(u64, u64); 3]> = const { std::cell::Cell::new([(0, 0); 3]) };
    }
    #[inline(always)]
    pub(super) fn add(_kind: usize, _rows: u64, _words: u64) {
        #[cfg(test)]
        TALLY.with(|t| {
            let mut v = t.get();
            v[_kind].0 += _rows;
            v[_kind].1 += _words;
            t.set(v);
        });
    }
    /// Read and clear this thread's tally.
    #[cfg(test)]
    pub(crate) fn take() -> [(u64, u64); 3] {
        TALLY.with(|t| t.replace([(0, 0); 3]))
    }
}
#[cfg(test)]
pub(crate) use rewrite_tally::take as take_rewrite_tally;

/// How a specialisation rewrites its kept rows; see [`eager_mode`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EagerMode {
    /// Lazily, each row from wherever it was last current when first read.
    Off,
    /// Eagerly through the step plan, segment by segment with `pext`/`pdep`.
    #[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
    Bmi2,
    /// Eagerly through the step plan, segment by segment with shift runs.
    Runs,
}

// Pre-#2043 accounting for committed evidence, set by the replay audit only
// (`ecbench::legacy`): every kept row is rewritten lazily, when first read,
// which is what every aarch64 host did before the shift-run eager rewrite
// and so what the records made on Apple silicon charge.  Per thread: the
// whole specialisation runs on the calling thread, and a test solving on
// another thread is unaffected.
thread_local! {
    static LEGACY_LAZY_REWRITE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Charge the lazy rewrite every host did before #2043 ([`eager_mode`] is
/// then `Off`); `false` restores the normal choice.  Only the replay of a
/// record made before that change turns it on.
pub fn set_legacy_lazy_rewrite(on: bool) {
    LEGACY_LAZY_REWRITE.with(|c| c.set(on));
}

/// Whether a specialisation rewrites every kept row into the child's
/// layout at once, through the step's own plan, instead of leaving each
/// to be rewritten from wherever it was last current when something first
/// reads it.  At every node most rows are read (on `K₀/2²³` dim 8, 70 %
/// of them were rewritten lazily anyway), and the lazy rewrite paid for a
/// composed map and a plan per pair of layouts.  With BMI2 a segment is one
/// `pdep ∘ pext`; without it (every aarch64 host) the same image is built
/// from its shift runs.  `KIC_F4_EAGER=0` keeps the lazy rows and
/// `KIC_F4_EAGER=runs` forces the shift runs on a BMI2 host, both as
/// same-binary controls; `KIC_F4_REWRITE=scatter` turns the eager rewrite
/// off with the word-wise one.  The row space, and so every verdict, is
/// the same in every mode; the specialisation word count charges every
/// kept row's rewrite, identically in both eager modes.
fn eager_mode() -> EagerMode {
    #[cfg(test)]
    if let Some((mode, _)) = rewrite_path_override::get() {
        return mode;
    }
    static MODE: std::sync::OnceLock<EagerMode> = std::sync::OnceLock::new();
    if LEGACY_LAZY_REWRITE.with(|c| c.get()) {
        return EagerMode::Off;
    }
    *MODE.get_or_init(|| {
        let var = std::env::var("KIC_F4_EAGER");
        if var.as_deref() == Ok("0") || std::env::var("KIC_F4_REWRITE").as_deref() == Ok("scatter")
        {
            EagerMode::Off
        } else if var.as_deref() == Ok("runs") || !wordwise_rewrite() {
            EagerMode::Runs
        } else {
            EagerMode::Bmi2
        }
    })
}

/// Test-only, per-thread choice of the rewrite path, so one process can run
/// the BMI2 path and the portable path (what every aarch64 host runs) on the
/// same input and compare images and charged work.
#[cfg(test)]
mod rewrite_path_override {
    use super::EagerMode;
    use std::cell::Cell;
    thread_local! {
        static PATH: Cell<Option<(EagerMode, bool)>> = const { Cell::new(None) };
    }
    pub(super) fn get() -> Option<(EagerMode, bool)> {
        PATH.with(Cell::get)
    }
    /// Run `f` with `eager_mode()` and `wordwise_rewrite()` forced.
    pub(super) fn with<T>(mode: EagerMode, wordwise: bool, f: impl FnOnce() -> T) -> T {
        let old = PATH.with(|p| p.replace(Some((mode, wordwise))));
        let out = f();
        PATH.with(|p| p.set(old));
        out
    }
}

/// Whether the word-wise rewrite is used: `pext`/`pdep` need BMI2, and
/// `KIC_F4_REWRITE=scatter` keeps the bit scatter as a same-binary control.
fn wordwise_rewrite() -> bool {
    #[cfg(test)]
    if let Some((_, wordwise)) = rewrite_path_override::get() {
        return wordwise;
    }
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| {
        if std::env::var("KIC_F4_REWRITE").as_deref() == Ok("scatter") {
            return false;
        }
        #[cfg(target_arch = "x86_64")]
        {
            std::arch::is_x86_feature_detected!("bmi2")
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            false
        }
    })
}

/// Apply `plan` to the live words of a row (`live[i]` is source word
/// `lead + i`), XORing the image into `staging`; returns the first
/// destination word touched, or `usize::MAX` when nothing survived.
///
/// # Safety
/// BMI2 must be available on this CPU.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "bmi2")]
unsafe fn apply_plan_bmi2(
    plan: &RewritePlan,
    live: &[u64],
    lead: usize,
    staging: &mut [u64],
) -> usize {
    use std::arch::x86_64::{_pdep_u64, _pext_u64};
    let mut lo = usize::MAX;
    for (i, &word) in live.iter().enumerate() {
        if word == 0 {
            continue;
        }
        for seg in plan.of(lead + i) {
            let x = _pext_u64(word, seg.src_mask);
            if x != 0 {
                let dw = seg.dst_word as usize;
                staging[dw] ^= _pdep_u64(x, seg.dst_mask);
                lo = lo.min(dw);
            }
        }
    }
    lo
}

/// The same with portable bit loops, for the tests and for CPUs without
/// BMI2 (where the scatter path runs instead).
#[cfg_attr(not(test), allow(dead_code))]
fn apply_plan_portable(
    plan: &RewritePlan,
    live: &[u64],
    lead: usize,
    staging: &mut [u64],
) -> usize {
    fn pext(x: u64, mut mask: u64) -> u64 {
        let mut out = 0u64;
        let mut k = 0;
        while mask != 0 {
            let b = mask.trailing_zeros();
            mask &= mask - 1;
            out |= ((x >> b) & 1) << k;
            k += 1;
        }
        out
    }
    fn pdep(x: u64, mut mask: u64) -> u64 {
        let mut out = 0u64;
        let mut k = 0;
        while mask != 0 {
            let b = mask.trailing_zeros();
            mask &= mask - 1;
            out |= ((x >> k) & 1) << b;
            k += 1;
        }
        out
    }
    let mut lo = usize::MAX;
    for (i, &word) in live.iter().enumerate() {
        if word == 0 {
            continue;
        }
        for seg in plan.of(lead + i) {
            let x = pext(word, seg.src_mask);
            if x != 0 {
                let dw = seg.dst_word as usize;
                staging[dw] ^= pdep(x, seg.dst_mask);
                lo = lo.min(dw);
            }
        }
    }
    lo
}

/// A child's generator system, computed once per node and shared by every
/// basis at it.
#[derive(Debug)]
pub struct ChildSystem {
    system: Rc<Vec<F2BoolPoly>>,
    degrees: Rc<Vec<u32>>,
    /// `(index in the child system, degree before, degree after)` for each
    /// generator whose degree dropped.
    dropped: Vec<(usize, u32, u32)>,
}

/// The column layout after `v := 1` (`bit` is `v`'s): the kept columns
/// and the folded ones `m ∖ v` merged in descending order, a folded column
/// equal to a kept one landing on it; `map` receives old → new.  Deleting
/// the columns holding `v` keeps the rest in order, and folding keeps the
/// folded ones in order among themselves, so this is one linear merge.
///
/// Its comparisons are `cmp_mono`'s popcounts, and the baseline x86-64
/// target has no `popcnt` instruction, so where the CPU has one the same
/// merge runs compiled with it; the layout is identical either way.
fn fold_layout(columns: &[Mask], bit: Mask, new_columns: &mut Vec<Mask>, map: &mut [u32]) {
    // `bitop/v1`: each old column is passed by both cursors (a bit test
    // each) and compared or written once
    bitop_tally::word_ops(3 * columns.len() as u64);
    #[cfg(target_arch = "x86_64")]
    {
        if std::arch::is_x86_feature_detected!("popcnt") {
            // SAFETY: the feature was just detected on this CPU.
            return unsafe { fold_layout_popcnt(columns, bit, new_columns, map) };
        }
    }
    fold_layout_body(columns, bit, new_columns, map)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "popcnt")]
unsafe fn fold_layout_popcnt(
    columns: &[Mask],
    bit: Mask,
    new_columns: &mut Vec<Mask>,
    map: &mut [u32],
) {
    fold_layout_body(columns, bit, new_columns, map)
}

#[inline(always)]
fn fold_layout_body(columns: &[Mask], bit: Mask, new_columns: &mut Vec<Mask>, map: &mut [u32]) {
    let n_old = columns.len();
    let (mut i, mut j) = (0usize, 0usize);
    let next_keep = |i: &mut usize| -> Option<usize> {
        while *i < n_old && columns[*i] & bit != 0 {
            *i += 1;
        }
        (*i < n_old).then_some(*i)
    };
    let next_fold = |j: &mut usize| -> Option<usize> {
        while *j < n_old && columns[*j] & bit == 0 {
            *j += 1;
        }
        (*j < n_old).then_some(*j)
    };
    loop {
        let k = next_keep(&mut i);
        let f = next_fold(&mut j);
        let target = new_columns.len() as u32;
        match (k, f) {
            (None, None) => break,
            (Some(k), None) => {
                new_columns.push(columns[k]);
                map[k] = target;
                i += 1;
            }
            (None, Some(f)) => {
                new_columns.push(columns[f] & !bit);
                map[f] = target;
                j += 1;
            }
            (Some(k), Some(f)) => {
                let (mk, mf) = (columns[k], columns[f] & !bit);
                match cmp_mono(F2BoolMono::from_mask(mk), F2BoolMono::from_mask(mf)) {
                    std::cmp::Ordering::Greater => {
                        new_columns.push(mk);
                        map[k] = target;
                        i += 1;
                    }
                    std::cmp::Ordering::Less => {
                        new_columns.push(mf);
                        map[f] = target;
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        new_columns.push(mk);
                        map[k] = target;
                        map[f] = target;
                        i += 1;
                        j += 1;
                    }
                }
            }
        }
    }
}

impl ChildSystem {
    /// [`ChildSystem::new`] taking the substituted system by value, so the
    /// generators move instead of being cloned; the solver reads the
    /// child's system back through [`ChildSystem::system`].
    ///
    /// The generators must be in canonical order, as
    /// [`F2BoolPoly::substitute`] returns them: descending, so degree
    /// first, and a generator's degree is its first term's.
    pub(crate) fn from_owned(parent_degrees: &[u32], substituted: Vec<F2BoolPoly>) -> Self {
        debug_assert_eq!(parent_degrees.len(), substituted.len());
        // `bitop/v1`: the zero test and degree comparison of each generator
        bitop_tally::word_ops(2 * substituted.len() as u64);
        let mut system = Vec::with_capacity(substituted.len());
        let mut degrees = Vec::with_capacity(substituted.len());
        let mut dropped = Vec::new();
        for (q, &old_degree) in substituted.into_iter().zip(parent_degrees) {
            if q.is_zero() {
                continue;
            }
            let new_degree = q.terms[0].degree();
            debug_assert_eq!(new_degree, poly_degree(&q), "canonical input");
            if new_degree != 0 && new_degree < old_degree {
                dropped.push((system.len(), old_degree, new_degree));
            }
            system.push(q);
            degrees.push(new_degree);
        }
        Self {
            system: Rc::new(system),
            degrees: Rc::new(degrees),
            dropped,
        }
    }

    /// The child's generators: the substituted system without its zeros.
    pub fn system(&self) -> &Rc<Vec<F2BoolPoly>> {
        &self.system
    }

    /// Whether some generator's degree dropped, so specialising a basis
    /// into this child would insert completion rows.
    pub fn has_dropped(&self) -> bool {
        !self.dropped.is_empty()
    }

    /// The child of a system whose generators have degrees
    /// `parent_degrees`, given `substituted[i]` — the image of the parent's
    /// `i`-th generator, zero or not.
    pub fn new(parent_degrees: &[u32], substituted: &[F2BoolPoly]) -> Self {
        debug_assert_eq!(parent_degrees.len(), substituted.len());
        // `bitop/v1`: the degree scan and the copy of every generator
        bitop_tally::word_ops(
            substituted
                .iter()
                .map(|q| 1 + 2 * q.terms.len() as u64)
                .sum(),
        );
        let mut system = Vec::with_capacity(substituted.len());
        let mut degrees = Vec::with_capacity(substituted.len());
        let mut dropped = Vec::new();
        for (q, &old_degree) in substituted.iter().zip(parent_degrees) {
            if q.is_zero() {
                continue;
            }
            let new_degree = poly_degree(q);
            if new_degree != 0 && new_degree < old_degree {
                dropped.push((system.len(), old_degree, new_degree));
            }
            system.push(q.clone());
            degrees.push(new_degree);
        }
        Self {
            system: Rc::new(system),
            degrees: Rc::new(degrees),
            dropped,
        }
    }
}

/// A basis row: its content, valid in the layout of epoch `version`.
///
/// Rows are shared by reference between a basis and the children
/// specialised from it; a row is rewritten into a newer layout — and
/// charged for it — only when something needs its content there.
///
/// **Trimmed.**  A row is stored from the word it starts in to the end of
/// its layout.  An echelon row is zero before its leading column, and the
/// rows the tree rewrites most lead far to the right — a displaced row's
/// image falls towards the tail, and so do the pivots it hits — so the
/// words before `lead` are neither stored, read nor written, as the
/// elimination's XORs have never touched a pivot's words before its
/// leading column either.
#[derive(Clone, Debug)]
struct LazyRow {
    version: u32,
    /// The layout word `data[0]` holds.
    start: u32,
    /// The first word that can be nonzero (`≥ start`); a rewrite reads
    /// from here.
    lead: u32,
    /// Words `start ..` to the end of the layout of `version`.
    data: RowWords,
}

/// The words of a basis row: a range of a shared buffer.  A row inserted
/// on its own owns its buffer; the kept rows of an eager specialisation
/// share one ([`ReducedBasis::rewrite_kept`]), which spares an allocation,
/// a staging pass and a copy per row; a row the batched rewrite brings
/// current on its own takes a span of the thread's [`RowArena`] chunk.
///
/// `ptr` points at the range's first word inside the buffer `owner`
/// keeps alive, so reading a row is one slice construction whatever
/// holds it.
#[derive(Clone, Debug)]
struct RowWords {
    ptr: *const u64,
    len: u32,
    _owner: RowOwner,
}

/// What keeps a [`RowWords`] range alive (held, never read).
#[derive(Clone, Debug)]
#[allow(dead_code)]
enum RowOwner {
    Words(Rc<[u64]>),
    Arena(Rc<ArenaChunk>),
}

impl RowWords {
    /// Words `off .. off + len` of `buf`.
    fn range(buf: Rc<[u64]>, off: usize, len: usize) -> Self {
        let ptr = buf[off..off + len].as_ptr();
        Self {
            ptr,
            len: len as u32,
            _owner: RowOwner::Words(buf),
        }
    }
}

impl std::ops::Deref for RowWords {
    type Target = [u64];
    #[inline]
    fn deref(&self) -> &[u64] {
        // SAFETY: `ptr .. ptr + len` lies inside the buffer `_owner` keeps
        // alive, and no one writes it once a `RowWords` exists: an `Rc`
        // buffer is shared and immutable, and an arena span is written
        // only before its `RowWords` is made ([`RowArena::span`]).
        unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) }
    }
}

impl From<Rc<[u64]>> for RowWords {
    fn from(buf: Rc<[u64]>) -> Self {
        let len = buf.len();
        Self::range(buf, 0, len)
    }
}

/// A write-once chunk of row words for the batched rewrite's one-row
/// materialisations: zeroed when allocated, handed out in disjoint spans,
/// each span written once by the rewrite that reserved it and only read
/// afterwards.  It spares the allocator a call, and a free, per row.
struct ArenaChunk {
    cells: Box<[std::cell::UnsafeCell<u64>]>,
    fill: std::cell::Cell<usize>,
}

impl std::fmt::Debug for ArenaChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArenaChunk({}/{})", self.fill.get(), self.cells.len())
    }
}

/// The thread's current [`ArenaChunk`].
struct RowArena;

impl RowArena {
    /// Words per chunk (32 KiB, under the allocator's `mmap` threshold, so
    /// a chunk is never a fresh mapping to fault in); a row longer than a
    /// quarter of it gets its own buffer instead.
    const CHUNK: usize = 1 << 12;
    /// Retired chunks kept for reuse at most.
    const POOL: usize = 64;

    /// Reserve `n` zeroed words, run `write` on them, and return them as a
    /// row's words; `None` (the span left unused) if `write` does.
    fn span<T>(n: usize, write: impl FnOnce(&mut [u64]) -> Option<T>) -> Option<(T, RowWords)> {
        thread_local! {
            static CURRENT: RefCell<Option<Rc<ArenaChunk>>> = const { RefCell::new(None) };
            /// Full chunks, reused once no row holds them any more.
            static RETIRED: RefCell<Vec<Rc<ArenaChunk>>> = const { RefCell::new(Vec::new()) };
        }
        if n > Self::CHUNK / 4 {
            let mut buf: Rc<[u64]> = std::iter::repeat_n(0u64, n).collect();
            let t = write(Rc::get_mut(&mut buf).expect("just allocated"))?;
            return Some((t, buf.into()));
        }
        let chunk = CURRENT.with(|c| {
            let mut c = c.borrow_mut();
            match &*c {
                Some(chunk) if chunk.fill.get() + n <= chunk.cells.len() => chunk.clone(),
                _ => {
                    let chunk = RETIRED.with(|r| {
                        let mut retired = r.borrow_mut();
                        if let Some(old) = c.take() {
                            retired.push(old);
                        }
                        // A chunk only the pool holds has no live span: zero
                        // what was handed out and start it again.
                        match retired.iter().position(|ch| Rc::strong_count(ch) == 1) {
                            Some(i) => {
                                let mut ch = retired.swap_remove(i);
                                let m = Rc::get_mut(&mut ch).expect("only the pool holds it");
                                let used = m.fill.get();
                                for cell in &mut m.cells[..used] {
                                    *cell.get_mut() = 0;
                                }
                                m.fill.set(0);
                                ch
                            }
                            None => {
                                if retired.len() > Self::POOL {
                                    // Forget the oldest; its rows keep it alive.
                                    retired.remove(0);
                                }
                                Rc::new(ArenaChunk {
                                    cells: (0..Self::CHUNK)
                                        .map(|_| std::cell::UnsafeCell::new(0))
                                        .collect(),
                                    fill: std::cell::Cell::new(0),
                                })
                            }
                        }
                    });
                    *c = Some(chunk.clone());
                    chunk
                }
            }
        });
        let off = chunk.fill.get();
        chunk.fill.set(off + n);
        // SAFETY: `off .. off + n` was just reserved and is handed out
        // once: no other reference to it exists.  `UnsafeCell<u64>` has the
        // layout of `u64`, and every cell is initialised (zeroed).
        let ptr = chunk.cells[off..off + n].as_ptr() as *mut u64;
        let out = unsafe { std::slice::from_raw_parts_mut(ptr, n) };
        let t = write(out)?;
        Some((
            t,
            RowWords {
                ptr: ptr as *const u64,
                len: n as u32,
                _owner: RowOwner::Arena(chunk),
            },
        ))
    }
}

impl From<Vec<u64>> for RowWords {
    fn from(words: Vec<u64>) -> Self {
        Rc::<[u64]>::from(words).into()
    }
}

/// A zeroed shared buffer of `n` words, allocated once (an `Rc<[u64]>`
/// made from a `Vec` would copy it).
#[allow(dead_code)]
fn zeroed_words(n: usize) -> Rc<[u64]> {
    bitop_tally::word_ops(n as u64);
    std::iter::repeat_n(0u64, n).collect()
}

impl LazyRow {
    /// Word `w` of the row in its own layout.
    fn word(&self, w: usize) -> u64 {
        if w < self.lead as usize {
            0
        } else {
            self.data[w - self.start as usize]
        }
    }

    /// The words from `w` (at least `lead`) to the end of the layout.
    #[allow(clippy::wrong_self_convention)]
    fn from_word(&self, w: usize) -> &[u64] {
        debug_assert!(w >= self.lead as usize);
        &self.data[w - self.start as usize..]
    }

    /// The words a rewrite reads.
    fn live(&self) -> &[u64] {
        self.from_word(self.lead as usize)
    }
}

/// A row being reduced into a basis: words `start ..` to the end of the
/// current layout.
struct Draft {
    start: usize,
    /// Uniquely owned until the draft becomes a pivot row, which then
    /// takes the allocation as it is instead of copying it.
    data: Rc<[u64]>,
}

impl Draft {
    /// A full-width row.
    fn full(data: Vec<u64>) -> Self {
        Self {
            start: 0,
            data: Rc::from(data),
        }
    }

    /// The words, for reduction; the draft is the only holder.
    fn data_mut(&mut self) -> &mut [u64] {
        Rc::get_mut(&mut self.data).expect("a draft is uniquely owned")
    }

    /// Leading set column at or after word `from` (a layout word).
    fn leading_column_from(&self, from: usize) -> Option<usize> {
        let found = self.data[from - self.start..].iter().position(|&w| w != 0);
        // `bitop/v1`: one test a word scanned
        bitop_tally::word_ops(
            found.map_or((self.data.len() + self.start - from) as u64, |i| {
                i as u64 + 1
            }),
        );
        found.map(|i| {
            let w = from + i;
            w * 64 + self.data[w - self.start].trailing_zeros() as usize
        })
    }
}

thread_local! {
    /// Full-width staging for [`ReducedBasis::rewrite`], zero between uses.
    static STAGING: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    /// Open-addressing slots for [`odd_terms`], and the call's stamp: a
    /// slot is live only if its high half is the current stamp, and then
    /// its low half is `k + 1` for the `k`-th distinct monomial seen.  The
    /// stamp spares clearing the table on every call.
    static ODD_SLOTS: RefCell<(Vec<u64>, u32)> = const { RefCell::new((Vec::new(), 0)) };
}

/// The completion rows of one specialisation, gathered before the layout
/// grows to hold them.
///
/// A product `t·p` has the terms `m ∪ t` of odd multiplicity, and the
/// rows of one generator share most of their monomials, many of them new
/// to the layout.  Each occurrence is hashed once, to a batch-local id;
/// a row's parity is taken on one bit per id, and whether a monomial is in
/// the layout is asked once per distinct monomial.  The rows, their order
/// and the monomials added to the layout are exactly those of multiplying
/// out each row, cancelling, and extending the layout by every monomial of
/// a kept row that it lacks.
#[derive(Default)]
struct CompletionBatch {
    /// Monomial → id.
    ids: MaskMap<u32>,
    /// Per id: the monomial, its column in the layout the batch was
    /// gathered against (`DELETED` if absent), and whether a kept row
    /// holds it.
    monos: Vec<Mask>,
    old_column: Vec<u32>,
    used: Vec<bool>,
    /// One parity bit per id, zero between rows.
    parity: Vec<u64>,
    /// The ids of the row being gathered, with repeats.
    scratch: Vec<u32>,
    /// The kept rows: the ids of their terms, in order of first occurrence.
    rows: Vec<Vec<u32>>,
}

impl CompletionBatch {
    /// Gather the row with terms `monos` (repeats cancelling in pairs)
    /// against the layout `index`; kept only if something survives.
    fn push_row(&mut self, monos: impl ExactSizeIterator<Item = Mask>, index: &MaskMap<usize>) {
        self.scratch.clear();
        let len = monos.len();
        self.scratch.reserve(len);
        self.monos.reserve(len);
        self.old_column.reserve(len);
        self.used.reserve(len);
        // `bitop/v1`: a hash probe a monomial (and one into the layout for
        // a new one), one parity flip a monomial and one test an occurrence
        bitop_tally::word_ops(2 * len as u64);
        let before = self.monos.len();
        for m in monos {
            let next = self.monos.len() as u32;
            let id = *self.ids.entry(m).or_insert(next);
            if id == next {
                self.monos.push(m);
                self.old_column
                    .push(index.get(&m).map_or(DELETED, |&c| c as u32));
                self.used.push(false);
                if id as usize / 64 >= self.parity.len() {
                    self.parity.push(0);
                }
            }
            self.parity[id as usize / 64] ^= 1u64 << (id % 64);
            self.scratch.push(id);
        }
        bitop_tally::hash_probes((len + self.monos.len() - before) as u64);
        // Emit each odd id at its first occurrence and clear its bit; the
        // even ones are already clear.
        let mut row = Vec::with_capacity(len);
        for &id in &self.scratch {
            let (w, bit) = (id as usize / 64, 1u64 << (id % 64));
            if self.parity[w] & bit != 0 {
                self.parity[w] &= !bit;
                self.used[id as usize] = true;
                row.push(id);
            }
        }
        if !row.is_empty() {
            self.rows.push(row);
        }
    }

    /// Grow `basis`'s layout by the monomials the kept rows hold and it
    /// lacks, and return every used id's column in the new layout.
    fn place(&self, basis: &mut ReducedBasis) -> Vec<u32> {
        // `bitop/v1`: the scan for missing monomials and the remap of every
        // id (the merge and sort are charged below)
        bitop_tally::word_ops(2 * self.monos.len() as u64);
        let mut missing: Vec<Mask> = (0..self.monos.len())
            .filter(|&i| self.used[i] && self.old_column[i] == DELETED)
            .map(|i| self.monos[i])
            .collect();
        if missing.is_empty() {
            return self.old_column.clone();
        }
        bitop_tally::word_ops(bitop_tally::sort_cost(missing.len()));
        bitop_tally::hash_probes(missing.len() as u64);
        sort_masks_descending(&mut missing);
        let placed = basis.insert_columns(&missing);
        let step = basis
            .history
            .last()
            .expect("a step was just adopted")
            .map
            .clone();
        let mut column: Vec<u32> = self
            .old_column
            .iter()
            .map(|&c| {
                if c == DELETED {
                    DELETED
                } else {
                    step[c as usize]
                }
            })
            .collect();
        for (m, c) in missing.iter().zip(placed) {
            column[self.ids[m] as usize] = c;
        }
        column
    }
}

/// `dst ^= src` over their common length, in fixed-width chunks the
/// compiler packs into vector XORs on the baseline target.  (An AVX2
/// build of the same loop was measured 2–8 % slower on the n = 23 cells
/// of `f6_ic_probe`: the rows are a few dozen words, so the wider vector
/// does not pay for its dispatch.)
#[inline]
fn xor_words(dst: &mut [u64], src: &[u64]) {
    let n = dst.len().min(src.len());
    let (dst, src) = (&mut dst[..n], &src[..n]);
    let mut d4 = dst.chunks_exact_mut(4);
    let mut s4 = src.chunks_exact(4);
    for (d, s) in (&mut d4).zip(&mut s4) {
        d[0] ^= s[0];
        d[1] ^= s[1];
        d[2] ^= s[2];
        d[3] ^= s[3];
    }
    for (d, &s) in d4.into_remainder().iter_mut().zip(s4.remainder()) {
        *d ^= s;
    }
}

/// [`xor_words`] for the next kernels: the same XORs, compiled for
/// AVX-512 or AVX2 where the CPU has them (chosen once per process;
/// `KIC_GF2_FORCE_AVX2=1` caps it at AVX2, as for the Four Russians
/// kernel, so a profiler that cannot decode AVX-512 can run it).
#[inline]
fn xor_words_simd(dst: &mut [u64], src: &[u64]) {
    #[cfg(target_arch = "x86_64")]
    {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Kind {
            Scalar,
            Avx2,
            Avx512,
        }
        static KIND: std::sync::OnceLock<Kind> = std::sync::OnceLock::new();
        let kind = *KIND.get_or_init(|| {
            let force_avx2 = std::env::var("KIC_GF2_FORCE_AVX2").as_deref() == Ok("1");
            if !force_avx2 && std::arch::is_x86_feature_detected!("avx512f") {
                Kind::Avx512
            } else if std::arch::is_x86_feature_detected!("avx2") {
                Kind::Avx2
            } else {
                Kind::Scalar
            }
        });
        match kind {
            // SAFETY: the feature was detected on this CPU above.
            Kind::Avx512 => return unsafe { xor_words_avx512(dst, src) },
            // SAFETY: as above.
            Kind::Avx2 => return unsafe { xor_words_avx2(dst, src) },
            Kind::Scalar => {}
        }
    }
    xor_words(dst, src)
}

#[inline(always)]
fn xor_words_plain(dst: &mut [u64], src: &[u64]) {
    for (d, &s) in dst.iter_mut().zip(src) {
        *d ^= s;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f")]
unsafe fn xor_words_avx512(dst: &mut [u64], src: &[u64]) {
    xor_words_plain(dst, src)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn xor_words_avx2(dst: &mut [u64], src: &[u64]) {
    xor_words_plain(dst, src)
}

/// The monomials occurring an odd number of times in `monos`, in order of
/// first occurrence: the terms of `Σ monos` over `F_2`.  A product `t·p`
/// in the Boolean ring maps the terms of `p` to `m ∪ t`, and two terms that
/// differ only inside `t` meet and cancel.  Order does not matter to any
/// caller (each packs the row by column or collects it into a set), so the
/// parity is taken with a small hash table rather than by sorting.
fn odd_terms(monos: impl ExactSizeIterator<Item = Mask>) -> Vec<Mask> {
    let len = monos.len();
    // `bitop/v1`: a hash probe a monomial, and the compaction scan
    bitop_tally::hash_probes(len as u64);
    bitop_tally::word_ops(len as u64);
    let mut keys: Vec<Mask> = Vec::with_capacity(len);
    let mut odd: Vec<bool> = Vec::with_capacity(len);
    ODD_SLOTS.with(|state| {
        let mut state = state.borrow_mut();
        let (slots, stamp) = &mut *state;
        let bits = (2 * len).max(16).next_power_of_two().trailing_zeros();
        let cap = 1usize << bits;
        *stamp = stamp.wrapping_add(1);
        if *stamp == 0 || slots.len() < cap {
            // a wrapped stamp could meet a stale slot; a grown table has
            // fresh ones: either way start from zeros
            slots.clear();
            slots.resize(cap.max(slots.capacity()), 0);
            *stamp = 1;
        }
        let live = u64::from(*stamp) << 32;
        for m in monos {
            let mut s =
                (mask_fold64(m).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - bits)) as usize;
            loop {
                let slot = slots[s];
                if slot & !0xFFFF_FFFF != live {
                    keys.push(m);
                    odd.push(true);
                    slots[s] = live | keys.len() as u64;
                    break;
                }
                let k = (slot & 0xFFFF_FFFF) as usize - 1;
                if keys[k] == m {
                    odd[k] ^= true;
                    break;
                }
                s = (s + 1) & (cap - 1);
            }
        }
    });
    let mut kept = 0;
    for i in 0..keys.len() {
        if odd[i] {
            keys[kept] = keys[i];
            kept += 1;
        }
    }
    keys.truncate(kept);
    keys
}

/// The reduced degree-`degree` Macaulay row space of a system, kept as an
/// echelon basis with distinct leading monomials, together with the
/// system it belongs to.
///
/// **Lazy materialisation.**  Only a fraction of the rows a level keeps
/// are ever touched at that level — displaced, hit as a pivot while a
/// displaced row reduces, or read for the tail; on `K_1/2^23` three
/// quarters are not.  So a kept row is not rewritten into the child's
/// layout: the child shares the parent's row by reference and records
/// the layout step, and a row is materialised only when needed, through
/// the composition of every step since it was last written, in one pass
/// charged once.  Its pivot column is tracked through the steps meanwhile,
/// which is all `specialise` needs to decide whether it stays a pivot.
#[derive(Clone, Debug)]
pub struct ReducedBasis {
    /// Macaulay degree.
    pub degree: u32,
    /// Number of Boolean variables of the ambient ring.
    pub n_vars: usize,
    /// The system whose Macaulay row space this is.
    pub system: Rc<Vec<F2BoolPoly>>,
    /// Degree of each generator when the basis was last completed.
    generator_degrees: Rc<Vec<u32>>,
    /// Variables specialised away on the path from the root.
    assigned: Mask,
    /// Column monomials of the current layout, in descending order.
    columns: Vec<Mask>,
    /// Monomial → column, built only when a row has to be packed from
    /// monomials (completion), which most levels never do.
    column_index: Option<MaskMap<usize>>,
    words: usize,
    /// Layout steps since the root: `history[e]` maps epoch `e` to `e + 1`.
    /// The current epoch is `history.len()`.
    history: Vec<Rc<LayoutStep>>,
    /// Echelon rows; every row is nonzero and their leading columns are
    /// pairwise distinct.
    rows: Vec<LazyRow>,
    /// `pivot_col[r]` is row `r`'s leading column in the current layout.
    pivot_col: Vec<u32>,
    /// `pivot_of[c]` is the row leading at column `c`.
    pivot_of: Vec<Option<u32>>,
    /// Rounds of degree-fall closure run per node; `0` keeps the plain
    /// Macaulay row space (see [`ReducedBasis::from_system_closed`]).
    closure_rounds: u32,
    /// Least leading degree of a row the closure multiplies out: `2` for
    /// the splitting solver, which eliminates linear rows in its tail; `1`
    /// for full MutantXL ([`ReducedBasis::from_system_closed_full`]).
    min_fall_degree: u32,
    /// Degree falls not yet multiplied by the variables, as row indices.
    pending: Vec<u32>,
    /// The row space contains the constant `1`.  The basis is then kept as
    /// that one row: every specialisation of `1` is `1`, and the solver
    /// reads nothing past it (see [`ReducedBasis::specialise_shared`]).
    refuted: bool,
    /// Multiply each generator only by the variables in its own support,
    /// at the root and in completion rows, instead of by every variable
    /// occurring in the system ([`ReducedBasis::from_system_with`]).
    /// Inherited by every basis specialised from this one.
    support_local: bool,
}

impl ReducedBasis {
    /// Build and reduce the Macaulay matrix of `system` at `degree` from
    /// scratch — the root of a splitting tree, or any node whose parent had
    /// no basis.  Multipliers range over the variables occurring in the
    /// system, the from-scratch step's own active-multiplier policy: a
    /// multiplier containing a variable that occurs nowhere adds rows that
    /// cannot reach the linear tail (see [`ReducedBasis::specialise`]).
    /// `None` if the matrix exceeds the F4 size caps.
    pub fn from_system(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
    ) -> Option<(Self, InheritCost)> {
        Self::from_system_with(system, n_vars, degree, false)
    }

    /// [`ReducedBasis::from_system`] with a choice of multipliers.  With
    /// `support_local`, generator `f` is multiplied only by monomials in the
    /// variables of its own support, so the row space is spanned by
    /// `{t·f : t ⊆ supp(f), deg t·f ≤ degree}` — a subspace of the
    /// occurring-variable one, so every consequence it yields is still in
    /// the ideal and the splitting solver still decides every target
    /// exactly; it may read a weaker tail at a node.  The policy is kept by
    /// every specialisation, whose completion rows follow it too.  On a
    /// chain in the interleaved order it drops the products of the last
    /// link by the first summands' variables, which the tail never used
    /// (`RESEARCH_SUPPORT_LOCAL_MULTIPLIERS.md`).
    pub fn from_system_with(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
    ) -> Option<(Self, InheritCost)> {
        let root = RootMatrix::build(system, n_vars, degree, support_local)?;
        let mut cost = InheritCost::default();
        let RootMatrix {
            system,
            generator_degrees,
            columns,
            mut matrix,
        } = root;
        if matrix.is_empty() {
            let out = Self::assemble(
                system,
                generator_degrees,
                n_vars,
                degree,
                support_local,
                Vec::new(),
                Vec::new(),
            );
            return Some((out, cost));
        }
        // A basis needs distinct leading columns and nothing more, so the
        // root is reduced to echelon form without back-substitution.  A
        // fully reduced root makes every later displaced row reduce in as
        // many XORs as it has pivot bits, and below 24 variables that paid
        // for its back-substitution while every child re-reduced all of its
        // displaced rows.  Now that a refuted child stops at its refutation
        // and a rewrite skips a row's leading zeros, the children no longer
        // earn it back on any rung (`RESEARCH_INHERITED_F4.md` §3.1).  A
        // middle ground, clearing only the pivot columns of degree below
        // `D`, was tried and costs what it saves.
        // `KIC_F4_INHERIT_ROOT=rref` restores the fully reduced root as a
        // control.
        if root_replay() {
            replay_root(
                &columns,
                &mut matrix,
                degree,
                support_local,
                &mut cost.reduce_word_ops,
            );
        }
        let rank = if root_rref() {
            rref_f2_counted(&mut matrix, columns.len(), &mut cost.reduce_word_ops)
        } else {
            echelon_f2_counted(&mut matrix, columns.len(), &mut cost.reduce_word_ops)
        };
        note_root_elimination(cost.reduce_word_ops);
        matrix.truncate(rank);
        let out = Self::assemble(
            system,
            generator_degrees,
            n_vars,
            degree,
            support_local,
            columns,
            matrix,
        );
        Some((out, cost))
    }

    /// A root basis from its reduced rows (distinct leading columns, in the
    /// order the elimination left them), collapsed to `1` if it holds it.
    fn assemble(
        system: Rc<Vec<F2BoolPoly>>,
        generator_degrees: Rc<Vec<u32>>,
        n_vars: usize,
        degree: u32,
        support_local: bool,
        columns: Vec<Mask>,
        rows: Vec<Vec<u64>>,
    ) -> Self {
        let mut out = Self {
            degree,
            n_vars,
            system,
            generator_degrees,
            assigned: 0,
            column_index: None,
            words: columns.len().div_ceil(64).max(1),
            pivot_of: vec![None; columns.len()],
            columns,
            history: Vec::new(),
            rows: Vec::new(),
            pivot_col: Vec::new(),
            closure_rounds: 0,
            min_fall_degree: 2,
            pending: Vec::new(),
            refuted: false,
            support_local,
        };
        // `bitop/v1`: the cleared pivot index, and each row's lead scan
        bitop_tally::word_ops(
            out.pivot_of.len() as u64
                + rows
                    .iter()
                    .map(|r| 1 + r.iter().take_while(|&&w| w == 0).count() as u64)
                    .sum::<u64>(),
        );
        for row in rows {
            let c = leading_column(&row).expect("rank rows are nonzero");
            debug_assert!(out.pivot_of[c].is_none());
            out.pivot_of[c] = Some(out.rows.len() as u32);
            out.pivot_col.push(c as u32);
            out.rows.push(LazyRow {
                version: 0,
                start: 0,
                lead: (c / 64) as u32,
                data: row.into(),
            });
        }
        if let Some(r) = out.one_row() {
            out.collapse_to_one(r);
        }
        out
    }

    /// **Lane-batched roots** (lit2026 W5, after Demin's SIMD-packed
    /// tracing in Groebner.jl, arXiv 2607.06372).  The roots of `systems`
    /// — `k ≤ 64` instances, typically one per target point — built
    /// together: each lane's Macaulay rows are generated exactly as
    /// [`ReducedBasis::from_system_with`] generates them, laid out on the
    /// union of the lanes' columns and **bit-sliced**, one lane word per
    /// matrix entry with bit `ℓ` for lane `ℓ`, then reduced to reduced row
    /// echelon form by one Gauss–Jordan pass in which a row operation acts
    /// on every lane at once, masked to the lanes that need it.
    ///
    /// Pivots are chosen per lane: at column `c`, the first row (in row
    /// order) that is still free in a lane and holds `c` there pivots that
    /// lane.  When the lanes share their pivot pattern one row pivots all
    /// of them and one pass serves `k` instances; when they diverge the
    /// column takes one pass per distinct pivot row, which is the
    /// detection and the fall-back in one ([`LaneStats::pivot_passes`]
    /// against [`LaneStats::pivot_columns`]).  Each lane runs exactly
    /// Gauss–Jordan elimination on its own rows, so its result is the
    /// unique reduced row echelon form of its own Macaulay row space: the
    /// basis [`ReducedBasis::from_system_with`] returns under
    /// `KIC_F4_INHERIT_ROOT=rref`, row for row.  Under the default
    /// echelon root the row space, and so every tail, refutation, forced
    /// variable and verdict, is the same; only the rows that span it differ.
    ///
    /// Charged per lane: its share of the elimination's lane-word
    /// operations, in 64-bit word equivalents (a lane word of `w` bits
    /// counts `w / 64`), split evenly over the active lanes.  Over `F₂` a
    /// packed row already holds 64 coefficients per word, so a bit-sliced
    /// row operation on `k` lanes moves the same number of bits as `k`
    /// packed ones only when `k = 64` (see the W5 study).  A lane whose
    /// matrix exceeds the F4 caps, or a batch past the memory cap, comes
    /// back `None` and is built on its own by the caller.
    pub fn from_systems_lanes(
        systems: &[&[F2BoolPoly]],
        n_vars: usize,
        degree: u32,
        support_local: bool,
    ) -> (Vec<Option<(Self, InheritCost)>>, LaneStats) {
        let k = systems.len();
        assert!(k <= 64, "at most 64 lanes");
        let mut stats = LaneStats {
            lanes: k as u32,
            ..LaneStats::default()
        };
        let roots: Vec<Option<RootMatrix>> = systems
            .iter()
            .map(|s| RootMatrix::build(s, n_vars, degree, support_local))
            .collect();
        let active: Vec<usize> = (0..k)
            .filter(|&l| roots[l].as_ref().is_some_and(|r| !r.matrix.is_empty()))
            .collect();
        let mut out: Vec<Option<(Self, InheritCost)>> = (0..k).map(|_| None).collect();
        // Empty matrices need no elimination.
        for (l, root) in roots.iter().enumerate() {
            if let Some(r) = root {
                if r.matrix.is_empty() {
                    out[l] = Some((
                        Self::assemble(
                            r.system.clone(),
                            r.generator_degrees.clone(),
                            n_vars,
                            degree,
                            support_local,
                            Vec::new(),
                            Vec::new(),
                        ),
                        InheritCost::default(),
                    ));
                }
            }
        }
        if active.is_empty() {
            return (out, stats);
        }
        let width = match active.len() {
            0..=8 => 8,
            9..=16 => 16,
            17..=32 => 32,
            _ => 64,
        };
        let reduced = match width {
            8 => lanes::reduce::<u8>(&roots, &active, &mut stats),
            16 => lanes::reduce::<u16>(&roots, &active, &mut stats),
            32 => lanes::reduce::<u32>(&roots, &active, &mut stats),
            _ => lanes::reduce::<u64>(&roots, &active, &mut stats),
        };
        let Some(reduced) = reduced else {
            return (out, stats);
        };
        stats.lane_bits = width;
        // 64-bit word equivalents, split evenly; the first lanes take the
        // remainder so the shares sum to the whole.
        let equivalent = stats.lane_word_ops * width as u64 / 64;
        stats.word_equivalent = equivalent;
        let a = active.len() as u64;
        let mut roots = roots;
        for (i, (&l, rows)) in active.iter().zip(reduced).enumerate() {
            let root = roots[l].take().expect("active lanes have a root");
            let share = equivalent / a + u64::from((i as u64) < equivalent % a);
            note_root_elimination(share);
            let cost = InheritCost {
                reduce_word_ops: share,
                ..InheritCost::default()
            };
            let basis = Self::assemble(
                root.system,
                root.generator_degrees,
                n_vars,
                degree,
                support_local,
                root.columns,
                rows,
            );
            out[l] = Some((basis, cost));
        }
        (out, stats)
    }

    /// Do two bases on the same column layout span the same row space?
    /// Both are read as packed rows and brought to reduced row echelon
    /// form, which is unique.  A diagnostic for the lane-batched roots;
    /// nothing is charged.
    pub fn same_row_space(&self, other: &Self) -> bool {
        if self.refuted || other.refuted {
            return self.refuted == other.refuted;
        }
        if self.columns != other.columns || self.rows.len() != other.rows.len() {
            return false;
        }
        let words = self.words;
        let packed = |b: &Self| -> Vec<Vec<u64>> {
            let mut b = b.clone();
            let mut ignored = InheritCost::default();
            b.materialise_all(&mut ignored);
            b.rows
                .iter()
                .map(|r| (0..words).map(|w| r.word(w)).collect())
                .collect()
        };
        let (mut x, mut y) = (packed(self), packed(other));
        if x.is_empty() {
            return y.is_empty();
        }
        let mut ignored = 0u64;
        let rx = rref_f2_counted(&mut x, self.columns.len(), &mut ignored);
        let ry = rref_f2_counted(&mut y, other.columns.len(), &mut ignored);
        x.truncate(rx);
        y.truncate(ry);
        x == y
    }

    /// The row leading at the constant monomial, if the row space has one.
    /// `1` is the smallest monomial of any degree-compatible order, so it
    /// can only be the last column.
    fn one_row(&self) -> Option<usize> {
        if self.columns.last() != Some(&0) {
            return None;
        }
        self.pivot_of.last().copied().flatten().map(|r| r as usize)
    }

    /// Reduce the basis to its row `r`, which is the constant `1`.
    fn collapse_to_one(&mut self, r: usize) {
        debug_assert_eq!(self.columns[self.pivot_col[r] as usize], 0);
        if COMPACT_REFUTED_BASIS.load(Ordering::Relaxed) {
            self.columns = vec![0];
            self.column_index = None;
            self.words = 1;
            self.history.clear();
            self.rows = vec![LazyRow {
                version: 0,
                start: 0,
                lead: 0,
                data: Rc::<[u64]>::from([1u64].as_slice()).into(),
            }];
            self.pivot_col = vec![0];
            self.pivot_of = vec![Some(0)];
            self.pending.clear();
            self.refuted = true;
            return;
        }
        let row = self.rows.swap_remove(r);
        let c = self.pivot_col[r];
        self.rows = vec![row];
        self.pivot_col = vec![c];
        self.pivot_of = vec![None; self.columns.len()];
        self.pivot_of[c as usize] = Some(0);
        self.column_index = None;
        self.pending.clear();
        self.refuted = true;
    }

    /// Monomial → current column, built on first use after a layout change.
    fn column_index(&mut self) -> &MaskMap<usize> {
        if self.column_index.is_none() {
            bitop_tally::hash_probes(self.columns.len() as u64);
            self.column_index = Some(
                self.columns
                    .iter()
                    .enumerate()
                    .map(|(i, &m)| (m, i))
                    .collect(),
            );
        }
        self.column_index.as_ref().expect("just built")
    }

    /// [`ReducedBasis::from_system`], closed under multiplying degree falls
    /// by monomials for `rounds` rounds at this node and at every node
    /// specialised from it (see [`ReducedBasis::close`]).  The falls seeded
    /// at the root are the rows whose leading monomial has degree in
    /// `[2, degree)` and is not a leading monomial of the degree-`degree−1`
    /// Macaulay row space: a row of that space times a monomial of degree
    /// one is already a row of the degree-`degree` matrix.
    pub fn from_system_closed(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
    ) -> Option<(Self, InheritCost)> {
        Self::from_system_closed_with(system, n_vars, degree, rounds, false)
    }

    /// [`ReducedBasis::from_system_closed`] with the multiplier choice of
    /// [`ReducedBasis::from_system_with`].
    pub fn from_system_closed_with(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
        support_local: bool,
    ) -> Option<(Self, InheritCost)> {
        Self::from_system_closed_impl(system, n_vars, degree, rounds, support_local, 2)
    }

    /// Full MutantXL at fixed degree: [`ReducedBasis::from_system_closed`]
    /// with linear falls multiplied out as well (the splitting solver leaves
    /// those to its linear tail).  The saturated row space is the degree-
    /// `degree` part of the closure of the Macaulay row space under
    /// multiplying any element of degree `< degree` by a variable; it
    /// contains `1` from a degree on that is never above XL's.  Degrees
    /// below 2 build no closure.
    pub fn from_system_closed_full(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
    ) -> Option<(Self, InheritCost)> {
        Self::from_system_closed_impl(system, n_vars, degree, rounds, false, 1)
    }

    fn from_system_closed_impl(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
        support_local: bool,
        min_fall_degree: u32,
    ) -> Option<(Self, InheritCost)> {
        let (mut basis, mut cost) = Self::seed_closure(
            system,
            n_vars,
            degree,
            rounds,
            support_local,
            min_fall_degree,
        )?;
        basis.close(&mut cost);
        Some((basis, cost))
    }

    /// Full MutantXL at fixed degree, stopped before its first round: the
    /// root basis with the falls seeded and nothing multiplied out.  The
    /// first state a closure checkpoint can hold; [`ReducedBasis::close_with`]
    /// continues it.  [`ReducedBasis::from_system_closed_full`] is this plus
    /// `close`, so the two reach the same basis.
    pub fn seed_closure_full(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
    ) -> Option<(Self, InheritCost)> {
        Self::seed_closure(system, n_vars, degree, rounds, false, 1)
    }

    fn seed_closure(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
        support_local: bool,
        min_fall_degree: u32,
    ) -> Option<(Self, InheritCost)> {
        let (mut basis, mut cost) = Self::from_system_with(system, n_vars, degree, support_local)?;
        if rounds == 0 || degree < min_fall_degree + 1 || basis.rows.is_empty() || basis.refuted {
            return Some((basis, cost));
        }
        basis.closure_rounds = rounds;
        basis.min_fall_degree = min_fall_degree;
        let occurring = basis
            .system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0 as Mask, |acc, t| acc | t.mask)
            & all_variable_mask(n_vars);
        // Full MutantXL seeds every fall: a fall of the degree-`degree−1`
        // space times a monomial of degree two or more is not in general a
        // row of this matrix.
        let lower = if min_fall_degree >= 2 {
            macaulay_rows_monos_with_mask(
                basis.system.as_slice(),
                n_vars,
                degree - 1,
                occurring,
                None,
            )?
        } else {
            Vec::new()
        };
        let mut lower_leading: std::collections::HashSet<Mask> = std::collections::HashSet::new();
        if !lower.is_empty() {
            let columns = macaulay_columns(&lower)?;
            let mut matrix = pack_rows(&lower, &columns);
            let rank = echelon_f2_counted(&mut matrix, columns.len(), &mut cost.reduce_word_ops);
            for row in matrix.iter().take(rank) {
                if let Some(c) = leading_column(row) {
                    lower_leading.insert(columns[c]);
                }
            }
        }
        for r in 0..basis.rows.len() {
            let lead = basis.columns[basis.pivot_col[r] as usize];
            if basis.is_fall(r) && !lower_leading.contains(&lead) {
                basis.pending.push(r as u32);
            }
        }
        Some((basis, cost))
    }

    /// Whether this basis multiplies generators only within their supports
    /// ([`ReducedBasis::from_system_with`]).
    pub fn support_local(&self) -> bool {
        self.support_local
    }

    /// Variables specialised away since the root, as a mask.
    pub fn assigned(&self) -> Mask {
        self.assigned
    }

    /// Rank of the row space.
    pub fn rank(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns.
    pub fn columns(&self) -> usize {
        self.columns.len()
    }

    /// Whether the row space contains the constant `1`.
    pub fn is_refuted(&self) -> bool {
        self.refuted
    }

    /// The closure's state if it is between rounds: not refuted, every row
    /// materialised in the current layout from its first word.  That holds
    /// right after the seeding ([`ReducedBasis::seed_closure_full`]) and
    /// after every absorbed round, which are the only points a checkpoint is
    /// taken; anywhere else this is `None`.
    pub fn closure_view(&self) -> Option<ClosureView<'_>> {
        let epoch = self.epoch();
        if self.refuted
            || self.closure_rounds == 0
            || self
                .rows
                .iter()
                .any(|r| r.version != epoch || r.start != 0 || r.data.len() != self.words)
        {
            return None;
        }
        Some(ClosureView {
            degree: self.degree,
            n_vars: self.n_vars,
            support_local: self.support_local,
            min_fall_degree: self.min_fall_degree,
            closure_rounds: self.closure_rounds,
            words: self.words,
            columns: &self.columns,
            pending: &self.pending,
            rows: &self.rows,
        })
    }

    /// The basis a [`ClosureView`] described, rebuilt from `system` (the
    /// same system, which the caller checks by digest) and the saved
    /// parts.  It is a root-like basis, with no layout history: every row is
    /// in the one layout, so `close_with` continues exactly as the
    /// uninterrupted closure would.  `None` if the parts do not fit
    /// (a row of the wrong length, rows without distinct leading columns,
    /// a pending index out of range).
    #[allow(clippy::too_many_arguments)]
    pub fn from_closure_state(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
        min_fall_degree: u32,
        closure_rounds: u32,
        columns: Vec<Mask>,
        rows: Vec<Vec<u64>>,
        pending: Vec<u32>,
    ) -> Option<Self> {
        let words = columns.len().div_ceil(64).max(1);
        if rows.iter().any(|r| r.len() != words)
            || pending.iter().any(|&p| p as usize >= rows.len())
        {
            return None;
        }
        let system = Rc::new(system.to_vec());
        let generator_degrees = Rc::new(system.iter().map(poly_degree).collect());
        let mut out = Self::assemble(
            system,
            generator_degrees,
            n_vars,
            degree,
            support_local,
            columns,
            Vec::new(),
        );
        out.closure_rounds = closure_rounds;
        out.min_fall_degree = min_fall_degree;
        for row in rows {
            let c = leading_column(&row)?;
            if out.pivot_of[c].is_some() {
                return None;
            }
            out.pivot_of[c] = Some(out.rows.len() as u32);
            out.pivot_col.push(c as u32);
            out.rows.push(LazyRow {
                version: 0,
                start: 0,
                lead: (c / 64) as u32,
                data: row.into(),
            });
        }
        out.pending = pending;
        Some(out)
    }

    /// A closure caught in the middle of a round's elimination
    /// ([`RoundView`]), rebuilt from `system` and the saved parts: a shell
    /// basis holding the layout, and the job to hand to
    /// [`ReducedBasis::close_with`] as `resume`.  `None` if the parts do not
    /// fit (a row of the wrong length, a position outside the matrix).
    #[allow(clippy::too_many_arguments)]
    pub fn from_round_state(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
        min_fall_degree: u32,
        closure_rounds: u32,
        columns: Vec<Mask>,
        was_pivot: Vec<bool>,
        matrix: Vec<Vec<u64>>,
        pivot_row: usize,
        word: usize,
    ) -> Option<(Self, RoundJob)> {
        let words = columns.len().div_ceil(64).max(1);
        if was_pivot.len() != columns.len()
            || matrix.iter().any(|r| r.len() != words)
            || pivot_row > matrix.len()
            || word > words
        {
            return None;
        }
        let system = Rc::new(system.to_vec());
        let generator_degrees = Rc::new(system.iter().map(poly_degree).collect());
        let mut out = Self::assemble(
            system,
            generator_degrees,
            n_vars,
            degree,
            support_local,
            columns,
            Vec::new(),
        );
        out.closure_rounds = closure_rounds;
        out.min_fall_degree = min_fall_degree;
        Some((
            out,
            RoundJob {
                matrix,
                was_pivot,
                resume: Some((pivot_row, word)),
            },
        ))
    }

    /// Degree falls queued for closure and not yet multiplied out: `0`
    /// after [`ReducedBasis::close`] has saturated the basis.
    pub fn pending_falls(&self) -> usize {
        self.pending.len()
    }

    /// Depth of this basis below its root: the number of variables
    /// specialised away.
    pub fn depth(&self) -> u32 {
        self.assigned.count_ones()
    }

    /// The current epoch: the number of layout steps since the root.
    fn epoch(&self) -> u32 {
        self.history.len() as u32
    }

    /// Index of the first column of degree `≤ 1` (the linear tail).
    fn low_start(&self) -> usize {
        self.columns
            .iter()
            .position(|m| m.count_ones() <= 1)
            .unwrap_or(self.columns.len())
    }

    /// The composed map from the columns of epoch `from` to those of epoch
    /// `to`.  Memoised on the step that maps into `to`, which every basis
    /// below that step shares, so a node composes at most one step on top
    /// of what its ancestors already built.  Index bookkeeping, not
    /// charged, as the from-scratch path's column indexing never was.
    fn composite_to(&self, to: u32, from: u32) -> Rc<Vec<u32>> {
        debug_assert!(from < to);
        let step = &self.history[(to - 1) as usize];
        if from + 1 == to {
            return step.map.clone();
        }
        if let Some(c) = step.to_here.borrow().get(&from) {
            return c.clone();
        }
        let prev = self.composite_to(to - 1, from);
        // `DELETED` is `u32::MAX`, out of range of every map, so `get`
        // sends it (and only it) to `DELETED` without a separate test
        let map = &step.map[..];
        // `bitop/v1`: one lookup an entry composed
        bitop_tally::word_ops(prev.len() as u64);
        // an `Rc<Vec>` takes the collected buffer as it is; an `Rc<[_]>`
        // would copy it into a fresh allocation
        let composed: Rc<Vec<u32>> = Rc::new(
            prev.iter()
                .map(|&c| map.get(c as usize).copied().unwrap_or(DELETED))
                .collect(),
        );
        step.to_here.borrow_mut().insert(from, composed.clone());
        composed
    }

    /// The composed map from the columns of epoch `from` to the current
    /// layout.
    fn composite(&self, from: u32) -> Rc<Vec<u32>> {
        self.composite_to(self.epoch(), from)
    }

    /// The eager kernel's flat plan of [`ReducedBasis::composite`]`(from)`,
    /// memoised on the step that maps into the current layout (a
    /// specialisation under the batched rewrite leaves its own plan there
    /// under `from = epoch − 1`).  Tallied as a specialisation tallies its
    /// step's plan: a column each, and a word each for its bounds.
    fn step_plan_from(&self, mode: EagerMode, from: u32) -> Rc<StepPlan> {
        let to = self.epoch();
        let step = &self.history[(to - 1) as usize];
        let own = from + 1 == to;
        if own {
            if let Some(p) = step.own_plan.get() {
                return p.clone();
            }
        } else if let Some(p) = step.step_plans.borrow().get(&from) {
            return p.clone();
        }
        let map = self.composite(from);
        bitop_tally::word_ops(2 * map.len().div_ceil(64) as u64);
        let plan = Rc::new(StepPlan::new_kept(&map, mode == EagerMode::Runs));
        if own {
            let _ = step.own_plan.set(plan.clone());
        } else {
            step.step_plans.borrow_mut().insert(from, plan.clone());
        }
        plan
    }

    /// **Batched rewrite, at a split.**  Bring every row into the current
    /// layout before the basis is shared by the two children, rows from
    /// the commonest stale epoch in one bulk pass through the composed
    /// plan ([`ReducedBasis::rewrite_kept`]), the rest one by one.  Under
    /// [`batched_rewrite`] a specialisation leaves its kept rows where they
    /// are, so every rewrite between two splits — the split variable's and
    /// each forced variable's — is done here at once, or earlier for a row
    /// that something reads.  A no-op in the lazy mode (`KIC_F4_EAGER=0`),
    /// where the switch changes nothing.
    pub fn materialise_shared(&mut self, cost: &mut InheritCost) {
        let mode = eager_mode();
        if self.refuted || mode == EagerMode::Off {
            return;
        }
        let epoch = self.epoch();
        // `bitop/v1`: a version test a row
        bitop_tally::word_ops(self.rows.len() as u64);
        // Stale epochs and their row counts; a node holds rows of a few
        // epochs at most, so a short list beats a hash table.
        let mut stale: Vec<(u32, usize)> = Vec::new();
        for row in &self.rows {
            if row.version != epoch {
                match stale.iter_mut().find(|(v, _)| *v == row.version) {
                    Some((_, n)) => *n += 1,
                    None => stale.push((row.version, 1)),
                }
            }
        }
        let Some(&(from, _)) = stale.iter().max_by_key(|&&(v, n)| (n, v)) else {
            return;
        };
        let plan = self.step_plan_from(mode, from);
        self.rewrite_kept(mode, &plan, from, cost);
    }

    /// The word-wise plan of [`ReducedBasis::composite`], memoised on the
    /// step that maps into the current layout like the map itself.
    #[allow(dead_code)]
    fn rewrite_plan(&self, from: u32) -> Rc<RewritePlan> {
        let to = self.epoch();
        let step = &self.history[(to - 1) as usize];
        if let Some(p) = step.plans.borrow().get(&from) {
            return p.clone();
        }
        let plan = Rc::new(RewritePlan::new(self.composite(from)));
        step.plans.borrow_mut().insert(from, plan.clone());
        plan
    }

    /// Rewrite `row` from the layout of epoch `version` into the current
    /// one, trimmed to the first word its image touches; `None` if every
    /// bit was deleted or the folds cancelled it.  Charged one word
    /// operation per word read (from the row's `lead`) and per word
    /// written (from the image's first word to the end of the layout).
    fn rewrite(&self, row: &LazyRow, cost: &mut InheritCost) -> Option<Draft> {
        self.rewrite_with(row, cost, |image, start| Draft {
            start,
            data: Rc::from(image),
        })
    }

    /// [`ReducedBasis::rewrite`] into a shared row, for an image that is
    /// only read afterwards: one allocation, no zero fill.
    fn rewrite_frozen(&self, row: &LazyRow, cost: &mut InheritCost) -> Option<(usize, Rc<[u64]>)> {
        self.rewrite_with(row, cost, |image, start| (start, Rc::from(image)))
    }

    /// The rewrite itself; `finish` receives the image's words from its
    /// first nonzero word `start` to the end of the layout.
    fn rewrite_with<T>(
        &self,
        row: &LazyRow,
        cost: &mut InheritCost,
        finish: impl FnOnce(&[u64], usize) -> T,
    ) -> Option<T> {
        debug_assert!(row.version < self.epoch());
        let words = self.words;
        let live = row.live();
        let lead = row.lead as usize;
        cost.specialise_word_ops += live.len() as u64;
        #[cfg(target_arch = "x86_64")]
        if wordwise_rewrite() {
            let plan = self.rewrite_plan(row.version);
            return STAGING.with(|staging| {
                let mut staging = staging.borrow_mut();
                if staging.len() < words {
                    staging.resize(words, 0);
                }
                let staging = &mut staging[..words];
                // SAFETY: `wordwise_rewrite` checked BMI2 for this process.
                let lo = unsafe { apply_plan_bmi2(&plan, live, lead, staging) };
                // `bitop/v1` besides the counted reads and writes: a
                // gather, spread and XOR a segment of each nonzero word
                bitop_tally::word_ops(
                    live.iter()
                        .enumerate()
                        .filter(|(_, &w)| w != 0)
                        .map(|(i, _)| 3 * plan.of(lead + i).len() as u64)
                        .sum::<u64>(),
                );
                if lo == usize::MAX {
                    return None;
                }
                // the zero test and the clearing of the image
                bitop_tally::word_ops(2 * (words - lo) as u64);
                cost.specialise_word_ops += (words - lo) as u64;
                let image = &mut staging[lo..];
                let nonzero = image.iter().any(|&w| w != 0);
                let out = nonzero.then(|| finish(image, lo));
                image.fill(0);
                out
            });
        }
        let map = self.composite(row.version);
        STAGING.with(|staging| {
            let mut staging = staging.borrow_mut();
            if staging.len() < words {
                staging.resize(words, 0);
            }
            let staging = &mut staging[..words];
            let mut lo = usize::MAX;
            let mut scatter = |c: u32| {
                if c != DELETED {
                    let w = c as usize / 64;
                    staging[w] ^= 1u64 << (c % 64);
                    lo = lo.min(w);
                }
            };
            // `bitop/v1` besides the counted reads and writes: a map lookup
            // and a scatter a set bit, and the zero test and clearing of
            // the image
            bitop_tally::word_ops(2 * live.iter().map(|w| u64::from(w.count_ones())).sum::<u64>());
            for (i, &word) in live.iter().enumerate() {
                let base = (lead + i) * 64;
                let mut bits = word;
                // A whole word of the map at once, so the lookups below
                // need no bounds check; only a layout's last word is short.
                if let Some(chunk) = map.get(base..base + 64) {
                    let chunk: &[u32; 64] = chunk.try_into().expect("64 entries");
                    while bits != 0 {
                        let b = bits.trailing_zeros() as usize & 63;
                        bits &= bits - 1;
                        scatter(chunk[b]);
                    }
                } else {
                    while bits != 0 {
                        let b = bits.trailing_zeros() as usize;
                        bits &= bits - 1;
                        scatter(map[base + b]);
                    }
                }
            }
            if lo == usize::MAX {
                return None;
            }
            bitop_tally::word_ops(2 * (words - lo) as u64);
            cost.specialise_word_ops += (words - lo) as u64;
            // staging is zero below `lo`, so its tail is the image
            let image = &mut staging[lo..];
            let nonzero = image.iter().any(|&w| w != 0);
            let out = nonzero.then(|| finish(image, lo));
            image.fill(0);
            out
        })
    }

    /// Rewrite every kept row that was current in the layout of epoch
    /// `from` into this one through `plan` (the map from `from` to the
    /// current epoch: the step's own plan, `from = epoch − 1`, when a
    /// specialisation calls it), all into one shared buffer, each trimmed
    /// to the plan's bound on its first word.  A row from any other older
    /// layout (none at a specialisation, unless a lazy ancestor left one)
    /// is materialised as before.  Charged as [`ReducedBasis::rewrite`]
    /// charges: the words read, and the words from the image's first
    /// nonzero one to the end.
    fn rewrite_kept(
        &mut self,
        mode: EagerMode,
        plan: &StepPlan,
        from: u32,
        cost: &mut InheritCost,
    ) {
        let to = self.epoch();
        debug_assert!(from < to);
        let words = self.words;
        let mut spans: Vec<(usize, usize)> = Vec::with_capacity(self.rows.len());
        let mut total = 0usize;
        for row in &self.rows {
            if row.version == from {
                let lo = plan.low[row.lead as usize] as usize;
                debug_assert!(lo < words, "a kept pivot survives");
                spans.push((total, lo));
                total += words - lo;
            } else {
                spans.push((usize::MAX, 0));
            }
        }
        let mut buf = zeroed_words(total);
        let bulk_before = cost.specialise_word_ops;
        {
            let data = Rc::get_mut(&mut buf).expect("just allocated");
            let fused = next_kernels();
            for (row, &(off, lo)) in self.rows.iter().zip(&spans) {
                if off != usize::MAX && fused {
                    // As below, with the tally counted in the rewrite pass.
                    let live = row.live();
                    cost.specialise_word_ops += live.len() as u64;
                    let out = &mut data[off..off + words - lo];
                    let segments = apply_step_counted(mode, plan, live, row.lead as usize, out, lo);
                    bitop_tally::word_ops(3 * segments);
                } else if off != usize::MAX {
                    let live = row.live();
                    cost.specialise_word_ops += live.len() as u64;
                    // `bitop/v1`: a gather, spread and XOR a segment of
                    // each nonzero word
                    let lead = row.lead as usize;
                    bitop_tally::word_ops(
                        live.iter()
                            .enumerate()
                            .filter(|(_, &w)| w != 0)
                            .map(|(i, _)| {
                                3 * (plan.first[lead + i + 1] - plan.first[lead + i]) as u64
                            })
                            .sum::<u64>(),
                    );
                    let out = &mut data[off..off + words - lo];
                    apply_step(mode, plan, live, row.lead as usize, out, lo);
                }
            }
        }
        let mut bulk_words = cost.specialise_word_ops - bulk_before;
        let mut bulk_rows = 0u64;
        for (r, &(off, lo)) in spans.iter().enumerate() {
            if off == usize::MAX {
                self.ensure_current(r, cost);
                continue;
            }
            let len = words - lo;
            let first = buf[off..off + len]
                .iter()
                .position(|&w| w != 0)
                .expect("a kept pivot survives");
            bitop_tally::word_ops(first as u64 + 1);
            cost.specialise_word_ops += (len - first) as u64;
            bulk_words += (len - first) as u64;
            bulk_rows += 1;
            self.rows[r] = LazyRow {
                version: to,
                start: lo as u32,
                lead: (lo + first) as u32,
                data: RowWords::range(buf.clone(), off, len),
            };
        }
        rewrite_tally::add(rewrite_tally::BULK, bulk_rows, bulk_words);
    }

    /// [`ReducedBasis::rewrite`] of a row current in the parent's layout,
    /// through the step's plan.
    fn rewrite_step(
        &self,
        mode: EagerMode,
        plan: &StepPlan,
        row: &LazyRow,
        cost: &mut InheritCost,
    ) -> Option<Draft> {
        debug_assert_eq!(row.version + 1, self.epoch());
        self.rewrite_through(mode, plan, row, cost)
            .map(|(lo, _, data)| Draft { start: lo, data })
    }

    /// [`ReducedBasis::rewrite_step`] through any plan whose map runs from
    /// the row's own epoch to the current one: the image's words from `lo`
    /// (the plan's bound) to the end, with the offset of its first nonzero
    /// word; `None` if the image is zero.  Charged as the eager rewrite
    /// charges: the words read, and the words from the image's first
    /// nonzero one to the end.
    fn rewrite_through(
        &self,
        mode: EagerMode,
        plan: &StepPlan,
        row: &LazyRow,
        cost: &mut InheritCost,
    ) -> Option<(usize, usize, Rc<[u64]>)> {
        let lo = plan.low[row.lead as usize] as usize;
        if lo >= self.words {
            cost.specialise_word_ops += row.live().len() as u64;
            return None;
        }
        let mut data = zeroed_words(self.words - lo);
        let out = Rc::get_mut(&mut data).expect("just allocated");
        let first = self.rewrite_into(mode, plan, row, cost, out, lo)?;
        Some((lo, first, data))
    }

    /// The body of [`ReducedBasis::rewrite_through`]: rewrite `row` into
    /// `out`, zeroed, holding words `lo ..` (`lo` the plan's bound) of the
    /// current layout; the offset of the image's first nonzero word, `None`
    /// if it is zero.  Charges as `rewrite_through` does, the zeroed
    /// buffer's tally apart.
    fn rewrite_into(
        &self,
        mode: EagerMode,
        plan: &StepPlan,
        row: &LazyRow,
        cost: &mut InheritCost,
        out: &mut [u64],
        lo: usize,
    ) -> Option<usize> {
        debug_assert!(row.version < self.epoch());
        let words = self.words;
        let live = row.live();
        cost.specialise_word_ops += live.len() as u64;
        let lead = row.lead as usize;
        let segments = if next_kernels() {
            apply_step_counted(mode, plan, live, lead, out, lo)
        } else {
            apply_step(mode, plan, live, lead, out, lo);
            live.iter()
                .enumerate()
                .filter(|(_, &w)| w != 0)
                .map(|(i, _)| (plan.first[lead + i + 1] - plan.first[lead + i]) as u64)
                .sum::<u64>()
        };
        let first_pos = out.iter().position(|&w| w != 0);
        bitop_tally::word_ops(3 * segments + first_pos.map_or(out.len() as u64, |f| f as u64 + 1));
        let first = first_pos?;
        cost.specialise_word_ops += (words - lo - first) as u64;
        Some(first)
    }

    /// Bring row `r` into the current layout if it is not there yet.  A
    /// kept row's leading monomial survives every step, so its image is
    /// nonzero.
    #[inline]
    fn ensure_current(&mut self, r: usize, cost: &mut InheritCost) {
        // Most calls find the row current (a pivot hit twice at one level),
        // so only the rewrite stays out of line.
        if self.rows[r].version != self.epoch() {
            self.materialise(r, cost);
        }
    }

    /// Rewrite row `r` into the current layout.
    #[inline(never)]
    fn materialise(&mut self, r: usize, cost: &mut InheritCost) {
        let before = cost.specialise_word_ops;
        self.materialise_row(r, cost);
        rewrite_tally::add(rewrite_tally::SINGLE, 1, cost.specialise_word_ops - before);
    }

    /// [`ReducedBasis::materialise`] itself.
    #[inline(always)]
    fn materialise_row(&mut self, r: usize, cost: &mut InheritCost) {
        if batched_rewrite() {
            let mode = eager_mode();
            if mode != EagerMode::Off {
                // The eager kernel and its charge, through the composed
                // map from the row's own epoch: one pass however many
                // layout steps it skipped.
                let plan = self.step_plan_from(mode, self.rows[r].version);
                let row = &self.rows[r];
                let lo = plan.low[row.lead as usize] as usize;
                assert!(lo < self.words, "a kept pivot survives");
                let n = self.words - lo;
                // `bitop/v1`: the zeroed span, as `zeroed_words` tallies it
                bitop_tally::word_ops(n as u64);
                let (first, data) =
                    RowArena::span(n, |out| self.rewrite_into(mode, &plan, row, cost, out, lo))
                        .expect("a kept pivot survives");
                self.rows[r] = LazyRow {
                    version: self.epoch(),
                    start: lo as u32,
                    lead: (lo + first) as u32,
                    data,
                };
                return;
            }
        }
        let (start, data) = self
            .rewrite_frozen(&self.rows[r], cost)
            .expect("a kept pivot survives");
        let lead = start + data.iter().position(|&w| w != 0).expect("nonzero");
        self.rows[r] = LazyRow {
            version: self.epoch(),
            start: start as u32,
            lead: lead as u32,
            data: data.into(),
        };
    }

    /// Bring every row into the current layout.
    pub fn materialise_all(&mut self, cost: &mut InheritCost) {
        for r in 0..self.rows.len() {
            self.ensure_current(r, cost);
        }
    }

    /// Row `r` in the current layout, materialising it if needed.
    fn current_row(&mut self, r: usize, cost: &mut InheritCost) -> LazyRow {
        self.ensure_current(r, cost);
        self.rows[r].clone()
    }

    /// Restore reduced row echelon form: clear every pivot column from
    /// every row but its own.
    ///
    /// One pass in **decreasing** pivot-column order suffices.  A row's
    /// bits all sit at or after its leading column, so when pivot column
    /// `c` is processed every pivot column beyond `c` has already been
    /// cleared from its row, and XORing it into another row introduces no
    /// pivot-column bit.  Only XORs are charged, as in the from-scratch
    /// kernels; the pivot-column tests are not.
    ///
    /// Why it is worth paying: after a specialisation the basis is in
    /// echelon form but not reduced — a fold `m ∋ v ↦ m ∖ v` can land on
    /// another row's pivot column, and a freshly inserted pivot has never
    /// been cleared from the rows above it.  A displaced row reduced
    /// against such a basis picks up those bits and cascades; against a
    /// reduced basis it clears one pivot column per XOR and stops.
    pub fn reduce_fully(&mut self, cost: &mut InheritCost) {
        if self.refuted {
            return;
        }
        self.materialise_all(cost);
        let words = self.words;
        let epoch = self.epoch();
        for c in (0..self.pivot_of.len()).rev() {
            let Some(r) = self.pivot_of[c] else {
                continue;
            };
            let r = r as usize;
            let (w, bit) = (c / 64, 1u64 << (c % 64));
            let pivot = self.rows[r].clone();
            for (i, row) in self.rows.iter_mut().enumerate() {
                if i != r && row.word(w) & bit != 0 {
                    let mut data = row.data.to_vec();
                    let at = w - row.start as usize;
                    for (dst, &src) in data[at..].iter_mut().zip(pivot.from_word(w)) {
                        *dst ^= src;
                    }
                    *row = LazyRow {
                        version: epoch,
                        start: row.start,
                        lead: row.lead,
                        data: data.into(),
                    };
                    cost.reduce_word_ops += (words - w) as u64;
                    cost.rref_word_ops += (words - w) as u64;
                }
            }
        }
    }

    /// Should a basis at `depth` be restored to reduced form after its
    /// specialisation?  `KIC_F4_INHERIT_RREF` names the policy: `0` never
    /// (the default), `k ≥ 1` every `k` levels.
    ///
    /// A retained negative control.  Restoring reduced form does cut the
    /// cascade — on `K_1/2^23` a displaced row then hits `9.8` pivots
    /// instead of `40.5` — but the pass itself costs `72 M` word XORs
    /// against the `49 M` it saves, because a reduced basis is *denser*
    /// than an echelon one and every specialisation refills some twenty
    /// pivot columns per row (`RESEARCH_INHERITED_F4.md` §3.5).
    fn rref_every() -> u32 {
        static EVERY: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
        *EVERY.get_or_init(|| {
            std::env::var("KIC_F4_INHERIT_RREF")
                .ok()
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(0)
        })
    }

    /// Insert a row given in the current layout, reducing its leading term
    /// against the existing pivots until it becomes a new pivot or vanishes.
    /// Every pivot it hits is materialised first.
    fn insert(&mut self, mut row: Draft, cost: &mut InheritCost) -> Option<usize> {
        let mut from = row.start;
        let simd = next_kernels();
        loop {
            let c = row.leading_column_from(from)?;
            let w = c / 64;
            // `bitop/v1`: the pivot lookup and the layout check of the pivot
            bitop_tally::word_ops(2);
            match self.pivot_of[c] {
                Some(r) => {
                    // materialise, then borrow: no row handle to clone
                    self.ensure_current(r as usize, cost);
                    let pivot = &self.rows[r as usize];
                    let at = w - row.start;
                    if simd {
                        xor_words_simd(&mut row.data_mut()[at..], pivot.from_word(w));
                    } else {
                        xor_words(&mut row.data_mut()[at..], pivot.from_word(w));
                    }
                    cost.reduce_word_ops += (self.words - w) as u64;
                    from = w;
                }
                None => {
                    self.pivot_of[c] = Some(self.rows.len() as u32);
                    self.pivot_col.push(c as u32);
                    self.rows.push(LazyRow {
                        version: self.epoch(),
                        start: row.start as u32,
                        lead: w as u32,
                        data: row.data.into(),
                    });
                    return Some(self.rows.len() - 1);
                }
            }
        }
    }

    /// Did the row just inserted land on the constant `1`?  If so, collapse
    /// the basis to it.
    fn refutes(&mut self, inserted: Option<usize>) -> bool {
        match inserted {
            Some(r) if self.columns[self.pivot_col[r] as usize] == 0 => {
                self.collapse_to_one(r);
                true
            }
            _ => false,
        }
    }

    /// Adopt a new column layout: record the step, remap every pivot
    /// column and rebuild the pivot index.  Rows are left where they are.
    fn adopt_layout(&mut self, columns: Vec<Mask>, map: Vec<u32>) {
        let step = LayoutStep {
            map: Rc::new(map),
            to_here: RefCell::new(HashMap::default()),
            plans: RefCell::new(HashMap::default()),
            step_plans: RefCell::new(HashMap::default()),
            own_plan: std::cell::OnceCell::new(),
        };
        self.column_index = None;
        self.words = columns.len().div_ceil(64).max(1);
        // `bitop/v1`: the cleared pivot index and the remap of every pivot
        bitop_tally::word_ops((columns.len() + 2 * self.pivot_col.len()) as u64);
        self.pivot_of = vec![None; columns.len()];
        for (r, pc) in self.pivot_col.iter_mut().enumerate() {
            let c = step.map[*pc as usize];
            debug_assert!(c != DELETED, "a kept pivot column survives the step");
            debug_assert!(self.pivot_of[c as usize].is_none(), "row {r} collides");
            self.pivot_of[c as usize] = Some(r as u32);
            *pc = c;
        }
        self.columns = columns;
        self.history.push(Rc::new(step));
    }

    /// The basis of `system|_{var = value}`: the kept rows shared by
    /// reference, the displaced ones materialised and re-reduced, plus
    /// completion rows for any generator whose degree dropped.
    ///
    /// **Completion.**  The Macaulay matrix multiplies `f_i` by every
    /// monomial of degree `≤ D − deg f_i`.  If `deg f_i|_{v=c} < deg f_i`,
    /// the specialised system has multipliers of higher degree that no
    /// parent row maps to; those products are built and inserted, with
    /// multipliers over the variables occurring in the specialised system.
    ///
    /// **Which row space, exactly.**  Write `V_M(S)` for the Macaulay row
    /// space of `S` with multipliers over the variable set `M`.  The
    /// inherited rows span `V_{unassigned}` for the generators whose degree
    /// did not drop and the completion adds `V_{occurring}` for the rest,
    /// so the basis lies between `V_{occurring}(S')` and `V_{all}(S')`, the
    /// latter being what the from-scratch step reduces.  Every space in
    /// that sandwich has the **same linear tail**: a row `x·g` with `x`
    /// occurring in no generator lies in the tail only if `g ∈ {0, 1}`,
    /// and `g = 1` is already a refutation.  The solver reads nothing but
    /// the tail, so it behaves identically.
    pub fn specialise(&self, var: u32, value: bool) -> (Self, InheritCost) {
        let substituted: Vec<F2BoolPoly> = self
            .system
            .iter()
            .map(|p| substitute(p, var, value))
            .collect();
        let child = ChildSystem::new(&self.generator_degrees, &substituted);
        self.specialise_shared(var, value, &child)
    }

    /// The generator degrees this basis was completed for, aligned with
    /// [`ReducedBasis::system`].
    pub fn generator_degrees(&self) -> &[u32] {
        &self.generator_degrees
    }

    /// [`ReducedBasis::specialise`] with the child's system already
    /// computed — by the solver, which substitutes it anyway, once for
    /// every basis at the node.  `child` must be the image of this basis's
    /// own system under `var := value`.
    ///
    /// **Refutation.**  Once a re-reduced or completion row lands on the
    /// constant `1`, the child's tail is a refutation whatever the rows not
    /// yet inserted would add, so they are not inserted: the child is
    /// collapsed to `1` and returned.  Its row space is then no longer the
    /// sandwich above, but the solver's view of it is — every space in the
    /// sandwich contains `1`, the solver refutes on `1` before reading
    /// anything else, and a specialisation of `1` is `1`, so a refuted
    /// basis stays refuted, at no cost, through whatever the solver still
    /// assigns before it reads this degree.
    pub fn specialise_shared(
        &self,
        var: u32,
        value: bool,
        child: &ChildSystem,
    ) -> (Self, InheritCost) {
        let mut cost = InheritCost::default();
        let bit = bit(var);
        if self.refuted {
            let mut out = self.clone();
            out.system = child.system.clone();
            out.generator_degrees = child.degrees.clone();
            out.assigned |= bit;
            return (out, cost);
        }
        let dropped: Vec<(usize, u32, u32)> = child
            .dropped
            .iter()
            .copied()
            .filter(|&(_, old_degree, _)| old_degree <= self.degree)
            .collect();

        // New column layout: the images of the old columns.  Deleting the
        // columns that contain `v` keeps the rest in order; folding
        // `m ∋ v ↦ m ∖ v` keeps the folded ones in order among themselves
        // (every degree drops by one, and the symmetric difference of two
        // of them is unchanged), so the new layout is a merge of two sorted
        // lists and the old → new map falls out of the merge.
        let n_old = self.columns.len();
        // `bitop/v1`: the map (two `u32` entries a word); the split of the
        // rows into kept and displaced, a test and a move each; the step's
        // plan, a column each and a word each for its bounds
        bitop_tally::word_ops(
            (n_old as u64).div_ceil(2)
                + 2 * self.rows.len() as u64
                + self.history.len() as u64
                + if eager_mode() != EagerMode::Off {
                    2 * n_old.div_ceil(64) as u64
                } else {
                    0
                },
        );
        let mut map: Vec<u32> = vec![DELETED; n_old];
        let mut new_columns: Vec<Mask> = Vec::with_capacity(n_old);
        if value {
            fold_layout(&self.columns, bit, &mut new_columns, &mut map);
        } else {
            bitop_tally::word_ops(2 * n_old as u64);
            for (k, &m) in self.columns.iter().enumerate() {
                if m & bit == 0 {
                    map[k] = new_columns.len() as u32;
                    new_columns.push(m);
                }
            }
        }
        debug_assert!(new_columns.windows(2).all(|w| {
            cmp_mono(F2BoolMono::from_mask(w[0]), F2BoolMono::from_mask(w[1]))
                == std::cmp::Ordering::Greater
        }));

        // The child starts as a copy of the parent's bookkeeping — rows by
        // reference — minus the displaced rows, then adopts the new layout.
        let mut out = Self {
            degree: self.degree,
            n_vars: self.n_vars,
            system: child.system.clone(),
            generator_degrees: child.degrees.clone(),
            assigned: self.assigned | bit,
            // Set by `adopt_layout` below; the split reads the parent's.
            columns: Vec::new(),
            column_index: None,
            words: self.words,
            history: self.history.clone(),
            rows: Vec::with_capacity(self.rows.len()),
            pivot_col: Vec::with_capacity(self.rows.len()),
            pivot_of: Vec::new(),
            closure_rounds: self.closure_rounds,
            min_fall_degree: self.min_fall_degree,
            pending: Vec::new(),
            refuted: false,
            support_local: self.support_local,
        };
        // Kept rows are renumbered; a pending fall that is kept stays pending.
        let mut is_pending = Vec::new();
        if !self.pending.is_empty() {
            is_pending = vec![false; self.rows.len()];
            for &r in &self.pending {
                is_pending[r as usize] = true;
            }
        }
        let mut displaced: Vec<(u32, LazyRow)> = Vec::new();
        for (r, (row, &pc)) in self.rows.iter().zip(&self.pivot_col).enumerate() {
            if self.columns[pc as usize] & bit == 0 {
                // Leading monomial survives and stays distinct; the row's
                // content is not needed here.
                if is_pending.get(r) == Some(&true) {
                    out.pending.push(out.rows.len() as u32);
                }
                out.rows.push(row.clone());
                out.pivot_col.push(pc);
            } else {
                displaced.push((pc, row.clone()));
            }
        }
        let mode = eager_mode();
        let batched = mode != EagerMode::Off && batched_rewrite();
        let step = (mode != EagerMode::Off).then(|| {
            Rc::new(if batched {
                StepPlan::new_kept(&map, mode == EagerMode::Runs)
            } else {
                StepPlan::new(&map, mode == EagerMode::Runs)
            })
        });
        out.adopt_layout(new_columns, map);
        if let Some(step) = &step {
            if batched {
                // Batched rewrite: the kept rows stay in the parent's
                // layout until something reads them or the node splits
                // ([`ReducedBasis::materialise_shared`]); the plan is kept
                // for those rewrites and for the displaced rows below.
                let from = out.epoch() - 1;
                let _ = out.history[from as usize].own_plan.set(step.clone());
            } else {
                out.rewrite_kept(mode, step, out.epoch() - 1, &mut cost);
            }
        }
        cost.displaced_rows = displaced.len() as u64;
        // Lowest pivot first.  The order changes neither the child's row
        // space nor, measurably, the cost of building it; it changes how
        // soon a refutation is found.  A row pivoting near the tail has
        // its image there too, next to the constant `1`, and inserted
        // first those rows reach `1` before the ones that re-reduce through
        // the whole basis are touched (`RESEARCH_INHERITED_F4.md` §3.7).
        bitop_tally::word_ops(bitop_tally::sort_cost(displaced.len()));
        displaced.sort_unstable_by_key(|&(pc, _)| std::cmp::Reverse(pc));
        for (_, row) in displaced {
            let before = cost.specialise_word_ops;
            let image = match &step {
                Some(step) if row.version + 1 == out.epoch() => {
                    out.rewrite_step(mode, step, &row, &mut cost)
                }
                // A row the batched rewrite left stale: the eager kernel
                // through the composed map, in one pass.
                Some(_) if batched => {
                    let plan = out.step_plan_from(mode, row.version);
                    out.rewrite_through(mode, &plan, &row, &mut cost)
                        .map(|(lo, _, data)| Draft { start: lo, data })
                }
                _ => out.rewrite(&row, &mut cost),
            };
            rewrite_tally::add(
                rewrite_tally::DISPLACED,
                1,
                cost.specialise_word_ops - before,
            );
            if let Some(image) = image {
                let inserted = out.insert(image, &mut cost);
                if out.refutes(inserted) {
                    return (out, cost);
                }
                out.note_fall(inserted);
            }
        }

        // Completion rows for generators whose degree dropped, with
        // multipliers over the variables occurring in the specialised
        // system (the from-scratch step's own active-multiplier policy):
        // a multiplier containing a variable that occurs nowhere adds only
        // rows `x·g` that cannot reach the linear tail.  A support-local
        // basis multiplies each dropped generator only within its own
        // support, as its root did.
        if !dropped.is_empty() {
            bitop_tally::word_ops(out.system.iter().map(|p| p.terms.len() as u64).sum());
            let multiplier_mask = out
                .system
                .iter()
                .flat_map(|p| p.terms.iter())
                .fold(0 as Mask, |acc, t| acc | t.mask)
                & all_variable_mask(out.n_vars);
            let mut batch = CompletionBatch::default();
            {
                out.column_index();
                let index = out.column_index.as_ref().expect("just built");
                for &(i, old_degree, new_degree) in &dropped {
                    let p = &out.system[i];
                    let old_gap = self.degree - old_degree;
                    let new_gap = self.degree - new_degree;
                    let mask = if self.support_local {
                        multiplier_mask & p.terms.iter().fold(0 as Mask, |acc, t| acc | t.mask)
                    } else {
                        multiplier_mask
                    };
                    for t in monomials_up_to_mask(mask, new_gap) {
                        if t.count_ones() <= old_gap {
                            continue;
                        }
                        batch.push_row(p.terms.iter().map(|m| m.mask | t), index);
                    }
                }
            }
            if !batch.rows.is_empty() {
                let columns = batch.place(&mut out);
                cost.completion_rows = batch.rows.len() as u64;
                note_rows_generated(
                    batch.rows.len() as u64,
                    (batch.rows.len() * out.words) as u64,
                );
                for ids in &batch.rows {
                    // `bitop/v1`: the zeroed row, a lookup and an OR an id
                    bitop_tally::word_ops((out.words + 2 * ids.len()) as u64);
                    let mut row = vec![0u64; out.words];
                    for &id in ids {
                        let c = columns[id as usize] as usize;
                        row[c / 64] |= 1u64 << (c % 64);
                    }
                    let inserted = out.insert(Draft::full(row), &mut cost);
                    if out.refutes(inserted) {
                        return (out, cost);
                    }
                    out.note_fall(inserted);
                }
            }
        }
        if Self::close_children() {
            out.close(&mut cost);
        } else {
            out.pending.clear();
        }
        let every = Self::rref_every();
        if every > 0 && out.depth().is_multiple_of(every) {
            out.reduce_fully(&mut cost);
        }
        (out, cost)
    }

    /// Does a closed basis keep closing below its root?
    /// `KIC_F4_CLOSURE_CHILDREN=0` confines the closure to the bases built
    /// from scratch; any other value, or none, closes every specialisation
    /// too.  Irrelevant unless `KIC_F4_CLOSURE_ROUNDS` is positive.
    fn close_children() -> bool {
        static CHILDREN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *CHILDREN.get_or_init(|| std::env::var("KIC_F4_CLOSURE_CHILDREN").as_deref() != Ok("0"))
    }

    /// Is row `r` a degree fall this basis closes — leading degree at
    /// least 2 and below the Macaulay degree?
    fn is_fall(&self, r: usize) -> bool {
        if self.closure_rounds == 0 {
            return false;
        }
        let d = self.columns[self.pivot_col[r] as usize].count_ones();
        d >= self.min_fall_degree && d < self.degree
    }

    /// Queue a freshly inserted row for closure if it is a degree fall.
    fn note_fall(&mut self, inserted: Option<usize>) {
        if let Some(r) = inserted {
            if self.is_fall(r) {
                self.pending.push(r as u32);
            }
        }
    }

    /// Multiply the pending degree falls by every monomial `t` that keeps
    /// the degree within the basis's (`1 ≤ deg t ≤ D − deg q`, over the
    /// variables occurring in the system) and insert the products; falls
    /// the products produce are queued in turn.  At most `closure_rounds`
    /// rounds run per node; what is left stays pending for the children.
    ///
    /// This is MutantXL's step at fixed degree: a degree fall `q` of the
    /// degree-`D` Macaulay row space has multiples `t·q` of degree `≤ D`
    /// that the Macaulay matrix does not contain, and on the Semaev systems
    /// here they are what lets a degree-3 basis refute or pin a branch
    /// that the plain degree-3 matrix has to split.  Every product lies in
    /// the ideal, so the tail stays sound.  Each product is charged one
    /// word read per word of `q` and one written per word of the product.
    pub fn close(&mut self, cost: &mut InheritCost) {
        self.close_with(cost, 0, None, &mut NoSink);
    }

    /// [`ReducedBasis::close`] from round `first_round`, offering `sink` its
    /// resumable states: between rounds (when the closure has more to do) and
    /// at each word boundary of a round's elimination.  `resume` continues a
    /// round that a stop left in its elimination
    /// ([`ReducedBasis::from_round_state`]).  A sink answering
    /// [`CloseFlow::Stop`] ends the call there, the basis left mid-round and
    /// not to be used again.  With a sink that always continues this is
    /// `close`, round for round, charging the same work.
    pub fn close_with(
        &mut self,
        cost: &mut InheritCost,
        first_round: u32,
        mut resume: Option<RoundJob>,
        sink: &mut dyn ClosureSink,
    ) -> CloseEnd {
        if self.closure_rounds == 0 || self.refuted {
            self.pending.clear();
            return CloseEnd {
                rounds: first_round,
                stopped: false,
            };
        }
        let mut done = first_round;
        let multipliers = self
            .system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0 as Mask, |acc, t| acc | t.mask)
            & all_variable_mask(self.n_vars);
        for _ in first_round..self.closure_rounds {
            let mut job = match resume.take() {
                Some(job) => job,
                None => {
                    let work = std::mem::take(&mut self.pending);
                    if work.is_empty() {
                        break;
                    }
                    let mut products: Vec<Vec<Mask>> = Vec::new();
                    let mut read_words = 0u64;
                    for r in work {
                        let r = r as usize;
                        if !self.is_fall(r) {
                            continue;
                        }
                        let gap =
                            self.degree - self.columns[self.pivot_col[r] as usize].count_ones();
                        let q = self.current_row(r, cost);
                        bitop_tally::word_ops(
                            q.live().iter().map(|w| 1 + u64::from(w.count_ones())).sum(),
                        );
                        let mut q_monos: Vec<Mask> = Vec::new();
                        for (i, &word) in q.live().iter().enumerate() {
                            let base = (q.lead as usize + i) * 64;
                            let mut bits = word;
                            while bits != 0 {
                                let b = bits.trailing_zeros() as usize;
                                bits &= bits - 1;
                                q_monos.push(self.columns[base + b]);
                            }
                        }
                        for t in monomials_up_to_mask(multipliers, gap) {
                            if t == 0 {
                                continue;
                            }
                            let product = odd_terms(q_monos.iter().map(|&m| m | t));
                            if !product.is_empty() {
                                products.push(product);
                                read_words += q.live().len() as u64;
                            }
                        }
                    }
                    if products.is_empty() {
                        break;
                    }
                    self.extend_columns(&products);
                    let words = self.words;
                    cost.specialise_word_ops += read_words + (products.len() * words) as u64;
                    cost.closure_products += products.len() as u64;
                    bitop_tally::hash_probes(products.iter().map(|p| p.len() as u64).sum());
                    bitop_tally::word_ops(products.iter().map(|p| (words + p.len()) as u64).sum());
                    let packed: Vec<Vec<u64>> = {
                        let index = self.column_index();
                        products
                            .iter()
                            .map(|monos| {
                                let mut row = vec![0u64; words];
                                for m in monos {
                                    let c = index[m];
                                    row[c / 64] |= 1u64 << (c % 64);
                                }
                                row
                            })
                            .collect()
                    };
                    match self.absorb_begin(packed, cost) {
                        Some(job) => job,
                        None => {
                            done += 1;
                            continue;
                        }
                    }
                }
            };
            let Some(rank) = self.absorb_eliminate(&mut job, cost, done, sink) else {
                return CloseEnd {
                    rounds: done,
                    stopped: true,
                };
            };
            self.absorb_finish(job, rank);
            done += 1;
            if self.refuted {
                break;
            }
            if !self.pending.is_empty() && sink.after_round(self, cost, done) == CloseFlow::Stop {
                return CloseEnd {
                    rounds: done,
                    stopped: true,
                };
            }
        }
        CloseEnd {
            rounds: done,
            stopped: false,
        }
    }

    /// Absorb a batch of rows given in the current layout: one elimination
    /// of the whole basis with the batch below it, through the shape-selected
    /// kernel ([`rref_f2_counted`]), leaving the basis in reduced row echelon
    /// form.  Inserting the rows one at a time into an echelon basis would
    /// cascade through its pivots; a batch of closure products is large
    /// enough that the Four Russians kernel reduces it for a fraction of
    /// that.  Rows whose pivot column is new and whose leading degree is a
    /// fall's are queued for closure.
    ///
    /// Three steps so that a closure can stop inside the elimination:
    /// [`absorb_begin`](Self::absorb_begin) builds the matrix,
    /// [`absorb_eliminate`](Self::absorb_eliminate) reduces it (resumably),
    /// [`absorb_finish`](Self::absorb_finish) rebuilds the basis from it.
    fn absorb_begin(&mut self, batch: Vec<Vec<u64>>, cost: &mut InheritCost) -> Option<RoundJob> {
        if batch.is_empty() {
            return None;
        }
        self.materialise_all(cost);
        let was_pivot: Vec<bool> = self.pivot_of.iter().map(Option::is_some).collect();
        let words = self.words;
        // `bitop/v1`: the pivot flags, the widened copy of every row, and
        // after the elimination the cleared index and each lead scan
        bitop_tally::word_ops(
            2 * self.pivot_of.len() as u64
                + (self.rows.len() + batch.len()) as u64 * 2 * words as u64,
        );
        let mut matrix: Vec<Vec<u64>> = self
            .rows
            .iter()
            .map(|row| {
                let mut full = vec![0u64; words];
                full[row.start as usize..].copy_from_slice(&row.data);
                full
            })
            .collect();
        matrix.extend(batch);
        Some(RoundJob {
            matrix,
            was_pivot,
            resume: None,
        })
    }

    /// Reduce the round's matrix to reduced row echelon form, offering the
    /// sink every word boundary when the Four Russians kernel runs.  Returns
    /// the rank, or `None` if the sink stopped it (the job then holds the
    /// partly reduced matrix and where it stopped).
    fn absorb_eliminate(
        &mut self,
        job: &mut RoundJob,
        cost: &mut InheritCost,
        done: u32,
        sink: &mut dyn ClosureSink,
    ) -> Option<usize> {
        let n_cols = self.columns.len();
        if !rref_uses_four_russians(job.matrix.len(), n_cols) {
            // Small matrices take the other kernels and finish in moments.
            return Some(rref_f2_counted(
                &mut job.matrix,
                n_cols,
                &mut cost.reduce_word_ops,
            ));
        }
        let (columns, was_pivot) = (&self.columns, &job.was_pivot);
        let (degree, n_vars, support_local) = (self.degree, self.n_vars, self.support_local);
        let (min_fall_degree, closure_rounds) = (self.min_fall_degree, self.closure_rounds);
        let base = *cost;
        let mut ops = cost.reduce_word_ops;
        let mut position = job.resume;
        let end = {
            let mut hook = |matrix: &[Vec<u64>], pivot_row: usize, word: usize, ops_now: u64| {
                let mut at = base;
                at.reduce_word_ops = ops_now;
                let view = RoundView {
                    degree,
                    n_vars,
                    support_local,
                    min_fall_degree,
                    closure_rounds,
                    columns,
                    was_pivot,
                    matrix,
                    pivot_row,
                    word,
                };
                sink.in_round(&view, &at, done) == CloseFlow::Stop
            };
            crate::cryptanalysis::gf2_elim::rref_resumable(
                &mut job.matrix,
                n_cols,
                &mut ops,
                position.take(),
                Some(&mut hook),
            )
        };
        cost.reduce_word_ops = ops;
        match end {
            crate::cryptanalysis::gf2_elim::ElimEnd::Done(rank) => Some(rank),
            crate::cryptanalysis::gf2_elim::ElimEnd::Stopped { pivot_row, word } => {
                job.resume = Some((pivot_row, word));
                None
            }
        }
    }

    fn absorb_finish(&mut self, job: RoundJob, rank: usize) {
        let RoundJob {
            mut matrix,
            was_pivot,
            ..
        } = job;
        matrix.truncate(rank);
        let epoch = self.epoch();
        self.rows.clear();
        self.pivot_col.clear();
        self.pivot_of = vec![None; self.columns.len()];
        self.pending.clear();
        for (r, row) in matrix.into_iter().enumerate() {
            let c = leading_column(&row).expect("rank rows are nonzero");
            self.pivot_of[c] = Some(r as u32);
            self.pivot_col.push(c as u32);
            self.rows.push(LazyRow {
                version: epoch,
                start: 0,
                lead: (c / 64) as u32,
                data: row.into(),
            });
            if !was_pivot[c] && self.is_fall(r) {
                self.pending.push(r as u32);
            }
        }
        if let Some(r) = self.one_row() {
            self.collapse_to_one(r);
        }
    }

    /// Add any monomials of `rows_monos` missing from the layout, keeping
    /// the descending column order.  A layout step like any other: rows
    /// are not touched.
    fn extend_columns(&mut self, rows_monos: &[Vec<Mask>]) {
        let index = self.column_index();
        // The products share most of their new monomials, so collect each
        // distinct one once as it is first met and key only those into the
        // layout's order.
        let mut seen: MaskMap<()> = MaskMap::default();
        let mut missing: Vec<Mask> = Vec::new();
        // `bitop/v1`: a layout probe a monomial and a set probe each absent one
        bitop_tally::hash_probes(2 * rows_monos.iter().map(|r| r.len() as u64).sum::<u64>());
        for &m in rows_monos.iter().flatten() {
            if !index.contains_key(&m) && seen.insert(m, ()).is_none() {
                missing.push(m);
            }
        }
        if missing.is_empty() {
            return;
        }
        bitop_tally::word_ops(bitop_tally::sort_cost(missing.len()));
        sort_masks_descending(&mut missing);
        self.insert_columns(&missing);
    }

    /// Adopt the layout with the monomials `missing` added: distinct, absent
    /// from the layout and in its descending order.  Returns the new column
    /// of each.  The layout is already in descending order, so the new one
    /// is a merge of two sorted lists and the old → new map falls out of
    /// the merge, as in `fold_layout`.
    fn insert_columns(&mut self, missing: &[Mask]) -> Vec<u32> {
        let old = &self.columns;
        // `bitop/v1`: one compare and one write a column of the merge
        bitop_tally::word_ops(2 * (old.len() + missing.len()) as u64);
        let mut columns = Vec::with_capacity(old.len() + missing.len());
        let mut map = vec![0u32; old.len()];
        let mut placed = Vec::with_capacity(missing.len());
        let (mut i, mut j) = (0usize, 0usize);
        while i < old.len() || j < missing.len() {
            let take_old = j == missing.len()
                || (i < old.len()
                    && cmp_mono(
                        F2BoolMono::from_mask(old[i]),
                        F2BoolMono::from_mask(missing[j]),
                    ) == std::cmp::Ordering::Greater);
            if take_old {
                map[i] = columns.len() as u32;
                columns.push(old[i]);
                i += 1;
            } else {
                placed.push(columns.len() as u32);
                columns.push(missing[j]);
                j += 1;
            }
        }
        self.adopt_layout(columns, map);
        placed
    }

    /// The linear tail — the intersection of the row space with the
    /// span of the degree-`≤ 1` monomials — in reduced row echelon form,
    /// as polynomials.  Exactly what the from-scratch step's readback
    /// consumes.  Only the rows leading in the tail are materialised.
    pub fn linear_tail(&mut self, cost: &mut InheritCost) -> Vec<F2BoolPoly> {
        if self.refuted {
            return vec![F2BoolPoly::one(self.n_vars)];
        }
        let low_start = self.low_start();
        let low_width = self.columns.len() - low_start;
        // `bitop/v1`: the scan for the tail's first column and the filter
        // of the rows by pivot
        bitop_tally::word_ops((low_start + 1 + self.rows.len()) as u64);
        if low_width == 0 {
            return Vec::new();
        }
        let low_words = low_width.div_ceil(64).max(1);
        let tail_rows: Vec<usize> = (0..self.rows.len())
            .filter(|&r| self.pivot_col[r] as usize >= low_start)
            .collect();
        let mut low: Vec<Vec<u64>> = Vec::with_capacity(tail_rows.len());
        for r in tail_rows {
            let row = self.current_row(r, cost);
            let c = self.pivot_col[r] as usize;
            // the zeroed tail row, and a bit test a column from the pivot
            bitop_tally::word_ops((low_words + self.columns.len() - c) as u64);
            let mut packed = vec![0u64; low_words];
            for column in c..self.columns.len() {
                if row.word(column / 64) & (1u64 << (column % 64)) != 0 {
                    let k = column - low_start;
                    packed[k / 64] |= 1u64 << (k % 64);
                }
            }
            low.push(packed);
        }
        if low.is_empty() {
            return Vec::new();
        }
        let rank = rref_f2_counted(&mut low, low_width, &mut cost.reduce_word_ops);
        let n_vars = self.n_vars;
        // `bitop/v1` read-back: a bit test a tail column of each rank row,
        // and the canonical rebuild of its terms
        bitop_tally::word_ops(
            low.iter()
                .take(rank)
                .map(|row| {
                    let k: u32 = row.iter().map(|w| w.count_ones()).sum();
                    low_width as u64 + bitop_tally::poly::from_monos(k as usize)
                })
                .sum(),
        );
        low.iter()
            .take(rank)
            .map(|row| {
                let monos: Vec<F2BoolMono> = (0..low_width)
                    .filter(|k| row[k / 64] & (1u64 << (k % 64)) != 0)
                    .map(|k| F2BoolMono::from_mask(self.columns[low_start + k]))
                    .collect();
                F2BoolPoly::from_monos(monos, n_vars)
            })
            .collect()
    }

    /// The rows the splitting solver acts on: the constant `1`, and every
    /// `v` or `v + 1` of the linear tail.
    pub fn decisive_rows(&mut self, cost: &mut InheritCost) -> Vec<F2BoolPoly> {
        let tail = self.linear_tail(cost);
        // `bitop/v1`: the shape test, a term each
        bitop_tally::word_ops(tail.iter().map(|p| 1 + 2 * p.terms.len() as u64).sum());
        tail.into_iter()
            .filter(|p| {
                (p.terms.len() == 1 && p.terms[0].mask == 0)
                    || (p.terms.len() <= 2
                        && p.terms.iter().filter(|t| t.mask.count_ones() == 1).count() == 1
                        && p.terms.iter().all(|t| t.mask.count_ones() <= 1))
            })
            .collect()
    }

    /// Specialise by several assignments in sequence.
    pub fn specialise_all(&self, assignments: &[(u32, bool)]) -> (Self, InheritCost) {
        let mut cost = InheritCost::default();
        let mut basis = self.clone();
        for &(v, val) in assignments {
            let (next, c) = basis.specialise(v, val);
            cost.add(c);
            basis = next;
        }
        (basis, cost)
    }
}

/// Specialise `p` by setting variable `var` to `value`.
pub fn substitute(p: &F2BoolPoly, var: u32, value: bool) -> F2BoolPoly {
    p.substitute(var, value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::cryptanalysis::pq_groebner_f2::{
        bit, low_bits, mask_from64, mask_low64, masks_low64, Mask, MASK_BITS,
    };

    /// The next kernels' SIMD row XOR is the scalar one, at every length
    /// and alignment the re-reduction hands it (`dst` and `src` may differ
    /// in length; the shorter decides).
    #[test]
    fn simd_row_xor_matches_the_scalar_one() {
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for len in 0..80usize {
            for extra in 0..3usize {
                let src: Vec<u64> = (0..len + extra).map(|_| next()).collect();
                let dst: Vec<u64> = (0..len).map(|_| next()).collect();
                let mut a = dst.clone();
                let mut b = dst.clone();
                xor_words(&mut a, &src);
                xor_words_simd(&mut b, &src);
                assert_eq!(a, b, "len {len} extra {extra}");
                let mut c = src.clone();
                let mut d = src.clone();
                xor_words(&mut c, &dst);
                xor_words_simd(&mut d, &dst);
                assert_eq!(c, d, "len {len} extra {extra}, longer dst");
            }
        }
    }

    /// The batched rewrite's plan builder is the eager one, segment for
    /// segment and tally for tally, on any map: random ones, mostly
    /// increasing ones (as layout steps are), and heavily deleted ones.
    #[test]
    fn kept_segment_builder_matches_the_original() {
        let mut seed = 0x5e6_4e47_b0b0_1234u64;
        for trial in 0..400 {
            let len = 1 + (xorshift(&mut seed) % 400) as usize;
            let mut next_dst = 0u32;
            let map: Vec<u32> = (0..len)
                .map(|_| {
                    let r = xorshift(&mut seed);
                    match trial % 4 {
                        0 => {
                            if r.is_multiple_of(3) {
                                DELETED
                            } else {
                                (r >> 8) as u32 % (2 * len as u32)
                            }
                        }
                        1 | 2 => {
                            if r.is_multiple_of(5) {
                                DELETED
                            } else {
                                next_dst += (r >> 8) as u32 % 3;
                                if r.is_multiple_of(7) {
                                    next_dst.saturating_sub(5)
                                } else {
                                    next_dst
                                }
                            }
                        }
                        _ => {
                            if !r.is_multiple_of(8) {
                                DELETED
                            } else {
                                (r >> 8) as u32 % (len as u32 + 64)
                            }
                        }
                    }
                })
                .collect();
            for with_runs in [false, true] {
                let t0 = bitop_tally::snapshot();
                let a = StepPlan::new(&map, with_runs);
                let t1 = bitop_tally::snapshot();
                let b = StepPlan::new_kept(&map, with_runs);
                let t2 = bitop_tally::snapshot();
                assert_eq!(a.first, b.first, "trial {trial}");
                assert_eq!(a.segments, b.segments, "trial {trial}");
                assert_eq!(a.low, b.low, "trial {trial}");
                assert_eq!(a.run_first, b.run_first, "trial {trial}");
                assert_eq!(a.runs.len(), b.runs.len(), "trial {trial}");
                assert_eq!(t1.since(t0), t2.since(t1), "trial {trial}: tally");
            }
        }
    }

    /// A random column layout, random fold and delete steps composed into
    /// one map, random rows: the segment plan rewrites every row exactly as
    /// the bit scatter does, on the portable and (where present) the BMI2
    /// path.
    #[test]
    fn rewrite_plan_matches_the_bit_scatter() {
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for trial in 0..200 {
            let n_vars = 4 + (next() % 9) as usize;
            let degree = 2 + (next() % 2) as u32;
            let mut columns = monomials_up_to_mask(all_variable_mask(n_vars), degree);
            columns.sort_unstable_by(|a, b| {
                cmp_mono(F2BoolMono::from_mask(*b), F2BoolMono::from_mask(*a))
            });
            if trial % 3 == 0 {
                // a thinned layout, as support-local roots have
                columns.retain(|_| next() % 3 != 0);
                if columns.is_empty() {
                    continue;
                }
            }
            let n_old = columns.len();
            let steps = 1 + (next() % 3) as usize;
            let mut composed: Vec<u32> = (0..n_old as u32).collect();
            let mut layout = columns.clone();
            let mut used = 0u64;
            for _ in 0..steps {
                let var = (next() % n_vars as u64) as u32;
                if used & (1 << var) != 0 {
                    continue;
                }
                used |= 1 << var;
                let bit = bit(var);
                let mut map = vec![DELETED; layout.len()];
                let mut new_layout = Vec::with_capacity(layout.len());
                if next() % 2 == 0 {
                    fold_layout(&layout, bit, &mut new_layout, &mut map);
                } else {
                    for (k, &m) in layout.iter().enumerate() {
                        if m & bit == 0 {
                            map[k] = new_layout.len() as u32;
                            new_layout.push(m);
                        }
                    }
                }
                for c in composed.iter_mut() {
                    *c = map.get(*c as usize).copied().unwrap_or(DELETED);
                }
                layout = new_layout;
            }
            let src_words = n_old.div_ceil(64);
            let dst_words = layout.len().div_ceil(64).max(1);
            let plan = RewritePlan::new(Rc::new(composed.clone()));
            let step = StepPlan::new(&composed, true);
            for _ in 0..8 {
                let lead = (next() % src_words as u64) as usize;
                let live: Vec<u64> = (lead..src_words)
                    .map(|w| {
                        let mut x = next() & next();
                        if w + 1 == src_words && !n_old.is_multiple_of(64) {
                            x &= (1u64 << (n_old % 64)) - 1;
                        }
                        x
                    })
                    .collect();
                let mut expected = vec![0u64; dst_words];
                let mut expected_lo = usize::MAX;
                for (i, &word) in live.iter().enumerate() {
                    let mut bits = word;
                    while bits != 0 {
                        let b = bits.trailing_zeros() as usize;
                        bits &= bits - 1;
                        let c = composed[(lead + i) * 64 + b];
                        if c != DELETED {
                            expected[c as usize / 64] ^= 1u64 << (c % 64);
                            expected_lo = expected_lo.min(c as usize / 64);
                        }
                    }
                }
                let mut got = vec![0u64; dst_words];
                let lo = apply_plan_portable(&plan, &live, lead, &mut got);
                assert_eq!(got, expected, "trial {trial}");
                assert_eq!(lo, expected_lo, "trial {trial}");
                #[cfg(target_arch = "x86_64")]
                if std::arch::is_x86_feature_detected!("bmi2") {
                    let mut got = vec![0u64; dst_words];
                    let lo = unsafe { apply_plan_bmi2(&plan, &live, lead, &mut got) };
                    assert_eq!(got, expected, "trial {trial} (bmi2)");
                    assert_eq!(lo, expected_lo, "trial {trial} (bmi2)");
                }
                // the eager step plan: shift runs on every CPU, and pdep/pext
                // where present, give the same image
                let mut got = vec![0u64; dst_words];
                apply_step_runs(&step, &live, lead, &mut got, 0);
                assert_eq!(got, expected, "trial {trial} (step runs)");
                #[cfg(target_arch = "x86_64")]
                if std::arch::is_x86_feature_detected!("bmi2") {
                    let mut got = vec![0u64; dst_words];
                    unsafe { apply_step_bmi2(&step, &live, lead, &mut got, 0) };
                    assert_eq!(got, expected, "trial {trial} (step bmi2)");
                }
            }
        }
    }
    use crate::cryptanalysis::koblitz_groebner::matrix_f4_f2;

    /// The sort-and-count parity the completion rows used before
    /// [`odd_terms`] and [`CompletionBatch`].
    fn odd_by_sort(mut monos: Vec<Mask>) -> Vec<Mask> {
        monos.sort_unstable();
        let mut out = Vec::new();
        let mut i = 0;
        while i < monos.len() {
            let mut j = i;
            while j < monos.len() && monos[j] == monos[i] {
                j += 1;
            }
            if (j - i) % 2 == 1 {
                out.push(monos[i]);
            }
            i = j;
        }
        out
    }

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn odd_terms_matches_sorting_parity() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        for round in 0..400 {
            let len = (xorshift(&mut seed) % 90) as usize;
            // a small alphabet, so repeats of every multiplicity occur
            let alphabet = 1 + xorshift(&mut seed) % 40;
            let monos: Vec<Mask> = (0..len)
                .map(|_| mask_from64((xorshift(&mut seed) % alphabet) << (round % 7)))
                .collect();
            let mut got = odd_terms(monos.iter().copied());
            got.sort_unstable();
            assert_eq!(got, odd_by_sort(monos), "round {round}");
        }
    }

    #[test]
    fn xor_words_is_wordwise_xor_over_the_common_length() {
        let mut seed = 0x9e37_79b9_7f4a_7c15u64;
        for len in 0..20 {
            let a: Vec<u64> = (0..len).map(|_| xorshift(&mut seed)).collect();
            let b: Vec<u64> = (0..len + 3).map(|_| xorshift(&mut seed)).collect();
            let mut got = a.clone();
            xor_words(&mut got, &b);
            let want: Vec<u64> = a.iter().zip(&b).map(|(x, y)| x ^ y).collect();
            assert_eq!(got, want);
        }
    }

    /// The completion batch keeps exactly the rows, in order, and adds
    /// exactly the columns that per-row cancellation plus a sorted layout
    /// extension would.
    #[test]
    fn completion_batch_matches_cancel_then_extend() {
        let mut seed = 0x51_7cc1_b727_220au64;
        for _ in 0..60 {
            let n_vars = 12;
            let system = vec![random_poly(n_vars, 3, 14, &mut seed)];
            let Some((basis, _)) = ReducedBasis::from_system(&system, n_vars, 3) else {
                continue;
            };
            // products of random monomials by random multipliers, some
            // cancelling, some reaching outside the layout
            let products: Vec<Vec<Mask>> = (0..12)
                .map(|_| {
                    let t = xorshift(&mut seed) & (0b111 << (xorshift(&mut seed) % 9));
                    (0..(xorshift(&mut seed) % 16))
                        .map(|_| mask_from64((xorshift(&mut seed) & 0xfff) | t))
                        .collect()
                })
                .collect();

            // reference: cancel each row, extend by sorting, pack by index
            let mut reference = basis.clone();
            let kept: Vec<Vec<Mask>> = products
                .iter()
                .map(|p| odd_by_sort(p.clone()))
                .filter(|r| !r.is_empty())
                .collect();
            let old: std::collections::HashSet<Mask> = reference.columns.iter().copied().collect();
            let mut want_columns = reference.columns.clone();
            let mut added: Vec<Mask> = kept
                .iter()
                .flatten()
                .copied()
                .filter(|m| !old.contains(m))
                .collect();
            added.sort_unstable();
            added.dedup();
            want_columns.extend(added);
            want_columns.sort_by(|a, b| {
                cmp_mono(F2BoolMono::from_mask(*a), F2BoolMono::from_mask(*b)).reverse()
            });
            if !kept.is_empty() {
                reference.extend_columns(&kept);
            }
            assert_eq!(reference.columns, want_columns);

            let mut out = basis.clone();
            let mut batch = CompletionBatch::default();
            out.column_index();
            let index = out.column_index.clone().expect("built");
            for p in &products {
                batch.push_row(p.iter().copied(), &index);
            }
            assert_eq!(batch.rows.len(), kept.len());
            let columns = batch.place(&mut out);
            assert_eq!(out.columns, want_columns);
            for (ids, want) in batch.rows.iter().zip(&kept) {
                let mut got: Vec<Mask> = ids
                    .iter()
                    .map(|&id| out.columns[columns[id as usize] as usize])
                    .collect();
                got.sort_unstable();
                assert_eq!(&got, want);
            }
        }
    }

    fn random_poly(n_vars: usize, deg: u32, terms: usize, seed: &mut u64) -> F2BoolPoly {
        let mut next = || {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *seed
        };
        let mut monos = Vec::new();
        for _ in 0..terms {
            let d = (next() % (deg as u64 + 1)) as u32;
            let mut mask = 0u64;
            for _ in 0..d {
                mask |= 1u64 << (next() % n_vars as u64);
            }
            monos.push(F2BoolMono::from_mask(mask_from64(mask)));
        }
        F2BoolPoly::from_monos(monos, n_vars)
    }

    fn canonical(mut rows: Vec<F2BoolPoly>) -> Vec<F2BoolPoly> {
        rows.retain(|p| !p.is_zero());
        rows.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
        rows
    }

    /// RREF of a list of polynomials, as a canonical set.
    fn rref_polys(polys: &[F2BoolPoly], n_vars: usize) -> Vec<F2BoolPoly> {
        let rows_monos: Vec<Vec<Mask>> = polys
            .iter()
            .filter(|p| !p.is_zero())
            .map(|p| p.terms.iter().map(|t| t.mask).collect())
            .collect();
        if rows_monos.is_empty() {
            return Vec::new();
        }
        let cols = macaulay_columns(&rows_monos).unwrap();
        let mut matrix = pack_rows(&rows_monos, &cols);
        let mut ops = 0;
        let rank = rref_f2_counted(&mut matrix, cols.len(), &mut ops);
        canonical(
            matrix
                .iter()
                .take(rank)
                .map(|row| {
                    F2BoolPoly::from_monos(
                        (0..cols.len())
                            .filter(|c| row[c / 64] & (1u64 << (c % 64)) != 0)
                            .map(|c| F2BoolMono::from_mask(cols[c]))
                            .collect(),
                        n_vars,
                    )
                })
                .collect(),
        )
    }

    /// From-scratch Macaulay row space with multipliers over `mask`.
    fn masked_space(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        mask: Mask,
    ) -> Vec<F2BoolPoly> {
        use crate::cryptanalysis::koblitz_groebner::macaulay_rows_monos_with_mask;
        let rows = macaulay_rows_monos_with_mask(system, n_vars, degree, mask, None).unwrap();
        let polys: Vec<F2BoolPoly> = rows
            .iter()
            .map(|monos| {
                F2BoolPoly::from_monos(
                    monos.iter().map(|&m| F2BoolMono::from_mask(m)).collect(),
                    n_vars,
                )
            })
            .collect();
        rref_polys(&polys, n_vars)
    }

    fn tail_of(polys: &[F2BoolPoly]) -> Vec<F2BoolPoly> {
        polys
            .iter()
            .filter(|p| p.terms.iter().all(|t| t.mask.count_ones() <= 1))
            .cloned()
            .collect()
    }

    /// The solver's view of a tail: a refutation if `1` is present,
    /// otherwise the tail itself.  The legacy step multiplies by assigned
    /// variables too, and once `1` is in the row space those junk rows put
    /// `x·1 = x` into its tail; the solver never reads past the `1`.
    fn solver_view(tail: &[F2BoolPoly], n_vars: usize) -> Vec<F2BoolPoly> {
        let tail = rref_polys(tail, n_vars);
        if tail
            .iter()
            .any(|p| p.terms.len() == 1 && p.terms[0].mask == 0)
        {
            vec![F2BoolPoly::one(n_vars)]
        } else {
            tail
        }
    }

    /// The solver refutes a system containing the constant `1` before any
    /// reduction, so the exact row-space invariant is only claimed for
    /// systems it would actually reduce.
    fn has_constant(system: &[F2BoolPoly]) -> bool {
        system
            .iter()
            .any(|p| p.terms.len() == 1 && p.terms[0].mask == 0)
    }

    fn occurring(system: &[F2BoolPoly]) -> Mask {
        system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0, |a, t| a | t.mask)
    }

    /// Does every polynomial of `inner` reduce to zero against the RREF
    /// `outer`?  (`inner ⊆ span(outer)`.)
    fn contained(inner: &[F2BoolPoly], outer: &[F2BoolPoly], n_vars: usize) -> bool {
        let joint = rref_polys(&[outer, inner].concat(), n_vars);
        joint.len() == rref_polys(outer, n_vars).len()
    }

    /// The sandwich `V_occurring(S) ⊆ basis ⊆ V_unassigned(S)` the module
    /// documents, for a node reached by assigning `assigned`.
    /// A refuted basis is collapsed to `1`, so for it the claim is only the
    /// solver's view: the smallest space of the sandwich refutes as well.
    fn assert_sandwich(
        basis: &ReducedBasis,
        system: &[F2BoolPoly],
        n_vars: usize,
        assigned: Mask,
        what: &str,
    ) {
        let rows = rref_polys(&basis_polys(basis), n_vars);
        let lower = masked_space(system, n_vars, basis.degree, occurring(system) & !assigned);
        if basis.refuted {
            assert_eq!(
                rows,
                vec![F2BoolPoly::one(n_vars)],
                "{what}: a refuted basis is exactly 1"
            );
            assert!(
                has_constant(&lower),
                "{what}: refuted, but V_occurring does not contain 1"
            );
            return;
        }
        let upper = masked_space(
            system,
            n_vars,
            basis.degree,
            all_variable_mask(n_vars) & !assigned,
        );
        assert!(
            contained(&lower, &rows, n_vars),
            "{what}: V_occurring not contained in the basis"
        );
        assert!(
            contained(&rows, &upper, n_vars),
            "{what}: basis not contained in V_unassigned"
        );
    }

    /// Every row, materialised into the current layout, as polynomials.
    fn basis_polys(b: &ReducedBasis) -> Vec<F2BoolPoly> {
        let mut b = b.clone();
        let mut cost = InheritCost::default();
        b.materialise_all(&mut cost);
        b.rows
            .iter()
            .map(|row| {
                F2BoolPoly::from_monos(
                    (0..b.columns.len())
                        .filter(|c| row.word(c / 64) & (1u64 << (c % 64)) != 0)
                        .map(|c| F2BoolMono::from_mask(b.columns[c]))
                        .collect(),
                    b.n_vars,
                )
            })
            .collect()
    }

    /// Every rewrite path charges the same work and builds the same rows.
    ///
    /// The BMI2 host path (`pext`/`pdep`, eager through the step plan) and
    /// the portable one that every aarch64 host runs (shift runs, and the
    /// bit scatter for rows from older layouts) are forced in turn, in the
    /// same process, on the same random specialisation chains.  Counted
    /// work is part of a method's recorded identity, so a session recorded
    /// on one path must replay on the other: every counter of
    /// [`InheritCost`], at every step, and every row's words must match
    /// exactly.  The lazy modes (`Off`) are compared pairwise the same way.
    #[test]
    fn rewrite_paths_charge_identical_work_and_build_identical_rows() {
        type Print = Vec<(u32, u32, u32, u32, Vec<u64>)>;
        fn print(b: &ReducedBasis) -> Print {
            b.rows
                .iter()
                .zip(&b.pivot_col)
                .map(|(r, &pc)| (pc, r.version, r.start, r.lead, r.live().to_vec()))
                .collect()
        }
        // One chain of specialisations under a forced path: the cost and
        // the rows after every step, and the cost of reading the tail.
        fn chain(
            mode: EagerMode,
            wordwise: bool,
            system: &[F2BoolPoly],
            n_vars: usize,
            degree: u32,
            steps: &[(u32, bool)],
        ) -> Vec<(InheritCost, Print)> {
            rewrite_path_override::with(mode, wordwise, || {
                let (mut basis, root_cost) =
                    ReducedBasis::from_system(system, n_vars, degree).unwrap();
                let mut out = vec![(root_cost, print(&basis))];
                for &(v, value) in steps {
                    let (next, cost) = basis.specialise(v, value);
                    basis = next;
                    out.push((cost, print(&basis)));
                    let mut tail_cost = InheritCost::default();
                    let _ = basis.linear_tail(&mut tail_cost);
                    out.push((tail_cost, print(&basis)));
                }
                out
            })
        }
        let have_bmi2 = cfg!(target_arch = "x86_64") && {
            #[cfg(target_arch = "x86_64")]
            let b = std::arch::is_x86_feature_detected!("bmi2");
            #[cfg(not(target_arch = "x86_64"))]
            let b = false;
            b
        };
        // (host path, portable path) pairs; the BMI2 ones need the CPU.
        let mut pairs = vec![(EagerMode::Runs, EagerMode::Runs, false, false)];
        pairs.push((EagerMode::Off, EagerMode::Off, false, false));
        if have_bmi2 {
            pairs.push((EagerMode::Bmi2, EagerMode::Runs, true, false));
            pairs.push((EagerMode::Off, EagerMode::Off, true, false));
        }
        let mut seed = 0x0bad_5eed_1234_5678u64;
        let mut compared = 0usize;
        for trial in 0..60 {
            let n_vars = 6 + trial % 5;
            let m = 3 + trial % 4;
            let system: Vec<F2BoolPoly> = (0..m)
                .map(|k| random_poly(n_vars, if k % 4 == 3 { 1 } else { 2 }, 5 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            if system.is_empty() {
                continue;
            }
            let degree = 2 + (trial % 2) as u32;
            let steps: Vec<(u32, bool)> = (0..3)
                .map(|i| ((xorshift(&mut seed) % n_vars as u64) as u32 + i) % n_vars as u32)
                .zip([false, true, true])
                .map(|(v, b)| (v, b ^ (trial % 2 == 0)))
                .collect();
            // distinct variables only
            let mut steps_d: Vec<(u32, bool)> = Vec::new();
            for st in steps {
                if steps_d.iter().all(|&(v, _)| v != st.0) {
                    steps_d.push(st);
                }
            }
            for &(mode_a, mode_b, wordwise_a, wordwise_b) in &pairs {
                let a = chain(mode_a, wordwise_a, &system, n_vars, degree, &steps_d);
                let b = chain(mode_b, wordwise_b, &system, n_vars, degree, &steps_d);
                assert_eq!(a.len(), b.len());
                for (i, ((ca, pa), (cb, pb))) in a.iter().zip(&b).enumerate() {
                    assert_eq!(
                        ca, cb,
                        "trial {trial} step {i}: charged work differs ({mode_a:?}/{wordwise_a} vs {mode_b:?}/{wordwise_b})"
                    );
                    assert_eq!(
                        pa, pb,
                        "trial {trial} step {i}: rows differ ({mode_a:?}/{wordwise_a} vs {mode_b:?}/{wordwise_b})"
                    );
                    compared += 1;
                }
            }
        }
        assert!(compared > 100, "too few comparisons: {compared}");
    }

    /// **The batched rewrite is the eager rewrite, done fewer times.**
    /// Chains of specialisations (a "split" — every row brought current by
    /// [`ReducedBasis::materialise_shared`] — after every second step, and
    /// once at the end), the linear tail read after every step, on both
    /// eager kernels: with the switch on, every tail is the same, the rows
    /// are the same polynomials with the same pivots, and the work charged
    /// is never more than the eager rewrite's.
    #[test]
    fn batched_rewrite_builds_identical_bases_with_no_more_work() {
        use crate::cryptanalysis::koblitz_groebner::with_batched_rewrite;
        type Rows = Vec<(u32, Vec<u64>)>;
        fn full_rows(b: &ReducedBasis) -> Rows {
            let mut b = b.clone();
            let mut c = InheritCost::default();
            b.materialise_all(&mut c);
            let words = b.words;
            b.rows
                .iter()
                .zip(&b.pivot_col)
                .map(|(r, &pc)| {
                    let mut full = vec![0u64; words];
                    full[r.start as usize..].copy_from_slice(&r.data);
                    (pc, full)
                })
                .collect()
        }
        type Chain = (u64, Vec<Vec<F2BoolPoly>>, Vec<Rows>);
        fn chain(
            mode: EagerMode,
            batched: bool,
            system: &[F2BoolPoly],
            n_vars: usize,
            degree: u32,
            steps: &[(u32, bool)],
        ) -> Chain {
            rewrite_path_override::with(mode, mode == EagerMode::Bmi2, || {
                with_batched_rewrite(batched, || {
                    let (mut basis, root_cost) =
                        ReducedBasis::from_system(system, n_vars, degree).unwrap();
                    let mut work = root_cost.word_ops();
                    let mut tails = Vec::new();
                    let mut rows = vec![full_rows(&basis)];
                    for (i, &(v, value)) in steps.iter().enumerate() {
                        let (next, cost) = basis.specialise(v, value);
                        basis = next;
                        work += cost.word_ops();
                        let mut tail_cost = InheritCost::default();
                        tails.push(basis.linear_tail(&mut tail_cost));
                        work += tail_cost.word_ops();
                        if batched && (i % 2 == 1 || i + 1 == steps.len()) {
                            let mut c = InheritCost::default();
                            basis.materialise_shared(&mut c);
                            work += c.word_ops();
                        }
                        rows.push(full_rows(&basis));
                    }
                    (work, tails, rows)
                })
            })
        }
        let have_bmi2 = cfg!(target_arch = "x86_64") && {
            #[cfg(target_arch = "x86_64")]
            let b = std::arch::is_x86_feature_detected!("bmi2");
            #[cfg(not(target_arch = "x86_64"))]
            let b = false;
            b
        };
        let mut modes = vec![EagerMode::Runs];
        if have_bmi2 {
            modes.push(EagerMode::Bmi2);
        }
        let mut seed = 0xba7c_4ed0_5eed_0001u64;
        let (mut compared, mut fewer) = (0usize, 0usize);
        let (mut eager_total, mut batched_total) = (0u64, 0u64);
        for trial in 0..80 {
            let n_vars = 7 + trial % 6;
            let m = 4 + trial % 5;
            let system: Vec<F2BoolPoly> = (0..m)
                .map(|k| random_poly(n_vars, if k % 5 == 4 { 1 } else { 2 }, 6 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            if system.is_empty() {
                continue;
            }
            let degree = 2 + (trial % 2) as u32;
            let mut steps: Vec<(u32, bool)> = Vec::new();
            while steps.len() < 5 {
                let v = (xorshift(&mut seed) % n_vars as u64) as u32;
                if steps.iter().all(|&(u, _)| u != v) {
                    steps.push((v, xorshift(&mut seed) % 2 == 1));
                }
            }
            for &mode in &modes {
                let eager = chain(mode, false, &system, n_vars, degree, &steps);
                let batched = chain(mode, true, &system, n_vars, degree, &steps);
                assert_eq!(eager.1, batched.1, "trial {trial} {mode:?}: tails differ");
                assert_eq!(eager.2, batched.2, "trial {trial} {mode:?}: rows differ");
                assert!(
                    batched.0 <= eager.0,
                    "trial {trial} {mode:?}: batched charged {} > eager {}",
                    batched.0,
                    eager.0
                );
                compared += 1;
                fewer += usize::from(batched.0 < eager.0);
                eager_total += eager.0;
                batched_total += batched.0;
            }
        }
        assert!(compared >= 80, "too few comparisons: {compared}");
        assert!(fewer > 0, "the batched rewrite never saved anything");
        eprintln!(
            "batched rewrite, basis chains: {compared} compared, {fewer} cheaper, \
             word ops {batched_total} / {eager_total}"
        );
    }

    /// The charge itself is pinned, so changing what is counted cannot pass
    /// unnoticed: if this fails, the accounting changed.  Bump
    /// [`INHERITED_F4_ACCOUNTING_VERSION`], then update the figures below in
    /// the same change.  The portable eager path is forced, which every host
    /// can run and which charges what the BMI2 path charges (the test above).
    #[test]
    fn inherited_f4_accounting_version_pins_the_charge() {
        let mut seed = 0x5eed_acc0_0057_0001u64;
        let mut total = InheritCost::default();
        let mut chains = 0usize;
        for trial in 0..40 {
            let n_vars = 6 + trial % 5;
            let m = 3 + trial % 4;
            let system: Vec<F2BoolPoly> = (0..m)
                .map(|k| random_poly(n_vars, if k % 4 == 3 { 1 } else { 2 }, 5 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            if system.is_empty() {
                continue;
            }
            let degree = 2 + (trial % 2) as u32;
            rewrite_path_override::with(EagerMode::Runs, false, || {
                let (mut basis, root) = ReducedBasis::from_system(&system, n_vars, degree).unwrap();
                total.add(root);
                let mut used = Vec::new();
                for i in 0..3u32 {
                    let v = ((xorshift(&mut seed) % n_vars as u64) as u32 + i) % n_vars as u32;
                    if used.contains(&v) {
                        continue;
                    }
                    used.push(v);
                    let (next, cost) = basis.specialise(v, (i + trial as u32).is_multiple_of(2));
                    basis = next;
                    total.add(cost);
                    let mut tail = InheritCost::default();
                    let _ = basis.linear_tail(&mut tail);
                    total.add(tail);
                }
            });
            chains += 1;
        }
        assert!(chains > 30, "too few chains: {chains}");
        let got = (
            total.reduce_word_ops,
            total.rref_word_ops,
            total.specialise_word_ops,
            total.displaced_rows,
            total.completion_rows,
            total.closure_products,
        );
        assert_eq!(
            got,
            (9944, 0, 8581, 914, 287, 0),
            "the inherited-F4 charge changed: bump INHERITED_F4_ACCOUNTING_VERSION (now {}) and \
             update this pin",
            INHERITED_F4_ACCOUNTING_VERSION
        );
    }

    #[test]
    fn specialised_basis_spans_the_from_scratch_row_space() {
        let mut seed = 0x1234_5678_9abc_def1u64;
        for trial in 0..60 {
            let n_vars = 5 + trial % 4;
            let m = 3 + trial % 4;
            let system: Vec<F2BoolPoly> = (0..m)
                .map(|k| random_poly(n_vars, if k % 4 == 3 { 1 } else { 2 }, 5 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            if system.is_empty() {
                continue;
            }
            for degree in 2..=3u32 {
                let (mut root, _) = ReducedBasis::from_system(&system, n_vars, degree).unwrap();
                // Root: the sandwich with nothing assigned, and the legacy tail.
                assert_sandwich(
                    &root,
                    &system,
                    n_vars,
                    0,
                    &format!("trial {trial} degree {degree} root"),
                );
                let f4 = matrix_f4_f2(&system, n_vars, degree).unwrap();
                let mut root_cost = InheritCost::default();
                assert_eq!(
                    solver_view(&tail_of(&f4), n_vars),
                    solver_view(&root.linear_tail(&mut root_cost), n_vars),
                    "trial {trial} degree {degree}: root tail differs"
                );
                // Every one-variable specialisation, both values, then a second one.
                let occurring = system
                    .iter()
                    .flat_map(|p| p.terms.iter())
                    .fold(0, |a, t| a | t.mask);
                let _all = all_variable_mask(n_vars);
                for v in 0..n_vars as u32 {
                    if occurring & (1 << v) == 0 {
                        continue;
                    }
                    for value in [false, true] {
                        let (mut child, _) = root.specialise(v, value);
                        let child_system: Vec<F2BoolPoly> = system
                            .iter()
                            .map(|p| substitute(p, v, value))
                            .filter(|p| !p.is_zero())
                            .collect();
                        assert_eq!(*child.system, child_system);
                        if !has_constant(&child_system) {
                            assert_sandwich(
                                &child,
                                &child_system,
                                n_vars,
                                1 << v,
                                &format!("trial {trial} degree {degree} v{v}={value}"),
                            );
                        }
                        // Tail agreement with the legacy all-variable step's readback.
                        let legacy =
                            matrix_f4_f2(&child_system, n_vars, degree).unwrap_or_default();
                        let mut cost = InheritCost::default();
                        let tail = child.linear_tail(&mut cost);
                        assert_eq!(
                            solver_view(&tail_of(&legacy), n_vars),
                            solver_view(&tail, n_vars),
                            "trial {trial} degree {degree} v{v}={value}: tail differs"
                        );
                        // One more level.
                        let w = (v + 1) % n_vars as u32;
                        let (mut grandchild, _) = child.specialise(w, !value);
                        let gc_system: Vec<F2BoolPoly> = child_system
                            .iter()
                            .map(|p| substitute(p, w, !value))
                            .filter(|p| !p.is_zero())
                            .collect();
                        if !has_constant(&gc_system) {
                            assert_sandwich(
                                &grandchild,
                                &gc_system,
                                n_vars,
                                (1 << v) | (1 << w),
                                &format!(
                                    "trial {trial} degree {degree} v{v}={value} w{w}: grandchild"
                                ),
                            );
                        }
                        let legacy = matrix_f4_f2(&gc_system, n_vars, degree).unwrap_or_default();
                        let tail = grandchild.linear_tail(&mut cost);
                        assert_eq!(
                            solver_view(&tail_of(&legacy), n_vars),
                            solver_view(&tail, n_vars),
                            "trial {trial} degree {degree} v{v}={value} w{w}: grandchild tail differs"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn degree_drop_completion_rows_are_added() {
        // f = x0·x1 + x2: setting x0 = 0 makes it linear, so degree-2
        // multipliers appear in the from-scratch matrix at degree 3.
        let n_vars = 4;
        let f = F2BoolPoly::from_monos(
            vec![F2BoolMono::from_mask(0b0011), F2BoolMono::from_mask(0b0100)],
            n_vars,
        );
        let g = F2BoolPoly::from_monos(
            vec![
                F2BoolMono::from_mask(0b1100),
                F2BoolMono::from_mask(0b0010),
                F2BoolMono::one(),
            ],
            n_vars,
        );
        let (root, _) = ReducedBasis::from_system(&[f, g], n_vars, 3).unwrap();
        let (child, cost) = root.specialise(0, false);
        assert!(
            cost.completion_rows > 0,
            "f dropped to degree 1; completion expected"
        );
        let system = child.system.clone();
        assert_sandwich(&child, &system, n_vars, 1, "degree drop");
        // With every unassigned variable occurring, the sandwich is an equality.
        assert_eq!(occurring(&system), all_variable_mask(n_vars) & !1);
        assert_eq!(
            masked_space(&system, n_vars, 3, all_variable_mask(n_vars) & !1),
            rref_polys(&basis_polys(&child), n_vars)
        );
    }

    /// Every point of `F_2^n_vars` on which the whole system vanishes.
    fn variety(system: &[F2BoolPoly], n_vars: usize) -> Vec<u64> {
        (0..1u64 << n_vars)
            .filter(|&x| system.iter().all(|p| p.eval(mask_from64(x)) == 0))
            .collect()
    }

    /// A closed basis may leave the Macaulay space — that is its point —
    /// but must stay inside the ideal and keep everything the plain basis
    /// has, so its tail refutes or pins at least what the plain one does.
    fn assert_closed_sound(
        basis: &ReducedBasis,
        system: &[F2BoolPoly],
        n_vars: usize,
        assigned: Mask,
        what: &str,
    ) {
        let rows = basis_polys(basis);
        let roots = variety(system, n_vars);
        for p in &rows {
            assert!(
                roots.iter().all(|&x| p.eval(mask_from64(x)) == 0),
                "{what}: a closed row does not vanish on the variety"
            );
        }
        if !has_constant(system) && !basis.refuted {
            let lower = masked_space(system, n_vars, basis.degree, occurring(system) & !assigned);
            assert!(
                contained(&lower, &rref_polys(&rows, n_vars), n_vars),
                "{what}: the closed basis lost part of the Macaulay space"
            );
        }
    }

    #[test]
    fn closed_bases_stay_in_the_ideal_and_contain_the_macaulay_space() {
        let mut seed = 0x0fed_cba9_8765_4321u64;
        let mut products = 0u64;
        for trial in 0..40 {
            let n_vars = 5 + trial % 4;
            let m = 3 + trial % 3;
            let system: Vec<F2BoolPoly> = (0..m)
                .map(|k| random_poly(n_vars, 2, 4 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            if system.is_empty() {
                continue;
            }
            let rounds = 1 + trial as u32 % 2;
            let (root, cost) =
                ReducedBasis::from_system_closed(&system, n_vars, 3, rounds).unwrap();
            products += cost.closure_products;
            assert_closed_sound(&root, &system, n_vars, 0, &format!("trial {trial} root"));
            for v in 0..n_vars as u32 {
                if occurring(&system) & (1 << v) == 0 {
                    continue;
                }
                for value in [false, true] {
                    let (child, cost) = root.specialise(v, value);
                    products += cost.closure_products;
                    let child_system: Vec<F2BoolPoly> = system
                        .iter()
                        .map(|p| substitute(p, v, value))
                        .filter(|p| !p.is_zero())
                        .collect();
                    assert_closed_sound(
                        &child,
                        &child_system,
                        n_vars,
                        1 << v,
                        &format!("trial {trial} v{v}={value}"),
                    );
                }
            }
        }
        assert!(
            products > 0,
            "no degree fall was closed; the test exercised nothing"
        );
    }

    #[test]
    fn a_refuted_child_is_one_and_stays_refuted_for_free() {
        let mut seed = 0x0bad_5eed_0dd5_0001u64;
        let mut refuted_children = 0;
        for trial in 0..120 {
            let n_vars = 5 + trial % 3;
            let system: Vec<F2BoolPoly> = (0..3 + trial % 3)
                .map(|k| random_poly(n_vars, 2, 4 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            let Some((root, _)) = ReducedBasis::from_system(&system, n_vars, 3) else {
                continue;
            };
            if root.refuted {
                continue;
            }
            let occurring = occurring(&system);
            for v in (0..n_vars as u32).filter(|v| occurring & (1 << v) != 0) {
                for value in [false, true] {
                    let (child, _) = root.specialise(v, value);
                    if !child.refuted {
                        continue;
                    }
                    refuted_children += 1;
                    let child_system: Vec<F2BoolPoly> = system
                        .iter()
                        .map(|p| substitute(p, v, value))
                        .filter(|p| !p.is_zero())
                        .collect();
                    let legacy = matrix_f4_f2(&child_system, n_vars, 3).unwrap_or_default();
                    assert!(
                        has_constant(&legacy),
                        "trial {trial} v{v}={value}: refuted, but not from scratch"
                    );
                    assert_eq!(basis_polys(&child), vec![F2BoolPoly::one(n_vars)]);
                    let Some(w) = (0..n_vars as u32).find(|&w| w != v && occurring & (1 << w) != 0)
                    else {
                        continue;
                    };
                    let (mut grandchild, cost) = child.specialise(w, !value);
                    assert!(
                        grandchild.refuted,
                        "trial {trial}: refutation lost below v{v}={value}"
                    );
                    assert_eq!(
                        cost.word_ops(),
                        0,
                        "trial {trial}: a refuted basis cost something to specialise"
                    );
                    let mut read = InheritCost::default();
                    assert_eq!(
                        grandchild.decisive_rows(&mut read),
                        vec![F2BoolPoly::one(n_vars)]
                    );
                    assert_eq!(read.word_ops(), 0);
                }
            }
        }
        assert!(
            refuted_children > 0,
            "no specialisation refuted; the test exercised nothing"
        );
    }

    #[test]
    #[ignore = "global compact-refutation toggle requires an isolated test process"]
    fn compact_refuted_basis_survives_two_more_specialisations() {
        let mut seed = 0x0bad_5eed_0dd5_0001u64;
        set_compact_refuted_basis(false);
        for trial in 0..120 {
            let n_vars = 5 + trial % 3;
            let system: Vec<F2BoolPoly> = (0..3 + trial % 3)
                .map(|k| random_poly(n_vars, 2, 4 + k, &mut seed))
                .filter(|p| poly_degree(p) >= 1)
                .collect();
            let Some((root, _)) = ReducedBasis::from_system(&system, n_vars, 3) else {
                continue;
            };
            if root.refuted {
                continue;
            }
            for v in 0..n_vars as u32 {
                for value in [false, true] {
                    let (ordinary, _) = root.specialise(v, value);
                    if !ordinary.refuted {
                        continue;
                    }
                    set_compact_refuted_basis(true);
                    let (compact, _) = root.specialise(v, value);
                    set_compact_refuted_basis(false);
                    assert!(compact.refuted);
                    assert_eq!(compact.columns, vec![0]);
                    assert_eq!(compact.words, 1);
                    assert!(compact.history.is_empty());
                    let remaining: Vec<u32> =
                        (0..n_vars as u32).filter(|&w| w != v).take(2).collect();
                    let (mut ordinary, _) =
                        ordinary.specialise_all(&[(remaining[0], true), (remaining[1], false)]);
                    let (mut compact, cost) =
                        compact.specialise_all(&[(remaining[0], true), (remaining[1], false)]);
                    assert_eq!(cost.word_ops(), 0);
                    assert_eq!(
                        ordinary.decisive_rows(&mut InheritCost::default()),
                        compact.decisive_rows(&mut InheritCost::default())
                    );
                    return;
                }
            }
        }
        panic!("seeded systems produced no refuted child");
    }

    #[test]
    fn zero_closure_rounds_is_the_plain_basis() {
        let mut seed = 0x5555_aaaa_1234_4321u64;
        for trial in 0..20 {
            let n_vars = 5 + trial % 4;
            let system: Vec<F2BoolPoly> = (0..4)
                .map(|k| random_poly(n_vars, 2, 5 + k, &mut seed))
                .collect();
            let (plain, plain_cost) = ReducedBasis::from_system(&system, n_vars, 3).unwrap();
            let (closed, closed_cost) =
                ReducedBasis::from_system_closed(&system, n_vars, 3, 0).unwrap();
            assert_eq!(basis_polys(&plain), basis_polys(&closed), "trial {trial}");
            assert_eq!(plain_cost, closed_cost, "trial {trial}");
        }
    }

    /// The lane-batched roots are, lane for lane, the reduced row echelon
    /// form the sequential kernel computes from the same rows: the same
    /// rows in the same order, the same pivots, the same refutation.  The
    /// lanes share some generators and differ in others, so the batch
    /// meets both shared and divergent pivot patterns; every lane width
    /// (8 to 64 bits) is exercised.
    #[test]
    fn lane_roots_match_the_sequential_rref() {
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        for &(k, n_vars, degree) in &[
            (1usize, 9usize, 3u32),
            (2, 10, 3),
            (5, 9, 3),
            (8, 10, 2),
            (13, 9, 3),
            (40, 8, 3),
            (64, 8, 2),
        ] {
            for support_local in [false, true] {
                let shared: Vec<F2BoolPoly> = (0..3)
                    .map(|_| random_poly(n_vars, 2, 6, &mut seed))
                    .collect();
                let systems: Vec<Vec<F2BoolPoly>> = (0..k)
                    .map(|lane| {
                        let mut s = shared.clone();
                        let own = 1 + lane % 3;
                        s.extend((0..own).map(|_| random_poly(n_vars, 2, 5, &mut seed)));
                        if lane % 7 == 3 {
                            // a zero generator, dropped as the root drops it
                            s.push(F2BoolPoly::from_monos(Vec::new(), n_vars));
                        }
                        s
                    })
                    .collect();
                let slices: Vec<&[F2BoolPoly]> = systems.iter().map(|s| s.as_slice()).collect();
                let (bases, stats) =
                    ReducedBasis::from_systems_lanes(&slices, n_vars, degree, support_local);
                assert_eq!(stats.lanes as usize, k);
                assert!(stats.pivot_passes >= stats.pivot_columns);
                for (lane, got) in bases.into_iter().enumerate() {
                    let root =
                        RootMatrix::build(&systems[lane], n_vars, degree, support_local).unwrap();
                    let mut matrix = root.matrix.clone();
                    let mut ops = 0u64;
                    let rank = if matrix.is_empty() {
                        0
                    } else {
                        rref_f2_counted(&mut matrix, root.columns.len(), &mut ops)
                    };
                    matrix.truncate(rank);
                    let columns = if root.matrix.is_empty() {
                        Vec::new()
                    } else {
                        root.columns.clone()
                    };
                    let want = ReducedBasis::assemble(
                        root.system.clone(),
                        root.generator_degrees.clone(),
                        n_vars,
                        degree,
                        support_local,
                        columns,
                        matrix,
                    );
                    let (got, _) = got.expect("every lane fits");
                    assert_eq!(got.columns, want.columns, "k={k} lane={lane}");
                    assert_eq!(got.pivot_col, want.pivot_col, "k={k} lane={lane}");
                    assert_eq!(got.refuted, want.refuted, "k={k} lane={lane}");
                    let rows = |b: &ReducedBasis| -> Vec<Vec<u64>> {
                        b.rows.iter().map(|r| r.data.to_vec()).collect()
                    };
                    assert_eq!(rows(&got), rows(&want), "k={k} lane={lane}");
                    assert_eq!(got.system, want.system);
                }
            }
        }
    }
}
