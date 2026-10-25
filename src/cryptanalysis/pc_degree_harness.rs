//! # Refutation-degree (last-fall / PC-degree) harness for binary Semaev PDP.
//!
//! Companion to [`crate::cryptanalysis::ffd_harness`].  Where that module
//! measures the **first** fall degree `d_ff` of the unrestricted binary
//! Semaev `S₃` system, this one measures the number that actually decides
//! security: the **refutation degree** `D*` — the smallest Macaulay
//! degree at which a Gröbner / Polynomial-Calculus computation derives the
//! unit `1` from a **non-decomposable** point-decomposition instance.
//!
//! ## Why a different instance
//!
//! `ffd_harness` lets `X₁, X₂` range over *all* of `F_{2ⁿ}` (the full `2n`
//! bit-variables).  That system is generically *satisfiable* — `S₃ = 0`
//! defines a curve with ≈ `2ⁿ` points — so `1 ∉ ⟨S₃, field eqs⟩` and there
//! is **no refutation** to measure.  The genuine index-calculus point-
//! decomposition problem (PDP) restricts the factor base to a proper
//! `F_2`-subspace `V ⊂ F_{2ⁿ}` of dimension `n' < n` and asks
//!
//! ```text
//!     ∃ X₁, X₂ ∈ V :  S₃(X₁, X₂, x(R)) = 0 ?
//! ```
//!
//! For `2n' < n` *most* targets `x(R)` are **non-decomposable**: the
//! restricted system is *unsatisfiable*, so `1 ∈ ⟨ … ⟩` and a Gröbner
//! computation must derive it.  By the Clegg–Edmonds–Impagliazzo
//! correspondence the degree at which `1` first appears in the Macaulay
//! row-space is exactly the Polynomial-Calculus refutation degree, which
//! equals (up to the usual `O(1)`) the Huang–Kosters–Yeo **last fall
//! degree** and the Gröbner solving degree.  See
//! `research/notes/index-calculus/RESEARCH_FFD_PROOF_COMPLEXITY.md` §2.
//!
//! ## What this measures, operationally
//!
//! Per `(n, n')` and a non-decomposable target `x₃`:
//!
//! 1. Build the `n` quadratic `F_2`-equations via
//!    [`ffd_harness::weil_descend_s3`], then **restrict** to the subspace
//!    `V = span{z⁰, …, z^{n'-1}}` by forcing the high bits of `X₁, X₂` to
//!    zero ([`restrict_to_subspace`]) — leaving `2n'` bit-variables.
//! 2. At each degree `D = 2, 3, …`, build the Macaulay matrix
//!    ([`ffd_harness::build_macaulay_rows`]) and test whether the constant
//!    monomial `1` (column 0) lies in the row-space ([`e0_in_rowspace`]).
//! 3. Report:
//!    - **first fall degree** `d_ff` — smallest `D` with `rank < rows` and
//!      `rank < cols` (same operational definition as `ffd_harness`), and
//!    - **refutation degree** `D*` — smallest `D` with `1 ∈ row-space`.
//!
//! The gap `D* − d_ff` is the Huang–Kosters–Yeo invariant made
//! operational, and the proposal's **prediction #1** is that across a
//! field sweep `d_ff` stays ≈ constant while `D*` climbs.
//!
//! ## Calibration
//!
//! Decomposability of `x₃` over `V` is checked by brute force over
//! `V × V` ([`is_decomposable`]).  A decomposable target is *satisfiable*,
//! so it must show **no** refutation (`D* = None`); a non-decomposable
//! target must eventually refute.  The test
//! `refutation_iff_nondecomposable` asserts exactly this equivalence at
//! small `n`, which validates the whole pipeline.
//!
//! ## Limits
//!
//! - Cost is dominated by the Macaulay rank at degree `D` in `v = 2n'`
//!   variables.  Comfortable to `v ≤ 10` (`n ≤ 10` with `n' = ⌊n/2⌋`) and
//!   `D ≤ v`.  Beyond that, swap in a sparse F4 backend
//!   (`cryptanalysis::groebner_f4`).
//! - Single `(b, x₃)` per `(n, n')`; for paper-grade numbers average over
//!   many non-decomposable targets.
//!
//! ## References
//!
//! - M. Clegg, J. Edmonds, R. Impagliazzo, *Using the Groebner basis
//!   algorithm to find proofs of unsatisfiability*, STOC 1996.
//! - M.-D. Huang, M. Kosters, S. L. Yeo, *Last fall degree, HFE, and Weil
//!   descent attacks on ECDLP*, CRYPTO 2015.
//! - I. Semaev, *Summation polynomials …*, ePrint 2004/031.

use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use crate::cryptanalysis::binary_semaev::binary_semaev_s3;
use crate::cryptanalysis::ffd_harness::{
    build_macaulay_rows, choose_irreducible, f2_rank, quad_monomial_index, random_nonzero_f2m,
    weil_descend_s3, F2BoolPoly,
};
use crate::cryptanalysis::gf2_elim;
use rand::rngs::StdRng;
use rand::SeedableRng;

// ── Public API ──────────────────────────────────────────────────────

/// One row of the refutation-degree table.
#[derive(Clone, Debug)]
pub struct PcRow {
    /// Field extension degree `n` (system lives in `F_{2ⁿ}`).
    pub n: u32,
    /// Factor-base subspace dimension `n'` (`V = span{z⁰..z^{n'-1}}`).
    pub n_sub: u32,
    /// Bit-variables after restriction (`2n'`).
    pub num_vars: u32,
    /// `F_2`-equations after Weil descent (`n`).
    pub num_eqs: u32,
    /// Whether the target `x₃` decomposes over `V` (brute-force checked).
    /// Non-decomposable ⇒ the system is unsatisfiable ⇒ a refutation
    /// exists.
    pub decomposable: bool,
    /// Per-degree Macaulay measurements.
    pub per_degree: Vec<PcMeasurement>,
    /// Smallest `D` with a non-trivial fall (`rank < rows`, `rank < cols`)
    /// — the operational first-fall degree, comparable to `ffd_harness`.
    pub first_fall: Option<u32>,
    /// Smallest `D` at which the constant `1` enters the row-space — the
    /// refutation degree `D*` (≈ last fall degree / solving degree).
    /// `None` if no refutation up to `d_max` (expected when `decomposable`).
    pub refutation_degree: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct PcMeasurement {
    pub degree: u32,
    pub rows_constructed: u64,
    pub cols: u64,
    pub rank: u64,
    /// `true` iff the constant monomial `1` is in the Macaulay row-space
    /// at this degree (i.e. a degree-`D` refutation exists).
    pub refuted: bool,
}

/// Measure the refutation degree for a single `(n, n', b, x₃)` instance.
pub fn measure_refutation_degree(
    n: u32,
    n_sub: u32,
    irr: &IrreduciblePoly,
    b: &F2mElement,
    x3: &F2mElement,
    d_max: u32,
) -> PcRow {
    assert!(n_sub >= 1 && n_sub <= n, "need 1 ≤ n' ≤ n");
    let full_eqs = weil_descend_s3(n, irr, b, x3);
    let eqs = restrict_to_subspace(&full_eqs, n, n_sub);
    let num_vars = 2 * n_sub;
    let num_eqs = eqs.len() as u32;
    let decomposable = is_decomposable(n, n_sub, irr, b, x3);

    let (first_fall, refutation_degree, per_degree) = refutation_scan(&eqs, num_vars, d_max);

    PcRow {
        n,
        n_sub,
        num_vars,
        num_eqs,
        decomposable,
        per_degree,
        first_fall,
        refutation_degree,
    }
}

/// Scan Macaulay degrees `2..=d_max` of an arbitrary `F_2`-quadratic system
/// `eqs` in `num_vars` Boolean variables, returning
/// `(first_fall, refutation_degree, per_degree)`:
///
/// - `first_fall` = smallest `D` with `rank < rows` and `rank < cols`
///   (operational first-fall onset), and
/// - `refutation_degree` = smallest `D` at which the constant `1` enters
///   the row-space (the PC / last-fall refutation degree `D*`).
///
/// Factored out of [`measure_refutation_degree`] so callers that build the
/// descended system differently — e.g. EXP-E's arbitrary factor-base
/// subspace via [`crate::cryptanalysis::descent_lowgamma`] — can reuse the
/// exact same measurement.
pub fn refutation_scan(
    eqs: &[F2BoolPoly],
    num_vars: u32,
    d_max: u32,
) -> (Option<u32>, Option<u32>, Vec<PcMeasurement>) {
    let mut per_degree = Vec::new();
    let mut first_fall: Option<u32> = None;
    let mut refutation_degree: Option<u32> = None;

    for d in 2..=d_max {
        // Dense single-pass: one echelon reduction yields both the rank and
        // the refutation test (1 ∈ row-space). Benchmarked against the
        // sparse backend (`sparse_rank_and_refute`) at 2n' ∈ {12,…,18}: the
        // dense bit-packed path WINS decisively (e.g. 23 s vs 391 s at
        // 2n'=16) because Macaulay matrices densify under fill-in at degree
        // ≥ 5, where 64-bit-wide XOR beats element-wise symmetric difference.
        // See `build_macaulay_rows_sparse` for that (negative) result.
        let (mut rows, cols, rows_constructed) = build_macaulay_rows(eqs, num_vars, d);
        let (rank_us, refuted) = rank_and_refute(&mut rows, cols);
        let rank = rank_us as u64;

        let nontrivial_syzygy = rank < rows_constructed;
        let not_saturated = rank < cols as u64;
        if first_fall.is_none() && nontrivial_syzygy && not_saturated {
            first_fall = Some(d);
        }
        if refutation_degree.is_none() && refuted {
            refutation_degree = Some(d);
        }

        per_degree.push(PcMeasurement {
            degree: d,
            rows_constructed,
            cols: cols as u64,
            rank,
            refuted,
        });

        // Once refuted, deeper degrees only re-confirm it — stop early.
        if refutation_degree.is_some() {
            break;
        }
    }
    (first_fall, refutation_degree, per_degree)
}

/// Run the refutation-degree sweep over `n_range`.  For each `n` the
/// factor-base dimension is `n' = ⌊n/2⌋` (so `|V|² ≈ 2ⁿ` and roughly half
/// the targets are non-decomposable), and a **non-decomposable** target is
/// found by scanning random `x₃` (so every reported `D*` is a genuine
/// refutation).  `d_max` caps the Macaulay degree; `seed` makes the random
/// `(b, x₃)` choices reproducible.
pub fn run_pc_sweep(n_range: std::ops::RangeInclusive<u32>, d_max: u32, seed: u64) -> Vec<PcRow> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut out = Vec::new();
    for n in n_range {
        let n_sub = (n / 2).max(1);
        let irr = choose_irreducible(n);
        let b = random_nonzero_f2m(&mut rng, n);
        // Find a non-decomposable target (bounded attempts; fall back to
        // the last candidate if the field is too small to have one).
        let mut x3 = random_nonzero_f2m(&mut rng, n);
        for _ in 0..256 {
            if !is_decomposable(n, n_sub, &irr, &b, &x3) {
                break;
            }
            x3 = random_nonzero_f2m(&mut rng, n);
        }
        out.push(measure_refutation_degree(n, n_sub, &irr, &b, &x3, d_max));
    }
    out
}

// ── Subspace restriction ────────────────────────────────────────────

/// Restrict a Weil-descent system over the full `2n` bit-variables to the
/// factor-base subspace `V = span{z⁰, …, z^{n'-1}}` by setting the high
/// bits of `X₁` and `X₂` (positions `n'..n`) to zero, then re-indexing the
/// surviving `2n'` variables as `[X₁ low bits ‖ X₂ low bits]`.
///
/// This is a literal `x_i ← 0` substitution: any monomial that touches a
/// dropped variable vanishes.
pub fn restrict_to_subspace(eqs: &[F2BoolPoly], n: u32, n_sub: u32) -> Vec<F2BoolPoly> {
    let old_vars = 2 * n;
    let new_vars = 2 * n_sub;
    eqs.iter()
        .map(|eq| {
            let mut out = F2BoolPoly::zero(new_vars);
            // Constant term.
            out.coeffs[0] ^= eq.coeffs[0];
            // Linear terms.
            for i in 0..old_vars {
                if eq.coeffs[1 + i as usize] {
                    if let Some(m) = map_var(i, n, n_sub) {
                        out.coeffs[1 + m as usize] ^= true;
                    }
                }
            }
            // Quadratic terms.
            for i in 0..old_vars {
                for j in (i + 1)..old_vars {
                    let idx = quad_monomial_index(i, j, old_vars);
                    if idx < eq.coeffs.len() && eq.coeffs[idx] {
                        if let (Some(mi), Some(mj)) = (map_var(i, n, n_sub), map_var(j, n, n_sub)) {
                            // mi != mj because the var map is injective.
                            let (a, c) = if mi < mj { (mi, mj) } else { (mj, mi) };
                            let nidx = quad_monomial_index(a, c, new_vars);
                            out.coeffs[nidx] ^= true;
                        }
                    }
                }
            }
            out
        })
        .collect()
}

/// Map an old bit-variable index (in the `2n`-variable layout
/// `[X₁ bits 0..n ‖ X₂ bits 0..n]`) to its index in the restricted
/// `2n'`-variable layout, or `None` if it is a dropped high bit.
fn map_var(old: u32, n: u32, n_sub: u32) -> Option<u32> {
    if old < n_sub {
        Some(old) // X₁ low bit
    } else if old < n {
        None // X₁ high bit → forced 0
    } else if old < n + n_sub {
        Some(n_sub + (old - n)) // X₂ low bit
    } else {
        None // X₂ high bit → forced 0
    }
}

// ── Decomposability oracle (brute force over V × V) ─────────────────

/// Does `x₃` admit a decomposition `S₃(X₁, X₂, x₃) = 0` with both
/// `X₁, X₂ ∈ V = span{z⁰, …, z^{n'-1}}`?  Brute force over `|V|² = 2^{2n'}`
/// pairs — cheap for the `n' ≤ 6` regime this harness targets.
pub fn is_decomposable(
    n: u32,
    n_sub: u32,
    irr: &IrreduciblePoly,
    b: &F2mElement,
    x3: &F2mElement,
) -> bool {
    let span = 1u32 << n_sub;
    for mask1 in 0..span {
        let x1 = subspace_element(mask1, n);
        for mask2 in 0..span {
            let x2 = subspace_element(mask2, n);
            if binary_semaev_s3(&x1, &x2, x3, b, irr).is_zero() {
                return true;
            }
        }
    }
    false
}

/// Build the `F_{2ⁿ}` element whose low `n'` bits are given by `mask`
/// (bit `k` of `mask` sets coordinate `z^k`).
fn subspace_element(mask: u32, n: u32) -> F2mElement {
    let positions: Vec<u32> = (0..32).filter(|k| (mask >> k) & 1 == 1).collect();
    F2mElement::from_bit_positions(&positions, n)
}

// ── Refutation test: is the unit `1` in the Macaulay row-space? ─────

/// Test whether the constant monomial `1` (column 0) lies in the row-space
/// of the Macaulay matrix.  Equivalent to: appending the unit row `e₀`
/// does **not** increase the rank.  `base_rank` is the already-computed
/// rank of `rows`, so this costs one extra rank reduction.
pub fn e0_in_rowspace(rows: &[Vec<u64>], cols: usize, base_rank: usize) -> bool {
    if rows.is_empty() || cols == 0 {
        return false;
    }
    let words = cols.div_ceil(64);
    let mut augmented = rows.to_vec();
    let mut e0 = vec![0u64; words];
    e0[0] = 1; // column 0 = the constant monomial `1`
    augmented.push(e0);
    let aug_rank = f2_rank(&mut augmented, cols);
    aug_rank == base_rank
}

/// Combined rank + refutation in a **single** reduction pass.
///
/// `refutation_scan` previously computed the Macaulay rank with `f2_rank`
/// and then re-reduced an augmented copy in `e0_in_rowspace` — two full
/// Gaussian eliminations plus two clones of the (large) row set per degree.
/// This routine row-reduces `rows` to echelon form **once**, then reduces
/// the unit vector `e₀` against that basis. The system is refuted (`1` ∈
/// row-space) iff `e₀` reduces to zero.
///
/// The echelon form comes from [`gf2_elim::echelon_counted`], the Four
/// Russians kernel, rather than a column-at-a-time loop: that loop spent
/// its time testing one bit of every remaining row per column and XORing
/// one pivot at a time into full-width rows.  The test needs only *an*
/// echelon basis.  Reducing `e₀` in pivot order clears every pivot column
/// of it, and a nonzero vector of the row space leads at a pivot column,
/// so `e₀` reduces to zero exactly when it lies in the row space, whichever
/// basis it is reduced against.  The rank and the pivot columns are fixed
/// by the row space, so they do not depend on the basis either.
///
/// Returns `(rank, refuted)`. Consumes `rows`: on return `rows[..rank]` is a
/// row echelon basis of the row space (leading columns strictly increasing,
/// so they are the pivot columns) and the rest are zero.  Which echelon
/// basis is not specified; it is not the one the column loop left.  Callers
/// that still need the original matrix should clone first, but the scan does
/// not, which is the point. Result is identical to
/// `(f2_rank(rows), e0_in_rowspace(rows, …))`; this is asserted in tests.
pub fn rank_and_refute(rows: &mut [Vec<u64>], cols: usize) -> (usize, bool) {
    if rows.is_empty() || cols == 0 {
        return (0, false);
    }
    let rank = gf2_elim::echelon_counted(rows, cols, &mut 0);

    // Reduce e₀ (only column 0 set) against the echelon basis: for each
    // pivot whose column is currently set in the working vector, XOR in the
    // pivot row. `1` ∈ row-space ⇔ the result is zero.  A pivot row is zero
    // before its leading word, and the leading words never decrease down the
    // basis, so each lead is found from the previous one's word and each
    // XOR starts at its own.
    let words = cols.div_ceil(64);
    let mut w = vec![0u64; words];
    w[0] = 1;
    let mut word = 0;
    for row in &rows[..rank] {
        while row[word] == 0 {
            word += 1;
        }
        if w[word] >> row[word].trailing_zeros() & 1 != 0 {
            for (wi, p) in w[word..].iter_mut().zip(&row[word..]) {
                *wi ^= *p;
            }
        }
    }
    let refuted = w.iter().all(|&x| x == 0);
    (rank, refuted)
}

/// XOR (symmetric difference) of two sorted, deduplicated index lists.
/// Keeps elements appearing in exactly one input — the `F_2` row sum.
fn symdiff(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

/// Sparse `F_2` rank + refutation, the elimination half of the sparse
/// backend. `rows` are sorted column-index lists (from
/// [`build_macaulay_rows_sparse`]). Performs sparse row-echelon reduction:
/// each row is XOR-reduced against the stored pivots (keyed by leading
/// column) until it either gains a fresh leading column (→ new pivot) or
/// vanishes. The leading column strictly increases at every reduction step,
/// so the process terminates.
///
/// Returns `(rank, refuted)` where `refuted` is whether the unit `e₀`
/// (column 0, the constant `1`) lies in the row-space — tested by reducing
/// the singleton `[0]` against the pivots. The result is identical to the
/// dense `(f2_rank, e0_in_rowspace)` / [`rank_and_refute`] path (the rank
/// of a matrix is representation-independent); this is asserted in tests.
pub fn sparse_rank_and_refute(rows: Vec<Vec<u32>>) -> (usize, bool) {
    use std::collections::HashMap;
    // pivots[leading_col] = the (reduced) pivot row whose minimum index is
    // `leading_col`.
    let mut pivots: HashMap<u32, Vec<u32>> = HashMap::new();
    for mut row in rows {
        while let Some(&lead) = row.first() {
            match pivots.get(&lead) {
                Some(p) => {
                    row = symdiff(&row, p);
                    // `lead` is now cancelled; the new leading index is > lead.
                }
                None => {
                    pivots.insert(lead, row);
                    break;
                }
            }
        }
        // If `row` emptied, it was linearly dependent — no new pivot.
    }
    let rank = pivots.len();

    // Reduce e₀ = [0] against the pivot basis.
    let mut w: Vec<u32> = vec![0];
    while let Some(&lead) = w.first() {
        match pivots.get(&lead) {
            Some(p) => w = symdiff(&w, p),
            None => break,
        }
    }
    let refuted = w.is_empty();
    (rank, refuted)
}

// ── Pretty printing ─────────────────────────────────────────────────

pub fn print_pc_sweep(rows: &[PcRow]) {
    println!();
    println!(
        "{:>3} {:>3} {:>5} {:>4} {:>6} | {:>9} {:>9}",
        "n", "n'", "vars", "eqs", "decmp", "first-fall", "D* (refute)"
    );
    println!("{}", "─".repeat(58));
    for r in rows {
        let ff = r
            .first_fall
            .map(|d| d.to_string())
            .unwrap_or_else(|| "—".into());
        let ds = r
            .refutation_degree
            .map(|d| d.to_string())
            .unwrap_or_else(|| "none".into());
        println!(
            "{:>3} {:>3} {:>5} {:>4} {:>6} | {:>9} {:>9}",
            r.n,
            r.n_sub,
            r.num_vars,
            r.num_eqs,
            if r.decomposable { "yes" } else { "no" },
            ff,
            ds
        );
    }
    println!();
    println!(
        "Reading: `first-fall` is the syzygy onset measured by ffd_harness;\n\
         `D*` is the Polynomial-Calculus refutation degree (≈ last fall\n\
         degree). Prediction #1: D* climbs with n while first-fall stays\n\
         roughly flat. All `D*` rows are non-decomposable (genuinely\n\
         unsatisfiable) instances."
    );
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn irr(n: u32) -> IrreduciblePoly {
        choose_irreducible(n)
    }

    /// The single-pass `rank_and_refute` must agree exactly with the old
    /// two-call path `(f2_rank, e0_in_rowspace)` on real Macaulay matrices,
    /// across degrees and across both refuting and non-refuting targets.
    #[test]
    fn rank_and_refute_matches_two_call_path() {
        use crate::cryptanalysis::ffd_harness::build_macaulay_rows;
        let n = 6;
        let n_sub = 3;
        let irr = irr(n);
        let mut checked_refute = false;
        let mut checked_nonrefute = false;
        // A handful of targets to hit both refuting and non-refuting cases.
        for (bm, xm) in [
            (0b011u32, 0b101u32),
            (0b110, 0b011),
            (0b101, 0b010),
            (0b111, 0b100),
        ] {
            let b = F2mElement::from_bit_positions(
                &(0..n).filter(|k| (bm >> k) & 1 == 1).collect::<Vec<_>>(),
                n,
            );
            let x3 = F2mElement::from_bit_positions(
                &(0..n).filter(|k| (xm >> k) & 1 == 1).collect::<Vec<_>>(),
                n,
            );
            if b.is_zero() || x3.is_zero() {
                continue;
            }
            let full = weil_descend_s3(n, &irr, &b, &x3);
            let eqs = restrict_to_subspace(&full, n, n_sub);
            for d in 2..=2 * n_sub {
                let (rows, cols, _) = build_macaulay_rows(&eqs, 2 * n_sub, d);
                if rows.is_empty() {
                    continue;
                }
                let mut for_rank = rows.clone();
                let want_rank = f2_rank(&mut for_rank, cols);
                let want_refuted = e0_in_rowspace(&rows, cols, want_rank);
                let mut mine = rows;
                let (got_rank, got_refuted) = rank_and_refute(&mut mine, cols);
                assert_eq!(got_rank, want_rank, "rank mismatch at d={d}");
                assert_eq!(got_refuted, want_refuted, "refute mismatch at d={d}");
                if got_refuted {
                    checked_refute = true;
                } else {
                    checked_nonrefute = true;
                }
            }
        }
        assert!(checked_refute, "test never hit a refuting case");
        assert!(checked_nonrefute, "test never hit a non-refuting case");
    }

    /// The column-at-a-time echelon loop `rank_and_refute` ran before it
    /// moved onto `gf2_elim`, kept as the reference: `(rank, refuted,
    /// pivot columns)`.
    fn rank_and_refute_by_columns(rows: &mut [Vec<u64>], cols: usize) -> (usize, bool, Vec<usize>) {
        let mut pivots: Vec<(usize, usize)> = Vec::new();
        let mut row = 0usize;
        for col in 0..cols {
            let (word, mask) = (col / 64, 1u64 << (col % 64));
            let Some(pivot) = (row..rows.len()).find(|&r| rows[r][word] & mask != 0) else {
                continue;
            };
            rows.swap(row, pivot);
            for r in (row + 1)..rows.len() {
                if rows[r][word] & mask != 0 {
                    let (lo, hi) = rows.split_at_mut(r);
                    for (rr, p) in hi[0].iter_mut().zip(lo[row].iter()) {
                        *rr ^= *p;
                    }
                }
            }
            pivots.push((row, col));
            row += 1;
            if row >= rows.len() {
                break;
            }
        }
        let mut w = vec![0u64; cols.div_ceil(64)];
        w[0] = 1;
        for &(prow, pcol) in &pivots {
            if w[pcol / 64] & (1u64 << (pcol % 64)) != 0 {
                for (wi, p) in w.iter_mut().zip(rows[prow].iter()) {
                    *wi ^= *p;
                }
            }
        }
        let refuted = w.iter().all(|&x| x == 0);
        (
            pivots.len(),
            refuted,
            pivots.iter().map(|&(_, c)| c).collect(),
        )
    }

    /// `rank_and_refute` on the Four Russians kernel returns what the column
    /// loop did — rank, verdict and pivot columns — on random matrices of
    /// full and deficient rank, with and without `e₀` in the row space, and
    /// leaves an echelon basis with those pivots followed by zero rows.
    #[test]
    fn rank_and_refute_matches_the_column_loop_on_random_matrices() {
        use rand::Rng;
        let mut rng = StdRng::seed_from_u64(0x5EED_0E0);
        let (mut refuting, mut non_refuting) = (0, 0);
        for &(n_rows, cols) in &[
            (1usize, 1usize),
            (3, 5),
            (40, 40),
            (70, 130),
            (130, 70),
            (200, 257),
            (257, 200),
            (90, 700),
        ] {
            let words = cols.div_ceil(64);
            let random_row = |rng: &mut StdRng, density: f64| {
                let mut row = vec![0u64; words];
                for c in 0..cols {
                    if rng.gen_bool(density) {
                        row[c / 64] |= 1 << (c % 64);
                    }
                }
                row
            };
            for density in [0.02, 0.2, 0.5] {
                for rank_cap in [n_rows, (n_rows / 3).max(1)] {
                    // Rows are random sums of a basis of `rank_cap` rows,
                    // one of which is `e₀` in half the draws.
                    let mut basis: Vec<Vec<u64>> = (0..rank_cap)
                        .map(|_| random_row(&mut rng, density))
                        .collect();
                    if rng.gen_bool(0.5) {
                        basis[0] = vec![0u64; words];
                        basis[0][0] = 1;
                    }
                    let m: Vec<Vec<u64>> = (0..n_rows)
                        .map(|_| {
                            let mut row = vec![0u64; words];
                            for b in &basis {
                                if rng.gen_bool(0.5) {
                                    for (r, x) in row.iter_mut().zip(b) {
                                        *r ^= x;
                                    }
                                }
                            }
                            row
                        })
                        .collect();
                    let mut want = m.clone();
                    let (rank, refuted, pivots) = rank_and_refute_by_columns(&mut want, cols);
                    let mut got = m;
                    assert_eq!(
                        rank_and_refute(&mut got, cols),
                        (rank, refuted),
                        "{n_rows}x{cols} density {density} rank cap {rank_cap}"
                    );
                    let leads: Vec<usize> = got[..rank]
                        .iter()
                        .map(|row| {
                            let k = row.iter().position(|&x| x != 0).unwrap();
                            k * 64 + row[k].trailing_zeros() as usize
                        })
                        .collect();
                    assert_eq!(leads, pivots);
                    assert!(got[rank..].iter().all(|row| row.iter().all(|&x| x == 0)));
                    if refuted {
                        refuting += 1;
                    } else {
                        non_refuting += 1;
                    }
                }
            }
        }
        assert!(
            refuting > 5 && non_refuting > 5,
            "{refuting} / {non_refuting}"
        );
    }

    /// The same comparison at the shapes the random test above does not
    /// reach: widths one either side of a word boundary, a single row, a
    /// zero matrix, rows that are all `e₀`, duplicated rows, and row counts
    /// below, at and above the width.  Empty inputs keep their early return.
    #[test]
    fn rank_and_refute_agrees_with_the_column_loop_at_word_boundaries() {
        use rand::Rng;
        let mut rng = StdRng::seed_from_u64(0xB0DA_5);
        assert_eq!(rank_and_refute(&mut [], 5), (0, false));
        assert_eq!(rank_and_refute(&mut [vec![0u64; 1]], 0), (0, false));
        let (mut refuting, mut non_refuting, mut deficient) = (0, 0, 0);
        for &cols in &[1usize, 2, 63, 64, 65, 127, 128, 129, 191, 192, 193] {
            let words = cols.div_ceil(64);
            for &n_rows in &[1usize, 2, 3, 10, 64, 65, 130, 200] {
                for kind in 0..6 {
                    for _ in 0..4 {
                        let density = [0.01, 0.1, 0.5, 0.95][rng.gen_range(0..4)];
                        let random_row = |rng: &mut StdRng| {
                            let mut row = vec![0u64; words];
                            for c in 0..cols {
                                if rng.gen_bool(density) {
                                    row[c / 64] |= 1 << (c % 64);
                                }
                            }
                            row
                        };
                        let e0 = {
                            let mut row = vec![0u64; words];
                            row[0] = 1;
                            row
                        };
                        let m: Vec<Vec<u64>> = match kind {
                            0 => (0..n_rows).map(|_| random_row(&mut rng)).collect(),
                            1 => vec![vec![0u64; words]; n_rows],
                            2 => vec![e0.clone(); n_rows],
                            3 => {
                                let one = random_row(&mut rng);
                                vec![one; n_rows]
                            }
                            4 => {
                                // each row is a random sum of a few random rows
                                // and, half the time, of e₀
                                let basis: Vec<Vec<u64>> =
                                    (0..3).map(|_| random_row(&mut rng)).collect();
                                (0..n_rows)
                                    .map(|_| {
                                        let mut row = vec![0u64; words];
                                        for b in basis.iter().chain(std::iter::once(&e0)) {
                                            if rng.gen_bool(0.5) {
                                                for (r, x) in row.iter_mut().zip(b) {
                                                    *r ^= x;
                                                }
                                            }
                                        }
                                        row
                                    })
                                    .collect()
                            }
                            _ => {
                                // e₀ among otherwise random rows
                                let mut m: Vec<Vec<u64>> =
                                    (0..n_rows).map(|_| random_row(&mut rng)).collect();
                                let at = rng.gen_range(0..n_rows);
                                m[at] = e0.clone();
                                m
                            }
                        };
                        let mut want = m.clone();
                        let (rank, refuted, pivots) = rank_and_refute_by_columns(&mut want, cols);
                        let mut got = m;
                        assert_eq!(
                            rank_and_refute(&mut got, cols),
                            (rank, refuted),
                            "{n_rows}x{cols} kind {kind}"
                        );
                        let leads: Vec<usize> = got[..rank]
                            .iter()
                            .map(|row| {
                                let k = row.iter().position(|&x| x != 0).unwrap();
                                k * 64 + row[k].trailing_zeros() as usize
                            })
                            .collect();
                        assert_eq!(leads, pivots, "{n_rows}x{cols} kind {kind}");
                        assert!(got[rank..].iter().all(|r| r.iter().all(|&x| x == 0)));
                        refuting += refuted as u32;
                        non_refuting += !refuted as u32;
                        deficient += (rank < n_rows.min(cols)) as u32;
                    }
                }
            }
        }
        assert!(
            refuting > 50 && non_refuting > 50 && deficient > 50,
            "{refuting} / {non_refuting} / {deficient}"
        );
    }

    /// The sparse backend (`build_macaulay_rows_sparse` +
    /// `sparse_rank_and_refute`) must produce exactly the same `(rank,
    /// refuted)` as the dense path on real Macaulay matrices, across degrees
    /// and both refuting / non-refuting targets. Also checks the sparse row
    /// supports equal the dense row supports.
    #[test]
    fn sparse_backend_matches_dense() {
        use crate::cryptanalysis::ffd_harness::{build_macaulay_rows, build_macaulay_rows_sparse};
        let n = 6;
        let n_sub = 3;
        let irr = irr(n);
        let mut hit_refute = false;
        let mut hit_nonrefute = false;
        for (bm, xm) in [
            (0b011u32, 0b101u32),
            (0b110, 0b011),
            (0b101, 0b010),
            (0b111, 0b100),
        ] {
            let b = F2mElement::from_bit_positions(
                &(0..n).filter(|k| (bm >> k) & 1 == 1).collect::<Vec<_>>(),
                n,
            );
            let x3 = F2mElement::from_bit_positions(
                &(0..n).filter(|k| (xm >> k) & 1 == 1).collect::<Vec<_>>(),
                n,
            );
            if b.is_zero() || x3.is_zero() {
                continue;
            }
            let eqs = restrict_to_subspace(&weil_descend_s3(n, &irr, &b, &x3), n, n_sub);
            for d in 2..=2 * n_sub {
                let (dense, cols, dn) = build_macaulay_rows(&eqs, 2 * n_sub, d);
                let (sparse, scols, sn) = build_macaulay_rows_sparse(&eqs, 2 * n_sub, d);
                assert_eq!(cols, scols);
                assert_eq!(dn, sn, "row count differs at d={d}");
                // Row supports match (dense packed bits ↔ sparse indices).
                for (drow, srow) in dense.iter().zip(sparse.iter()) {
                    let mut want: Vec<u32> = Vec::new();
                    for (w, word) in drow.iter().enumerate() {
                        for bit in 0..64 {
                            if word & (1u64 << bit) != 0 {
                                want.push((w * 64 + bit) as u32);
                            }
                        }
                    }
                    assert_eq!(srow, &want, "sparse support differs at d={d}");
                }
                let mut for_rank = dense.clone();
                let want_rank = f2_rank(&mut for_rank, cols);
                let want_refuted = e0_in_rowspace(&dense, cols, want_rank);
                let (got_rank, got_refuted) = sparse_rank_and_refute(sparse);
                assert_eq!(got_rank, want_rank, "sparse rank differs at d={d}");
                assert_eq!(got_refuted, want_refuted, "sparse refute differs at d={d}");
                if got_refuted {
                    hit_refute = true;
                } else {
                    hit_nonrefute = true;
                }
            }
        }
        assert!(hit_refute && hit_nonrefute);
    }

    /// Restriction drops every monomial that touches a high bit and
    /// re-indexes into `2n'` variables.
    #[test]
    fn restriction_drops_high_bit_monomials() {
        let n = 4;
        let n_sub = 2;
        let irr = irr(n);
        let b = F2mElement::from_bit_positions(&[0, 1], n);
        let x3 = F2mElement::from_bit_positions(&[1, 2], n);
        let full = weil_descend_s3(n, &irr, &b, &x3);
        let restricted = restrict_to_subspace(&full, n, n_sub);
        // Same number of equations, smaller variable space.
        assert_eq!(restricted.len(), full.len());
        let new_len = F2BoolPoly::zero(2 * n_sub).coeffs.len();
        for eq in &restricted {
            assert_eq!(eq.coeffs.len(), new_len);
        }
    }

    /// `map_var` is injective on the kept indices and `None` on dropped
    /// high bits.
    #[test]
    fn map_var_layout() {
        let (n, n_sub) = (5, 2);
        // X₁ low bits 0,1 → 0,1 ; X₂ low bits 5,6 → 2,3.
        assert_eq!(map_var(0, n, n_sub), Some(0));
        assert_eq!(map_var(1, n, n_sub), Some(1));
        assert_eq!(map_var(2, n, n_sub), None); // X₁ high bit
        assert_eq!(map_var(4, n, n_sub), None);
        assert_eq!(map_var(5, n, n_sub), Some(2)); // X₂ bit 0
        assert_eq!(map_var(6, n, n_sub), Some(3));
        assert_eq!(map_var(7, n, n_sub), None); // X₂ high bit
    }

    /// **The calibration theorem**: over a small field, a target refutes
    /// (`1` enters the row-space) **iff** it is non-decomposable over `V`.
    /// This validates that `D*` measures the right thing.
    #[test]
    fn refutation_iff_nondecomposable() {
        let n = 4;
        let n_sub = 2; // |V| = 4, |V|² = 16 pairs
        let irr = irr(n);
        let b = F2mElement::from_bit_positions(&[0], n);
        let d_max = 2 * n_sub + 1; // enough degree to saturate at n'=2

        let mut checked = 0;
        for t in 1u32..(1u32 << n) {
            let x3 = subspace_element_full(t, n);
            if x3.is_zero() {
                continue;
            }
            let row = measure_refutation_degree(n, n_sub, &irr, &b, &x3, d_max);
            // The equivalence: decomposable ⇔ NOT refuted.
            assert_eq!(
                row.decomposable,
                row.refutation_degree.is_none(),
                "x3=#{t}: decomposable={} but refutation={:?}",
                row.decomposable,
                row.refutation_degree
            );
            checked += 1;
        }
        assert!(checked > 0, "no targets were checked");
    }

    /// Build an `F_{2ⁿ}` element from the low bits of `t` (all `n` bits,
    /// not just the subspace) — used to range over every target.
    fn subspace_element_full(t: u32, n: u32) -> F2mElement {
        let positions: Vec<u32> = (0..n).filter(|k| (t >> k) & 1 == 1).collect();
        F2mElement::from_bit_positions(&positions, n)
    }

    /// A non-decomposable target produces a finite refutation degree
    /// `D* ≥ 2`, and the decomposability oracle agrees.
    #[test]
    fn nondecomposable_target_refutes() {
        let n = 4;
        let n_sub = 2;
        let irr = irr(n);
        let b = F2mElement::from_bit_positions(&[0], n);
        // Search for a non-decomposable target.
        let mut found = None;
        for t in 1u32..(1u32 << n) {
            let x3 = subspace_element_full(t, n);
            if x3.is_zero() {
                continue;
            }
            if !is_decomposable(n, n_sub, &irr, &b, &x3) {
                found = Some(x3);
                break;
            }
        }
        let x3 = found.expect("expected a non-decomposable target at n=4, n'=2");
        let row = measure_refutation_degree(n, n_sub, &irr, &b, &x3, 2 * n_sub + 1);
        assert!(!row.decomposable);
        let dstar = row.refutation_degree.expect("non-decomposable must refute");
        assert!(dstar >= 2);
    }

    /// `e0_in_rowspace` is correct on tiny hand-built matrices.
    #[test]
    fn e0_membership_basic() {
        // cols = 3. Rows {110, 011}: span does NOT contain e0 = 100
        // (every combination has even-or-paired structure: 110, 011,
        // 110^011 = 101 — none is 100).
        let rows = vec![vec![0b110u64], vec![0b011u64]];
        let base = f2_rank(&mut rows.clone(), 3);
        assert!(!e0_in_rowspace(&rows, 3, base));
        // Add row 010: now 110 ^ 010 = 100 = e0 is in the span.
        let rows2 = vec![vec![0b110u64], vec![0b011u64], vec![0b010u64]];
        let base2 = f2_rank(&mut rows2.clone(), 3);
        assert!(e0_in_rowspace(&rows2, 3, base2));
    }

    /// End-to-end sweep runs and every reported `D*` is from a
    /// non-decomposable (unsatisfiable) instance.
    #[test]
    fn pc_sweep_runs_and_is_nondecomposable() {
        let rows = run_pc_sweep(4..=6, 7, 0x_DEC0_DE_u64);
        assert_eq!(rows.len(), 3);
        for r in &rows {
            assert_eq!(r.num_eqs, r.n);
            assert_eq!(r.num_vars, 2 * r.n_sub);
            if let Some(_d) = r.refutation_degree {
                // A genuine refutation requires an unsatisfiable instance.
                assert!(!r.decomposable, "refuted a decomposable target?!");
            }
        }
    }
}
