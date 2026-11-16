//! **Joux–Vitse trace record and replay for the dense `F_2` Macaulay step.**
//!
//! In the index-calculus relation search thousands of Boolean systems of
//! one shape (same monomial support, different coefficients) are reduced one
//! after another by [`matrix_f4_f2`].  Joux and Vitse ("A variant of the F4
//! algorithm", CT-RSA 2011) observe that which products `t · f_i` reduce to
//! zero is, to a large extent, a property of the shape (Koszul and field-equation
//! syzygies), not of the coefficients.  They record that on the first system
//! and replay only the useful rows on the later ones.
//!
//! * [`matrix_f4_f2_record`] runs the ordinary kernel, then selects the rows
//!   that survive elimination.  The rows are taken in degree-major order
//!   (multiplier degree ascending, then input index) and a row survives iff it
//!   is independent of the rows before it, so survivors are the lexicographically
//!   first basis of the row space.  The [`F4Trace`] stores them as
//!   `(input index, multiplier mask)` pairs: no coefficient is stored.
//! * [`matrix_f4_f2_replay`] builds only the surviving products of the new
//!   system and eliminates them.  It returns `None` (caller falls back to the
//!   full run) unless the shape matches (variable count, degree, input count,
//!   per-input degree), every recorded product is non-empty, all survivors stay
//!   independent (rank equals the trace's), and the set of leading monomials of
//!   the reduced basis equals the recorded one.
//!
//! **Soundness.**  Every returned polynomial lies in the ideal, so a returned
//! constant `1` is a genuine certificate.  What replay cannot know cheaply is
//! that no *dropped* row is independent of the kept ones in the new system
//! (an accidental dependency of the recorded system only).  Then the replayed
//! basis would be a proper subspace of the full one.  Plain replay accepts
//! that risk, as Joux–Vitse do; [`matrix_f4_f2_replay_counted`] with
//! `audit = k` closes it up to probability `2^-k` by reducing `k` random
//! combinations of the dropped rows against the replayed basis, in counted
//! word operations.
//!
//! Counts are 64-bit word XORs of the elimination, the unit of
//! [`matrix_f4_f2_counted`]; building rows is not counted, in either arm.
//!
//! ## Measured savings
//!
//! Word XORs of the elimination, trace recorded on draws 0-2 (union of the
//! three, `F4Trace::merge`), replayed on the next 20 draws, replay audited with
//! [`DEFAULT_AUDIT`] = 16 combinations
//! (`f4_trace_counted_savings_table`, release, 0.13 s).  Fixed seeds; one run.
//!
//! | family (degree 4) | accepted of 20 | full, accepted draws | replay audited | replay unaudited | spent on 9/18/14 refusals | full, all 20 | record, 3 draws |
//! |:--|--:|--:|--:|--:|--:|--:|--:|
//! | generic dense, 10 var, 10 quadrics | 11 | 1,681,231 | 1,265,565 (-24.7%) | 1,136,634 (-32.4%) | 820,038 | 3,059,406 | 493,436 |
//! | Koblitz `m=3, n=5, l=2` (11 var), target varies | 2 | 88,254 | 88,014 (-0.3%) | 84,429 (-4.3%) | 511,100 | 1,221,959 | 25,601 |
//! | Koblitz `m=3, n=7, l=2` (13 var), target varies | 6 | 1,460,717 | 1,413,008 (-3.3%) | 1,349,188 (-7.6%) | 2,895,863 | 4,990,405 | 150,250 |
//!
//! Reading the table:
//!
//! * Per accepted draw, replay is cheaper only on the generic family
//!   (-24.7% audited).  On the Weil-descended Koblitz systems the saving is
//!   under 4%: few of the rows that reduce to zero reduce to zero for every
//!   target.
//! * **Net, with the fallbacks, the pipeline loses**: refused replays spend
//!   their elimination before the rank or leading-term check fails.  Generic:
//!   `1,265,565 + 820,038 + (3,059,406 - 1,681,231) = 3,463,778` against
//!   `3,059,406` for always running the full kernel (+13%), before the
//!   recording cost.  Replay pays only at an acceptance rate well above the
//!   55% seen here, i.e. with more training draws or a family whose
//!   dependencies are structural rather than accidents of `F_2` (over `F_2`
//!   a recorded basis stays a basis in a fresh system with probability about
//!   0.29 for a random matrix).
//! * A single recording is not enough: replayed alone, it was accepted and
//!   **wrong** on draws where an accidental dependency of the recorded system
//!   was absent from the new one (unaudited, seen in exploration on both
//!   families; the audit refused every one).  Merge three or more recordings
//!   and keep the audit.
//! * Specialising a variable to 0 or 1 changes the input degrees, and all 72
//!   specialised draws were refused (`f4_trace_specialised_*`).
//!
//! This is a stage measurement of the elimination kernel, not an end-to-end
//! `S`; no speedup of the index-calculus pipeline is claimed.
//!
//! Reference: A. Joux, V. Vitse, "A variant of the F4 algorithm", CT-RSA 2011,
//! LNCS 6558, pp. 356-375.

use std::collections::{HashMap, HashSet};

use super::{
    all_variable_mask, charge_word_ops, macaulay_columns, matrix_f4_f2_counted, max_f4_rows,
    monomials_up_to_mask, pack_rows, poly_from_column_order, row_monos, rref_f2_counted,
};
use crate::cryptanalysis::pq_groebner_f2::F2BoolPoly;

/// One degree step of the recorded run: the products whose multiplier has
/// degree `multiplier_degree`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F4TraceStep {
    pub multiplier_degree: u32,
    /// Non-empty products offered at this step.
    pub rows: usize,
    /// The products that were independent of everything before them:
    /// `(input index, multiplier monomial mask)`.  The other `rows -
    /// survivors.len()` reduced to zero.
    pub survivors: Vec<(u32, u64)>,
}

/// What a recorded run says about a system shape; independent of coefficients.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F4Trace {
    pub n_vars: usize,
    pub degree: u32,
    /// Largest term degree of each input polynomial, the shape fingerprint.
    pub poly_degrees: Vec<u32>,
    pub steps: Vec<F4TraceStep>,
    /// Rank of the Macaulay matrix (total survivors).
    pub rank: usize,
    /// Number of leading monomials of the reduced basis of each total degree
    /// (index = degree, `0..=degree`): the pivot structure.  After
    /// [`F4Trace::merge`], the elementwise minimum.
    pub leading_by_degree: Vec<usize>,
    /// Word XORs of the survivor selection pass (the price of recording).
    pub record_word_ops: u64,
}

impl F4Trace {
    /// Fold a second recording of the same shape into this one: a product is
    /// kept if it survived in either run, so a product is dropped only when
    /// it reduced to zero in both (a dependency the shape forces, not one a
    /// single draw's coefficients happened to produce).  The rank becomes
    /// the smaller of the two and the required leading monomials their
    /// intersection.  `false`, leaving `self` unchanged, if the shapes differ.
    pub fn merge(&mut self, other: &F4Trace) -> bool {
        if self.n_vars != other.n_vars
            || self.degree != other.degree
            || self.poly_degrees != other.poly_degrees
        {
            return false;
        }
        for o in &other.steps {
            match self
                .steps
                .iter_mut()
                .find(|s| s.multiplier_degree == o.multiplier_degree)
            {
                Some(s) => {
                    let have: HashSet<(u32, u64)> = s.survivors.iter().copied().collect();
                    s.survivors
                        .extend(o.survivors.iter().filter(|x| !have.contains(x)));
                    s.rows = s.rows.max(o.rows);
                }
                None => self.steps.push(o.clone()),
            }
        }
        self.steps.sort_by_key(|s| s.multiplier_degree);
        self.rank = self.rank.min(other.rank);
        for (a, b) in self
            .leading_by_degree
            .iter_mut()
            .zip(&other.leading_by_degree)
        {
            *a = (*a).min(*b);
        }
        self.record_word_ops += other.record_word_ops;
        true
    }

    pub fn total_rows(&self) -> usize {
        self.steps.iter().map(|s| s.rows).sum()
    }
    pub fn total_survivors(&self) -> usize {
        self.steps.iter().map(|s| s.survivors.len()).sum()
    }
}

fn leading_histogram(masks: impl Iterator<Item = u64>, degree: u32) -> Vec<usize> {
    let mut h = vec![0usize; degree as usize + 1];
    for m in masks {
        h[(m.count_ones() as usize).min(degree as usize)] += 1;
    }
    h
}

fn poly_degree(p: &F2BoolPoly) -> u32 {
    p.terms
        .iter()
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(0)
}

/// The product `mult · p` as ascending monomial masks, odd multiplicities
/// kept (the Boolean ring: colliding terms cancel).
fn product_row(p: &F2BoolPoly, mult: u64) -> Vec<u64> {
    let mut all: Vec<u64> = p.terms.iter().map(|t| t.mask | mult).collect();
    all.sort_unstable();
    let mut out: Vec<u64> = Vec::with_capacity(all.len());
    let mut i = 0;
    while i < all.len() {
        let mut j = i;
        while j < all.len() && all[j] == all[i] {
            j += 1;
        }
        if (j - i) % 2 == 1 {
            out.push(all[i]);
        }
        i = j;
    }
    out
}

/// Every non-empty product in degree-major order, tagged with its identity.
/// `keep` filters the rows that are materialised.  `None` past the row cap.
fn degree_major_rows(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    mut keep: impl FnMut(u32, u64) -> bool,
) -> Option<Vec<(u32, u64, u32, Vec<u64>)>> {
    let mask = all_variable_mask(n_vars);
    let max_gap = polys
        .iter()
        .filter(|p| poly_degree(p) <= degree)
        .map(|p| degree - poly_degree(p))
        .max()?;
    let mults = monomials_up_to_mask(mask, max_gap);
    let cap = max_f4_rows();
    let mut out = Vec::new();
    let mut seen = 0usize;
    for d in 0..=max_gap {
        for &t in mults.iter().filter(|t| t.count_ones() == d) {
            for (i, p) in polys.iter().enumerate() {
                let pd = poly_degree(p);
                if pd > degree || degree - pd < d {
                    continue;
                }
                let row = product_row(p, t);
                if row.is_empty() {
                    continue;
                }
                seen += 1;
                if seen > cap {
                    return None;
                }
                if keep(i as u32, t) {
                    out.push((i as u32, t, d, row));
                }
            }
        }
    }
    Some(out)
}

/// As [`super::matrix_f4_f2`], and also records the trace.  `None` when the
/// matrix exceeds the size limits.
pub fn matrix_f4_f2_record(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
) -> Option<(Vec<F2BoolPoly>, F4Trace)> {
    matrix_f4_f2_record_counted(polys, n_vars, degree).map(|(r, _, t)| (r, t))
}

/// As [`matrix_f4_f2_record`], also returning the kernel's word XORs (the
/// full-run price; the recording pass is in `trace.record_word_ops`).
pub fn matrix_f4_f2_record_counted(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
) -> Option<(Vec<F2BoolPoly>, u64, F4Trace)> {
    let (result, ops) = matrix_f4_f2_counted(polys, n_vars, degree)?;
    let mut trace = F4Trace {
        n_vars,
        degree,
        poly_degrees: polys.iter().map(poly_degree).collect(),
        steps: Vec::new(),
        rank: result.len(),
        leading_by_degree: leading_histogram(
            result
                .iter()
                .filter_map(|p| p.terms.first())
                .map(|t| t.mask),
            degree,
        ),
        record_word_ops: 0,
    };
    if polys.is_empty() {
        return Some((result, ops, trace));
    }
    let rows = match degree_major_rows(polys, n_vars, degree, |_, _| true) {
        Some(r) => r,
        None if polys.iter().all(|p| poly_degree(p) > degree) => Vec::new(),
        None => return None,
    };
    let monos: Vec<Vec<u64>> = rows.iter().map(|r| r.3.clone()).collect();
    let cols = macaulay_columns(&monos)?;
    let words = cols.len().div_ceil(64);
    let matrix = pack_rows(&monos, &cols);
    // Incremental insertion: a row survives iff it is independent of the
    // survivors before it.  Pivot rows are zero left of their leading column.
    let mut pivot_of: Vec<Option<usize>> = vec![None; cols.len()];
    let mut basis: Vec<Vec<u64>> = Vec::new();
    let mut ops_sel = 0u64;
    for (r, mut row) in rows.iter().zip(matrix) {
        let mut w = 0usize;
        let mut kept = false;
        'reduce: loop {
            while w < words && row[w] == 0 {
                w += 1;
            }
            if w == words {
                break;
            }
            let col = w * 64 + row[w].trailing_zeros() as usize;
            match pivot_of[col] {
                Some(p) => {
                    for (a, b) in row[w..].iter_mut().zip(&basis[p][w..]) {
                        *a ^= *b;
                    }
                    ops_sel += (words - w) as u64;
                }
                None => {
                    pivot_of[col] = Some(basis.len());
                    kept = true;
                    break 'reduce;
                }
            }
        }
        if trace.steps.last().map(|s| s.multiplier_degree) != Some(r.2) {
            trace.steps.push(F4TraceStep {
                multiplier_degree: r.2,
                rows: 0,
                survivors: Vec::new(),
            });
        }
        let step = trace.steps.last_mut().unwrap();
        step.rows += 1;
        if kept {
            step.survivors.push((r.0, r.1));
            basis.push(row);
        }
    }
    trace.record_word_ops = ops_sel;
    debug_assert_eq!(trace.total_survivors(), trace.rank);
    Some((result, ops, trace))
}

/// Why a replay was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplayReject {
    /// Variable count, degree, input count or per-input degree differ.
    Shape,
    /// A recorded product is empty in the new system.
    EmptyProduct,
    /// The column or row limits.
    Size,
    /// The survivors are no longer independent (rank below the trace's).
    Rank,
    /// The reduced basis has fewer leading monomials of some degree than the trace.
    Leading,
    /// The audit found a dropped product outside the replayed span.
    Audit,
}

/// Random dropped-row combinations [`matrix_f4_f2_replay`] reduces: a replay
/// that is missing part of the true row space is accepted with probability at
/// most `2^-16`.
pub const DEFAULT_AUDIT: usize = 16;

/// Replay `trace` on `polys`: `Some(basis)` equal to the full run's when the
/// shape is consistent with the trace, `None` otherwise (run the full kernel).
/// Audited with [`DEFAULT_AUDIT`] combinations; see the module docs.
pub fn matrix_f4_f2_replay(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    trace: &F4Trace,
) -> Option<Vec<F2BoolPoly>> {
    matrix_f4_f2_replay_counted(polys, n_vars, degree, trace, DEFAULT_AUDIT).map(|(r, _)| r)
}

/// As [`matrix_f4_f2_replay`], returning the word XORs of the replayed
/// elimination (and of the `audit` random dropped-row combinations, at most
/// 64, reduced against the replayed basis, if `audit > 0`).
pub fn matrix_f4_f2_replay_counted(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    trace: &F4Trace,
    audit: usize,
) -> Option<(Vec<F2BoolPoly>, u64)> {
    matrix_f4_f2_replay_checked(polys, n_vars, degree, trace, audit).ok()
}

/// As [`matrix_f4_f2_replay_counted`], naming the refusal and the word XORs
/// the refused attempt had already spent.
pub fn matrix_f4_f2_replay_checked(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    trace: &F4Trace,
    audit: usize,
) -> Result<(Vec<F2BoolPoly>, u64), (ReplayReject, u64)> {
    use ReplayReject as R;
    if trace.n_vars != n_vars
        || trace.degree != degree
        || trace.poly_degrees.len() != polys.len()
        || polys
            .iter()
            .zip(&trace.poly_degrees)
            .any(|(p, &d)| poly_degree(p) != d)
        || audit > 64
    {
        return Err((R::Shape, 0));
    }
    if trace.steps.is_empty() {
        return Ok((Vec::new(), 0));
    }
    let mut monos: Vec<Vec<u64>> = Vec::with_capacity(trace.total_survivors());
    for step in &trace.steps {
        for &(i, t) in &step.survivors {
            let row = product_row(polys.get(i as usize).ok_or((R::Shape, 0))?, t);
            if row.is_empty() {
                return Err((R::EmptyProduct, 0));
            }
            monos.push(row);
        }
    }
    let cols = macaulay_columns(&monos).ok_or((R::Size, 0))?;
    let mut matrix = pack_rows(&monos, &cols);
    let mut ops = 0u64;
    let rank = rref_f2_counted(&mut matrix, cols.len(), &mut ops);
    if rank < trace.rank {
        return Err((R::Rank, ops));
    }
    let lead = leading_histogram(
        matrix[..rank].iter().map(|row| {
            let w = row.iter().position(|&x| x != 0).unwrap();
            cols[w * 64 + row[w].trailing_zeros() as usize]
        }),
        degree,
    );
    if lead
        .iter()
        .zip(&trace.leading_by_degree)
        .any(|(a, b)| a < b)
    {
        return Err((R::Leading, ops));
    }
    if audit > 0
        && !audit_dropped(
            polys,
            n_vars,
            degree,
            trace,
            &cols,
            &matrix[..rank],
            audit,
            &mut ops,
        )
    {
        return Err((R::Audit, ops));
    }
    charge_word_ops(ops);
    let out = matrix[..rank]
        .iter()
        .map(|row| poly_from_column_order(row_monos(row, &cols), polys[0].n_vars))
        .collect();
    Ok((out, ops))
}

/// `k` random combinations of the dropped products must lie in the span of
/// the replayed basis (RREF rows).  Each failing combination proves a dropped
/// row outside the span; a passing set bounds that miss by `2^-k`.
#[allow(clippy::too_many_arguments)]
fn audit_dropped(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    trace: &F4Trace,
    cols: &[u64],
    basis: &[Vec<u64>],
    k: usize,
    ops: &mut u64,
) -> bool {
    let kept: HashSet<(u32, u64)> = trace
        .steps
        .iter()
        .flat_map(|s| s.survivors.iter().copied())
        .collect();
    let Some(dropped) = degree_major_rows(polys, n_vars, degree, |i, t| !kept.contains(&(i, t)))
    else {
        return false;
    };
    let mut state =
        0x9e37_79b9_7f4a_7c15u64 ^ (dropped.len() as u64).wrapping_mul(0xff51_afd7_ed55_8ccd);
    let keep_bits = if k == 64 { u64::MAX } else { (1u64 << k) - 1 };
    let mut acc: HashMap<u64, u64> = HashMap::new();
    for r in &dropped {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let tag = state & keep_bits;
        for &m in &r.3 {
            *acc.entry(m).or_insert(0) ^= tag;
        }
    }
    let index: HashMap<u64, usize> = cols.iter().enumerate().map(|(c, &m)| (m, c)).collect();
    let words = cols.len().div_ceil(64);
    let mut vecs = vec![vec![0u64; words]; k];
    for (m, bits) in acc {
        if bits == 0 {
            continue;
        }
        let Some(&c) = index.get(&m) else {
            return false;
        };
        for (j, v) in vecs.iter_mut().enumerate() {
            if bits >> j & 1 == 1 {
                v[c / 64] ^= 1 << (c % 64);
            }
        }
    }
    let leads: Vec<usize> = basis
        .iter()
        .map(|row| {
            let w = row.iter().position(|&x| x != 0).unwrap();
            w * 64 + row[w].trailing_zeros() as usize
        })
        .collect();
    for mut v in vecs {
        for (row, &lc) in basis.iter().zip(&leads) {
            if v[lc / 64] >> (lc % 64) & 1 == 1 {
                for (a, b) in v[lc / 64..].iter_mut().zip(&row[lc / 64..]) {
                    *a ^= *b;
                }
                *ops += (words - lc / 64) as u64;
            }
        }
        if v.iter().any(|&x| x != 0) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use crate::cryptanalysis::koblitz_bench::ladder_sample;
    use crate::cryptanalysis::koblitz_groebner::{
        build_decomposition_system, matrix_f4_f2, matrix_f4_f2_counted, FieldStructure,
    };
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn key(rows: &[F2BoolPoly]) -> Vec<Vec<u64>> {
        let mut k: Vec<Vec<u64>> = rows
            .iter()
            .map(|p| p.terms.iter().map(|t| t.mask).collect())
            .collect();
        k.sort();
        k
    }

    fn generic_draws(nv: usize, np: usize, seed: u64, count: usize) -> Vec<Vec<F2BoolPoly>> {
        let mut st = seed | 1;
        let mut next = move || {
            st ^= st << 13;
            st ^= st >> 7;
            st ^= st << 17;
            st
        };
        let mut monos = vec![0u64];
        for i in 0..nv {
            monos.push(1 << i);
            for j in i + 1..nv {
                monos.push((1 << i) | (1 << j));
            }
        }
        (0..count)
            .map(|_| {
                (0..np)
                    .map(|_| {
                        let t = monos
                            .iter()
                            .filter(|_| next() & 1 == 1)
                            .map(|&m| F2BoolMono::from_mask(m))
                            .collect();
                        F2BoolPoly::from_monos(t, nv)
                    })
                    .collect()
            })
            .collect()
    }

    /// The trace of the first `train` draws, merged.
    fn train_trace(sys: &[Vec<F2BoolPoly>], nv: usize, deg: u32, train: usize) -> F4Trace {
        let (_, _, mut tr) = matrix_f4_f2_record_counted(&sys[0], nv, deg).unwrap();
        for s in &sys[1..train] {
            let (_, _, t) = matrix_f4_f2_record_counted(s, nv, deg).unwrap();
            assert!(tr.merge(&t));
        }
        tr
    }

    /// Replay on every draw; every accepted replay equals the full run.
    /// Returns (accepted, rejected).
    fn check_family(sys: &[Vec<F2BoolPoly>], nv: usize, deg: u32, tr: &F4Trace) -> (usize, usize) {
        let (mut acc, mut rej) = (0, 0);
        for s in sys {
            let full = matrix_f4_f2(s, nv, deg).unwrap();
            match matrix_f4_f2_replay(s, nv, deg, tr) {
                Some(r) => {
                    assert_eq!(
                        key(&r),
                        key(&full),
                        "replay accepted but differs from the full run"
                    );
                    acc += 1;
                }
                None => rej += 1,
            }
        }
        (acc, rej)
    }

    #[test]
    fn f4_trace_replay_equals_full_run_on_koblitz_family() {
        // Fixed factor-base subspace, fresh target per draw.
        for (m, n, ell, deg) in [(3usize, 5u32, 2usize, 4u32), (3, 7, 2, 3)] {
            let (sys, nv) = draws_fixed_basis(m, n, ell, 5, 23);
            let tr = train_trace(&sys, nv, deg, 3);
            let (acc, rej) = check_family(&sys[3..], nv, deg, &tr);
            eprintln!("koblitz m={m} n={n} ell={ell} d={deg}: replay accepted {acc}, fell back {rej} of 20");
            assert_eq!(acc + rej, 20);
        }
    }

    #[test]
    fn f4_trace_replay_equals_full_run_on_generic_family() {
        let sys = generic_draws(10, 10, 77, 23);
        let tr = train_trace(&sys, 10, 4, 3);
        let (acc, rej) = check_family(&sys[3..], 10, 4, &tr);
        eprintln!("generic nv=10 np=10 d=4: replay accepted {acc}, fell back {rej} of 20");
        assert!(acc >= 1);
    }

    #[test]
    fn f4_trace_rejects_a_changed_shape() {
        let sys = generic_draws(10, 10, 77, 4);
        let tr = train_trace(&sys, 10, 4, 3);
        let s = &sys[3];
        // Variable count, degree, input count and input degree.
        assert!(matrix_f4_f2_replay(s, 11, 4, &tr).is_none());
        assert!(matrix_f4_f2_replay(s, 10, 3, &tr).is_none());
        assert!(matrix_f4_f2_replay(s, 10, 5, &tr).is_none());
        assert!(matrix_f4_f2_replay(&s[..9], 10, 4, &tr).is_none());
        let mut cubic = s.clone();
        cubic[0] = F2BoolPoly::from_monos(vec![F2BoolMono::from_mask(0b111)], 10);
        assert!(matrix_f4_f2_replay(&cubic, 10, 4, &tr).is_none());
        let mut other = tr.clone();
        assert!(!other.merge(&F4Trace {
            degree: 3,
            ..tr.clone()
        }));
    }

    /// A draw whose structure differs -- a variable forced to 0 or to 1 --
    /// either replays correctly or is refused, on both families.
    #[test]
    fn f4_trace_specialised_draw_never_returns_a_wrong_answer() {
        fn specialise(sys: &[F2BoolPoly], var: usize, value: bool, nv: usize) -> Vec<F2BoolPoly> {
            sys.iter()
                .map(|p| {
                    let terms: Vec<F2BoolMono> = p
                        .terms
                        .iter()
                        .filter_map(|t| {
                            if t.mask >> var & 1 == 0 {
                                Some(*t)
                            } else if value {
                                Some(F2BoolMono::from_mask(t.mask & !(1u64 << var)))
                            } else {
                                None
                            }
                        })
                        .collect();
                    F2BoolPoly::from_monos(terms, nv)
                })
                .filter(|p| !p.is_zero())
                .collect()
        }
        let mut outcomes = (0usize, 0usize);
        let (ksys, knv) = draws_fixed_basis(3, 5, 2, 5, 12);
        let gsys = generic_draws(10, 10, 77, 12);
        for (sys, nv, deg) in [(ksys, knv, 4u32), (gsys, 10, 4)] {
            let tr = train_trace(&sys, nv, deg, 3);
            for s in &sys[3..] {
                for value in [false, true] {
                    for var in [0, nv - 1] {
                        let sp = specialise(s, var, value, nv);
                        if sp.is_empty() {
                            continue;
                        }
                        let full = matrix_f4_f2(&sp, nv, deg).unwrap();
                        match matrix_f4_f2_replay(&sp, nv, deg, &tr) {
                            Some(r) => {
                                assert_eq!(key(&r), key(&full), "x{var}:={value}: wrong answer");
                                outcomes.0 += 1;
                            }
                            None => outcomes.1 += 1,
                        }
                    }
                }
            }
        }
        eprintln!(
            "specialised draws: replayed {}, refused {}",
            outcomes.0, outcomes.1
        );
    }

    /// Word operations, full against replayed, over 20 draws.
    #[test]
    fn f4_trace_counted_savings_table() {
        struct Row {
            name: String,
            full_all: u64,
            full_acc: u64,
            replay_acc: u64,
            replay_unaudited: u64,
            wasted: u64,
            acc: usize,
            record: u64,
        }
        let mut rows = Vec::new();
        let sets: Vec<(String, Vec<Vec<F2BoolPoly>>, usize, u32)> = {
            let g = generic_draws(10, 10, 77, 23);
            let (k5, knv5) = draws_fixed_basis(3, 5, 2, 5, 23);
            let (k7, knv7) = draws_fixed_basis(3, 7, 2, 5, 23);
            vec![
                ("generic 10 var, 10 quadrics, d=4".into(), g, 10, 4),
                (
                    format!("Koblitz m=3 n=5 l=2 ({knv5} var), d=4"),
                    k5,
                    knv5,
                    4,
                ),
                (
                    format!("Koblitz m=3 n=7 l=2 ({knv7} var), d=4"),
                    k7,
                    knv7,
                    4,
                ),
            ]
        };
        for (name, sys, nv, deg) in sets {
            let tr = train_trace(&sys, nv, deg, 3);
            let mut row = Row {
                name,
                full_all: 0,
                full_acc: 0,
                replay_acc: 0,
                replay_unaudited: 0,
                wasted: 0,
                acc: 0,
                record: tr.record_word_ops,
            };
            for s in &sys[3..] {
                let (full, full_ops) = matrix_f4_f2_counted(s, nv, deg).unwrap();
                row.full_all += full_ops;
                match matrix_f4_f2_replay_checked(s, nv, deg, &tr, DEFAULT_AUDIT) {
                    Ok((r, ops)) => {
                        assert_eq!(key(&r), key(&full));
                        row.acc += 1;
                        row.full_acc += full_ops;
                        row.replay_acc += ops;
                        if let Ok((_, raw)) = matrix_f4_f2_replay_checked(s, nv, deg, &tr, 0) {
                            row.replay_unaudited += raw;
                        }
                    }
                    Err((_, w)) => row.wasted += w,
                }
            }
            rows.push(row);
        }
        eprintln!("| family | accepted/20 | full (accepted) | replay audited | replay unaudited | wasted on refusals | full (all 20) | record (3 draws) |");
        for r in &rows {
            eprintln!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                r.name,
                r.acc,
                r.full_acc,
                r.replay_acc,
                r.replay_unaudited,
                r.wasted,
                r.full_all,
                r.record
            );
        }
        let g = &rows[0];
        assert!(
            g.acc >= 1 && g.replay_acc < g.full_acc,
            "replay must be cheaper on the generic family"
        );
    }

    fn draws_fixed_basis(
        m: usize,
        n: u32,
        ell: usize,
        seed: u64,
        count: usize,
    ) -> (Vec<Vec<F2BoolPoly>>, usize) {
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let b = F2mElement::one(n);
        let mut rng = StdRng::seed_from_u64(seed);
        let (basis, _) = ladder_sample(n, ell, &mut rng);
        let mut n_vars = 0;
        let systems = (0..count)
            .map(|_| {
                let (_, x_r) = ladder_sample(n, ell, &mut rng);
                let sys = build_decomposition_system(&basis, &x_r, &b, m, &st).unwrap();
                n_vars = sys.n_vars;
                sys.equations
            })
            .collect();
        (systems, n_vars)
    }
}
