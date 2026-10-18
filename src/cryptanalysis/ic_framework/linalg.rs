//! # Relation matrices, as plug-ins.
//!
//! The relation loop takes any [`RelationSolver`]; this module ships a
//! second one so the linear-algebra stage is a real lever rather than a
//! fixed cost.
//!
//! | name | method | storage | pivot | work unit |
//! |:--|:--|:--|:--|:--|
//! | `incremental-gauss` | dense reduced row echelon, maintained as rows arrive | `rank × |F|` entries | leftmost non-zero | `row_ops` |
//! | `incremental-gauss-full-rank` | same dense matrix, continue after target pin until every column is independent | `rank × |F|` entries | leftmost non-zero | `row_ops` |
//! | `incremental-gauss-full-rank-checked` | same full-rank matrix, then point-check one representative per factor-base column | `rank × |F|` entries | leftmost non-zero | `row_ops` |
//! | `structured-gauss` | sparse reduced row echelon, maintained as rows arrive | the non-zeros | lightest column (Markowitz) | `row_ops` |
//! | `sparse-cado` | batch: CADO-style filter (clique purge, Markowitz/spanning-tree merge) then block Wiedemann, re-run as relations arrive | the non-zeros | — | `row_ops` |
//! | `filter+wiedemann` | the same configuration as `sparse-cado`, under the explicit core-solver name | the non-zeros | — | `row_ops` |
//! | `filter+lanczos` | batch: the same CADO-style filter then block Lanczos | the non-zeros | — | `row_ops` |
//! | `wiedemann` | batch: no filter (every row kept, folded to a square system) then block Wiedemann | the non-zeros | — | `row_ops` |
//! | `lanczos` | batch: no filter then block Lanczos | the non-zeros | — | `row_ops` |
//!
//! The five batch rows share one deferred-solve implementation
//! ([`SparseCado`]): rows are buffered as they arrive, the solve runs at
//! full column rank, and `row_ops` is
//! [`SparseSolveReport::row_ops`](crate::cryptanalysis::koblitz_sparse_la::SparseSolveReport::row_ops)
//! summed over every solve tried (the filter's merges, the folding, one
//! per non-zero per lane of each sparse product of every Krylov run that
//! returned, the checks and the reconstruction; the `block × block`
//! arithmetic of the approximant basis or the Lanczos recurrences is not
//! counted, as the dense method does not count its column scans), plus
//! one per non-zero for converting each arriving row.  Each charge
//! is made once, where the operation happens.  The word width follows the
//! modulus, so the pipeline's `u64` moduli run the 64-bit kernels; the
//! solvers of `koblitz_sparse_la` take wider moduli directly.
//!
//! Both count a multiply-subtract on a **non-zero** entry as one
//! `row_op`, so the two `work` figures are directly comparable and
//! differ only through the pivot choice: pivoting on the lightest
//! column limits fill-in, which on the `m`-non-zero rows an
//! index-calculus relation phase produces is worth a few per cent, not
//! a factor (538 against 582 `row_ops` on 63 two-summand rows over 121
//! columns, same rank, same answer).  What the dense matrix pays and
//! does not count is the scan of every column of every pivot row, and
//! `rank × |F|` words of storage against the sparse one's non-zeros;
//! that is the cost a structured elimination exists to avoid, and it
//! shows in `wall_ns` and `peak` before it shows in `row_ops`.
//!
//! ## The contract both obey
//!
//! `add_row` reduces the new row against the pivots held so far, and
//! either adopts it as a new pivot (`Independent`), finds it reduces to
//! `0 = 0` (`Dependent`) or to `0 = c ≠ 0` (`Inconsistent`).  `pinned`
//! answers whether the matrix determines one column on its own.  The
//! relation loop calls them in that order after every relation, so an
//! implementation must answer incrementally; a batch method would have
//! to re-solve on every call and is not what this trait is for.

use std::cell::RefCell;

use num_bigint::BigUint;

use super::stages::RelationSolverFactory;
use crate::cryptanalysis::ic_boundary::{
    CompletionRule, IncrementalGauss, RelationSolver, RowStatus,
};
use crate::cryptanalysis::koblitz_sparse_la::{
    FilterOptions, SparseCoreSolver, SparseRows, SparseSolveOptions, SparseSolveOutcome,
};

#[cfg(test)]
fn addmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}
fn submod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + m as u128 - b as u128) % m as u128) as u64
}
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn invmod(a: u64, m: u64) -> Option<u64> {
    // Extended Euclid over i128; `m` is prime, so any non-zero `a` inverts.
    let (mut r0, mut r1) = (m as i128, (a % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if r0 != 1 {
        return None;
    }
    Some(((t0 % m as i128 + m as i128) % m as i128) as u64)
}

/// A sparse row: `(column, coefficient)` sorted by column, no zeros.
type Sparse = Vec<(usize, u64)>;

/// `a − f·b`, both sparse and sorted, counting one `row_op` per
/// non-zero touched.
fn sparse_sub_scaled(a: &Sparse, f: u64, b: &Sparse, m: u64, ops: &mut u64) -> Sparse {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        match (a.get(i), b.get(j)) {
            (Some(&(ca, va)), Some(&(cb, vb))) if ca == cb => {
                let v = submod(va, mulmod(f, vb, m), m);
                *ops += 1;
                if v != 0 {
                    out.push((ca, v));
                }
                i += 1;
                j += 1;
            }
            (Some(&(ca, va)), Some(&(cb, _))) if ca < cb => {
                out.push((ca, va));
                i += 1;
            }
            (Some(&(ca, va)), None) => {
                out.push((ca, va));
                i += 1;
            }
            (_, Some(&(cb, vb))) => {
                let v = submod(0, mulmod(f, vb, m), m);
                *ops += 1;
                if v != 0 {
                    out.push((cb, v));
                }
                j += 1;
            }
            (None, None) => break,
        }
    }
    out
}

/// Sparse incremental elimination, pivoting on the lightest column.
///
/// Rows are kept sparse and the pivot of a new row is its non-zero
/// column of least global weight — the Markowitz-style choice that
/// keeps fill-in down.  Every multiply-subtract on a stored non-zero is
/// one `row_op`; nothing is charged for the zeros the dense method
/// walks over.  On relation rows with `m` non-zeros in `|F|` columns
/// that is the difference between `O(|F|)` and `O(m)` per reduction
/// step, which is where a real index-calculus linear algebra spends its
/// time and its memory.
pub struct StructuredGauss {
    modulus: u64,
    cols: usize,
    rows: Vec<Sparse>,
    rhs: Vec<u64>,
    pivot_of_row: Vec<usize>,
    row_of_pivot: Vec<Option<usize>>,
    /// Non-zeros per column across stored rows, for the pivot choice.
    weight: Vec<u32>,
    pub row_ops: u64,
    pub dependent: u64,
    pub inconsistent: u64,
    /// Total non-zeros stored: the method's footprint, which the dense
    /// method cannot report because it has none to speak of.
    pub nonzeros: u64,
    pub peak_nonzeros: u64,
}

impl StructuredGauss {
    pub fn new(cols: usize, modulus: u64) -> Self {
        Self {
            modulus,
            cols,
            rows: Vec::new(),
            rhs: Vec::new(),
            pivot_of_row: Vec::new(),
            row_of_pivot: vec![None; cols],
            weight: vec![0; cols],
            row_ops: 0,
            dependent: 0,
            inconsistent: 0,
            nonzeros: 0,
            peak_nonzeros: 0,
        }
    }

    fn note_weight(&mut self, row: &Sparse, delta: i32) {
        for &(c, _) in row {
            let w = &mut self.weight[c];
            *w = (*w as i32 + delta).max(0) as u32;
        }
    }
}

impl RelationSolver for StructuredGauss {
    fn add_row(&mut self, dense: Vec<u64>, mut rhs: u64) -> RowStatus {
        let m = self.modulus;
        debug_assert_eq!(dense.len(), self.cols);
        let mut row: Sparse = dense
            .iter()
            .enumerate()
            .filter(|(_, &v)| v != 0)
            .map(|(c, &v)| (c, v % m))
            .collect();
        // Reduce against every pivot the new row touches.  A sparse row
        // touches few, and the reduction adds only the pivot row's
        // non-zeros, so this is the step the structure pays for.
        loop {
            let hit = row
                .iter()
                .find_map(|&(c, f)| self.row_of_pivot[c].map(|i| (i, f)));
            let Some((i, f)) = hit else { break };
            let pivot_row = self.rows[i].clone();
            row = sparse_sub_scaled(&row, f, &pivot_row, m, &mut self.row_ops);
            rhs = submod(rhs, mulmod(f, self.rhs[i], m), m);
            self.row_ops += 1;
        }
        if row.is_empty() {
            if rhs == 0 {
                self.dependent += 1;
                return RowStatus::Dependent;
            }
            self.inconsistent += 1;
            return RowStatus::Inconsistent;
        }
        // Pivot on the lightest column the row touches.
        let (pc, pv) = row
            .iter()
            .copied()
            .min_by_key(|&(c, _)| (self.weight[c], c))
            .expect("non-empty row");
        let inv = invmod(pv, m).expect("prime modulus");
        for (_, v) in row.iter_mut() {
            *v = mulmod(*v, inv, m);
            self.row_ops += 1;
        }
        rhs = mulmod(rhs, inv, m);
        self.row_ops += 1;
        // Eliminate the new pivot column from every stored row that
        // holds it; only those rows are touched.
        for i in 0..self.rows.len() {
            let f = match self.rows[i].binary_search_by_key(&pc, |&(c, _)| c) {
                Ok(k) => self.rows[i][k].1,
                Err(_) => continue,
            };
            let before = self.rows[i].clone();
            let after = sparse_sub_scaled(&before, f, &row, m, &mut self.row_ops);
            self.rhs[i] = submod(self.rhs[i], mulmod(f, rhs, m), m);
            self.row_ops += 1;
            self.nonzeros = self.nonzeros - before.len() as u64 + after.len() as u64;
            self.note_weight(&before, -1);
            self.note_weight(&after, 1);
            self.rows[i] = after;
        }
        self.row_of_pivot[pc] = Some(self.rows.len());
        self.pivot_of_row.push(pc);
        self.nonzeros += row.len() as u64;
        self.note_weight(&row, 1);
        self.peak_nonzeros = self.peak_nonzeros.max(self.nonzeros);
        self.rows.push(row);
        self.rhs.push(rhs);
        RowStatus::Independent
    }

    fn pinned(&self, col: usize) -> Option<u64> {
        let i = self.row_of_pivot[col]?;
        let row = &self.rows[i];
        (row.len() == 1 && row[0].0 == col).then_some(self.rhs[i])
    }

    fn rank(&self) -> usize {
        self.rows.len()
    }

    fn dependent(&self) -> u64 {
        self.dependent
    }

    fn work(&self) -> (u64, &'static str) {
        (self.row_ops, "row_ops")
    }
}

/// The large-matrix pipeline of `koblitz_sparse_la` behind the
/// incremental contract: CADO-NFS-style filtering
/// ([`FilterOptions::cado`]: duplicates, singletons, clique purge,
/// Markowitz merge along spanning trees) and block Wiedemann on the core.
///
/// It is a batch method, so it buffers the rows and re-solves.  A solve
/// is tried once every column occurs in some row and there are at least
/// as many rows as columns, and then again every `max(1, cols / 32)`
/// further rows, so it reads at most about 3 % more relations than the
/// full-rank point needs.  It answers **only at full column rank**: the
/// framework runs it under the full-rank completion rule, and its trial
/// counts compare with `incremental-gauss-full-rank`, not with the
/// target-pinned default, which can stop with fewer relations than
/// columns.
///
/// [`solve_sparse_system`](crate::cryptanalysis::koblitz_sparse_la::solve_sparse_system)
/// verifies a solution against every row but does not certify rank, so
/// a solution is accepted only when a second solve under an independent
/// seed returns the same vector: for a rank-deficient system the two
/// differ by a random kernel element except with probability about
/// `1/r`.  Both solves are charged.  Until then [`RelationSolver::rank`]
/// reports `min(rows, cols − 1)`, a placeholder rather than a rank (the
/// method tracks none); at acceptance it is `cols` and the rows beyond
/// it are counted as dependent.
///
/// `work` is [`SparseSolveReport::row_ops`](crate::cryptanalysis::koblitz_sparse_la::SparseSolveReport::row_ops)
/// summed over every solve tried, in the unit the incremental methods
/// use, plus one per non-zero for converting each arriving row.
pub struct SparseCado {
    modulus: u64,
    cols: usize,
    filter: FilterOptions,
    core: SparseCoreSolver,
    state: RefCell<SparseCadoState>,
}

struct SparseCadoState {
    rows: SparseRows,
    covered: Vec<bool>,
    uncovered: usize,
    next_attempt: usize,
    solution: Option<Vec<u64>>,
    /// The last solve found a contradiction; the next row reports it.
    inconsistent: bool,
    row_ops: u64,
    dependent: u64,
    attempts: u64,
}

impl SparseCado {
    pub fn new(cols: usize, modulus: u64) -> Self {
        Self::with_core(cols, modulus, SparseCoreSolver::BlockWiedemann, true)
    }

    /// A deferred sparse solver with the given Krylov core, behind the
    /// CADO-style filter (`filtered`) or on the unfiltered rows.
    pub fn with_core(cols: usize, modulus: u64, core: SparseCoreSolver, filtered: bool) -> Self {
        let filter = if filtered {
            FilterOptions::cado()
        } else {
            // Keep every row: no duplicate or singleton removal, no
            // excess removal, no merging; the core solve folds the
            // surplus rows into a square system.
            FilterOptions {
                remove_duplicates: false,
                remove_singletons: false,
                target_excess: usize::MAX,
                merge_max_weight: 0,
                max_row_weight: usize::MAX,
                ..FilterOptions::default()
            }
        };
        Self {
            modulus,
            cols,
            filter,
            core,
            state: RefCell::new(SparseCadoState {
                rows: SparseRows::for_modulus(&BigUint::from(modulus))
                    .expect("a prime modulus of at most 256 bits"),
                covered: vec![false; cols],
                uncovered: cols,
                next_attempt: cols,
                solution: None,
                inconsistent: false,
                row_ops: 0,
                dependent: 0,
                attempts: 0,
            }),
        }
    }

    /// Solves tried so far.
    pub fn attempts(&self) -> u64 {
        self.state.borrow().attempts
    }

    fn try_solve(&self, st: &mut SparseCadoState) {
        let rows = st.rows.len();
        if st.solution.is_some() || st.uncovered > 0 || rows < st.next_attempt {
            return;
        }
        st.next_attempt = rows + (self.cols / 32).max(1);
        let solve = |seed: u64, st: &mut SparseCadoState| {
            let opts = SparseSolveOptions {
                filter: self.filter,
                solver: self.core,
                seed,
                ..SparseSolveOptions::default()
            };
            st.attempts += 1;
            let (outcome, report) = st.rows.solve(self.cols, &opts);
            st.row_ops += report.row_ops;
            outcome
        };
        let first = match solve(0x5350_4152_5345_4341, st) {
            SparseSolveOutcome::Solved(x) => x,
            SparseSolveOutcome::Undetermined => return,
            SparseSolveOutcome::Inconsistent => {
                st.inconsistent = true;
                return;
            }
        };
        let second = match solve(0x4345_5254_4946_5932 ^ rows as u64, st) {
            SparseSolveOutcome::Solved(x) => x,
            _ => return,
        };
        if first != second {
            return;
        }
        let modulus = BigUint::from(self.modulus);
        st.solution = Some(
            first
                .iter()
                .map(|v| (v % &modulus).to_u64_digits().first().copied().unwrap_or(0))
                .collect(),
        );
        st.dependent += (rows - self.cols) as u64;
    }
}

impl RelationSolver for SparseCado {
    fn add_row(&mut self, dense: Vec<u64>, rhs: u64) -> RowStatus {
        let m = self.modulus;
        debug_assert_eq!(dense.len(), self.cols);
        let st = self.state.get_mut();
        if std::mem::take(&mut st.inconsistent) {
            return RowStatus::Inconsistent;
        }
        let entries: Vec<(u32, BigUint)> = dense
            .iter()
            .enumerate()
            .filter(|(_, &v)| !v.is_multiple_of(m))
            .map(|(c, &v)| (c as u32, BigUint::from(v % m)))
            .collect();
        st.row_ops += entries.len() as u64;
        if entries.is_empty() {
            if rhs.is_multiple_of(m) {
                st.dependent += 1;
                return RowStatus::Dependent;
            }
            return RowStatus::Inconsistent;
        }
        if let Some(x) = &st.solution {
            // Past full rank every row is a combination of the others:
            // check it against the solution instead of storing it.
            let lhs = entries.iter().fold(0u64, |acc, (c, v)| {
                let v = v.to_u64_digits()[0];
                ((acc as u128 + v as u128 * x[*c as usize] as u128) % m as u128) as u64
            });
            st.row_ops += entries.len() as u64;
            if lhs == rhs % m {
                st.dependent += 1;
                return RowStatus::Dependent;
            }
            return RowStatus::Inconsistent;
        }
        for (c, _) in &entries {
            let c = *c as usize;
            if !st.covered[c] {
                st.covered[c] = true;
                st.uncovered -= 1;
            }
        }
        st.rows.push_entries(&entries, &BigUint::from(rhs % m));
        RowStatus::Independent
    }

    fn pinned(&self, col: usize) -> Option<u64> {
        let mut st = self.state.borrow_mut();
        self.try_solve(&mut st);
        st.solution.as_ref().map(|x| x[col])
    }

    fn rank(&self) -> usize {
        let st = self.state.borrow();
        if st.solution.is_some() {
            self.cols
        } else {
            st.rows.len().min(self.cols.saturating_sub(1))
        }
    }

    fn dependent(&self) -> u64 {
        self.state.borrow().dependent
    }

    fn work(&self) -> (u64, &'static str) {
        (self.state.borrow().row_ops, "row_ops")
    }
}

/// The matrices the framework knows, by name.
pub const MATRIX_NAMES: &[(&str, &str)] = &[
    (
        "incremental-gauss",
        "dense reduced row echelon maintained as rows arrive, pivot on the leftmost non-zero; one row_op per non-zero multiply-subtract",
    ),
    (
        "incremental-gauss-full-rank",
        "same dense incremental matrix, but collect until every factor-base column and the target column are independent",
    ),
    (
        "incremental-gauss-full-rank-checked",
        "same full-rank matrix, then charge a full-point logarithm check for one representative of every factor-base column",
    ),
    (
        "structured-gauss",
        "sparse reduced row echelon maintained as rows arrive, pivot on the lightest column; one row_op per non-zero multiply-subtract, so the two are comparable",
    ),
    (
        "sparse-cado",
        "batch CADO-style filter (clique purge, Markowitz/spanning-tree merge) then block Wiedemann, re-solved as relations arrive; full-rank completion, answer certified by two independently seeded solves; one row_op per entry multiply-subtract",
    ),
    (
        "filter+wiedemann",
        "the sparse-cado configuration under its explicit name: CADO-style filter then block Wiedemann, full-rank completion, two independently seeded solves; one row_op per entry multiply-subtract",
    ),
    (
        "filter+lanczos",
        "CADO-style filter then block Lanczos on the core, re-solved as relations arrive; full-rank completion, answer certified by two independently seeded solves; row_ops as sparse-cado",
    ),
    (
        "wiedemann",
        "no filter (every row kept, surplus rows folded into a square system) then block Wiedemann, re-solved as relations arrive; full-rank completion, answer certified by two independently seeded solves; row_ops as sparse-cado",
    ),
    (
        "lanczos",
        "no filter then block Lanczos, re-solved as relations arrive; full-rank completion, answer certified by two independently seeded solves; row_ops as sparse-cado",
    ),
];

/// The deferred sparse solvers answer only at full column rank, so the
/// pipeline runs them under the full-rank completion rule.
pub fn is_deferred_sparse(name: &str) -> bool {
    matches!(
        name,
        "sparse-cado" | "filter+wiedemann" | "filter+lanczos" | "wiedemann" | "lanczos"
    )
}

/// Build one by name, for a matrix with `cols` columns over `Z/rZ`.
pub fn matrix_by_name(
    name: &str,
    cols: usize,
    modulus: u64,
) -> Result<Box<dyn RelationSolver>, String> {
    match name {
        "incremental-gauss"
        | "incremental-gauss-full-rank"
        | "incremental-gauss-full-rank-checked" => {
            Ok(Box::new(IncrementalGauss::new(cols, modulus)))
        }
        "structured-gauss" => Ok(Box::new(StructuredGauss::new(cols, modulus))),
        "sparse-cado" | "filter+wiedemann" => Ok(Box::new(SparseCado::new(cols, modulus))),
        "filter+lanczos" => Ok(Box::new(SparseCado::with_core(
            cols,
            modulus,
            SparseCoreSolver::BlockLanczos,
            true,
        ))),
        "wiedemann" => Ok(Box::new(SparseCado::with_core(
            cols,
            modulus,
            SparseCoreSolver::BlockWiedemann,
            false,
        ))),
        "lanczos" => Ok(Box::new(SparseCado::with_core(
            cols,
            modulus,
            SparseCoreSolver::BlockLanczos,
            false,
        ))),
        other => Err(format!(
            "unknown relation matrix `{other}`; known: {}",
            MATRIX_NAMES
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// Adapter for the shipped matrices. Downstream factories implement
/// [`RelationSolverFactory`] directly and need no entry in this list.
pub struct BuiltinMatrix {
    name: &'static str,
}

impl BuiltinMatrix {
    pub fn new(name: &str) -> Result<Self, String> {
        let name = if name.is_empty() {
            "incremental-gauss"
        } else {
            name
        };
        MATRIX_NAMES
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(name, _)| Self { name })
            .ok_or_else(|| format!("unknown relation matrix `{name}`"))
    }
}

impl RelationSolverFactory for BuiltinMatrix {
    fn name(&self) -> &str {
        self.name
    }

    fn create(&self, columns: usize, modulus: u64) -> Result<Box<dyn RelationSolver + '_>, String> {
        matrix_by_name(self.name, columns, modulus)
    }

    fn completion_rule(&self) -> CompletionRule {
        match self.name {
            "incremental-gauss-full-rank"
            | "incremental-gauss-full-rank-checked"
            | "sparse-cado" => CompletionRule::FullRank,
            _ => CompletionRule::TargetPinned,
        }
    }

    fn verify_base_columns(&self) -> bool {
        self.name == "incremental-gauss-full-rank-checked"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// **Both matrices must pin the same unknown to the same value on
    /// the same rows, at the same moment.**  A second implementation
    /// that agreed on the answer but pinned it a row later would change
    /// every trial count downstream while looking correct.
    #[test]
    fn structured_and_dense_agree_row_by_row() {
        let modulus = 1_000_003u64;
        let cols = 12usize;
        let target = cols - 1;
        let mut rng = StdRng::seed_from_u64(20260922);
        // A planted solution, so every row is consistent and the value
        // pinned can be checked against something.
        let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..modulus)).collect();
        let mut dense = IncrementalGauss::new(cols, modulus);
        let mut sparse = StructuredGauss::new(cols, modulus);
        let mut pinned_at = None;
        for row_no in 0..40 {
            // Sparse relation rows: two or three non-zeros plus the target.
            let mut row = vec![0u64; cols];
            for _ in 0..rng.gen_range(2..=3) {
                row[rng.gen_range(0..cols)] = rng.gen_range(1..modulus);
            }
            row[target] = rng.gen_range(1..modulus);
            let rhs = row.iter().zip(&x).fold(0u64, |acc, (&a, &b)| {
                addmod(acc, mulmod(a, b, modulus), modulus)
            });
            let a = dense.add_row(row.clone(), rhs);
            let b = sparse.add_row(row, rhs);
            assert_eq!(
                std::mem::discriminant(&a),
                std::mem::discriminant(&b),
                "row {row_no}: the two matrices classified it differently"
            );
            assert_eq!(
                dense.rank(),
                RelationSolver::rank(&sparse),
                "row {row_no}: rank"
            );
            let pd = dense.pinned(target);
            let ps = sparse.pinned(target);
            assert_eq!(pd, ps, "row {row_no}: pinned value");
            if let Some(v) = ps {
                assert_eq!(v, x[target], "pinned the wrong value");
                pinned_at.get_or_insert(row_no);
            }
        }
        assert!(
            pinned_at.is_some(),
            "forty rows on twelve columns must pin the target"
        );
        // And the structure must have bought something: fewer
        // multiply-subtracts than the dense walk over every column.
        assert!(
            sparse.row_ops < dense.row_ops,
            "structured {} vs dense {} row_ops",
            sparse.row_ops,
            dense.row_ops
        );
    }

    /// An inconsistent row must be reported, not absorbed.
    #[test]
    fn an_inconsistent_row_is_reported() {
        let modulus = 101u64;
        let mut m = StructuredGauss::new(3, modulus);
        assert!(matches!(
            m.add_row(vec![1, 0, 0], 5),
            RowStatus::Independent
        ));
        assert!(matches!(m.add_row(vec![1, 0, 0], 5), RowStatus::Dependent));
        assert!(matches!(
            m.add_row(vec![2, 0, 0], 11),
            RowStatus::Inconsistent
        ));
        assert_eq!(m.pinned(0), Some(5));
    }

    /// The batch solver answers at full rank, with the planted values,
    /// and charges its work in the incremental methods' unit.
    #[test]
    fn sparse_cado_recovers_a_planted_system_at_full_rank() {
        let modulus = 4_611_686_018_427_387_847u64; // a 62-bit prime
        let cols = 300usize;
        let mut rng = StdRng::seed_from_u64(20261015);
        let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..modulus)).collect();
        let mut m = SparseCado::new(cols, modulus);
        let mut rows = 0;
        while m.pinned(cols - 1).is_none() {
            assert!(rows < 3 * cols, "no solution after {rows} rows");
            let mut row = vec![0u64; cols];
            for _ in 0..3 {
                row[rng.gen_range(0..cols)] = rng.gen_range(1..modulus);
            }
            let rhs = row.iter().zip(&x).fold(0u64, |acc, (&a, &b)| {
                addmod(acc, mulmod(a, b, modulus), modulus)
            });
            assert!(!matches!(m.add_row(row, rhs), RowStatus::Inconsistent));
            rows += 1;
        }
        assert!(rows >= cols);
        assert_eq!(RelationSolver::rank(&m), cols);
        for (c, &v) in x.iter().enumerate() {
            assert_eq!(m.pinned(c), Some(v), "column {c}");
        }
        assert!(m.attempts() >= 2, "accepted without the second solve");
        let (work, unit) = m.work();
        assert_eq!(unit, "row_ops");
        assert!(work > 0);
        // A wrong row after the answer is reported, a right one is not.
        let mut row = vec![0u64; cols];
        row[0] = 1;
        assert!(matches!(m.add_row(row.clone(), x[0]), RowStatus::Dependent));
        assert!(matches!(
            m.add_row(row, addmod(x[0], 1, modulus)),
            RowStatus::Inconsistent
        ));
    }

    const DEFERRED: &[&str] = &[
        "sparse-cado",
        "filter+wiedemann",
        "filter+lanczos",
        "wiedemann",
        "lanczos",
    ];

    /// Random sparse rows (three non-zeros) of a planted system.
    fn planted_rows(
        rng: &mut StdRng,
        cols: usize,
        count: usize,
        modulus: u64,
        x: &[u64],
    ) -> Vec<(Vec<u64>, u64)> {
        (0..count)
            .map(|_| {
                let mut row = vec![0u64; cols];
                for _ in 0..3 {
                    row[rng.gen_range(0..cols)] = rng.gen_range(1..modulus);
                }
                let rhs = row.iter().zip(x).fold(0u64, |acc, (&a, &b)| {
                    addmod(acc, mulmod(a, b, modulus), modulus)
                });
                (row, rhs)
            })
            .collect()
    }

    /// Every deferred solver, on random sparse systems over several
    /// word-sized primes, returns a solution that satisfies every row
    /// (checked by substitution) and that `structured-gauss`, fed the same
    /// rows, pins to the same values; the work is reported in `row_ops`
    /// and is the same on a rerun (nothing charged twice or at random).
    #[test]
    fn deferred_sparse_solvers_agree_with_structured_gauss() {
        for (k, &modulus) in [1_000_003u64, 4_294_967_291, (1u64 << 61) - 1]
            .iter()
            .enumerate()
        {
            let cols = 90usize;
            let mut rng = StdRng::seed_from_u64(20261018 + k as u64);
            let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..modulus)).collect();
            let rows = planted_rows(&mut rng, cols, 3 * cols, modulus, &x);
            let mut reference = StructuredGauss::new(cols, modulus);
            for (row, rhs) in &rows {
                reference.add_row(row.clone(), *rhs);
            }
            assert_eq!(RelationSolver::rank(&reference), cols, "p={modulus}");
            for name in DEFERRED {
                let run = || {
                    let mut m = matrix_by_name(name, cols, modulus).unwrap();
                    let mut fed = 0usize;
                    let mut nnz = 0u64;
                    while m.pinned(cols - 1).is_none() {
                        assert!(fed < rows.len(), "{name} p={modulus}: unsolved");
                        let (row, rhs) = &rows[fed];
                        nnz += row.iter().filter(|&&v| v != 0).count() as u64;
                        assert!(!matches!(
                            m.add_row(row.clone(), *rhs),
                            RowStatus::Inconsistent
                        ));
                        fed += 1;
                    }
                    let sol: Vec<u64> = (0..cols).map(|c| m.pinned(c).unwrap()).collect();
                    (sol, m.work(), nnz, fed)
                };
                let (sol, (work, unit), nnz, fed) = run();
                assert_eq!(unit, "row_ops", "{name}");
                assert!(
                    work > nnz,
                    "{name}: work {work} must exceed the conversion {nnz}"
                );
                for (row, rhs) in &rows[..fed] {
                    let lhs = row.iter().zip(&sol).fold(0u64, |acc, (&a, &b)| {
                        addmod(acc, mulmod(a, b, modulus), modulus)
                    });
                    assert_eq!(lhs, *rhs, "{name} p={modulus}: substitution");
                }
                for c in 0..cols {
                    assert_eq!(
                        reference.pinned(c),
                        Some(sol[c]),
                        "{name} p={modulus} col {c}"
                    );
                }
                assert_eq!(sol, x, "{name} p={modulus}: planted solution");
                let (sol2, (work2, _), _, fed2) = run();
                assert_eq!(
                    (sol2, work2, fed2),
                    (sol, work, fed),
                    "{name}: deterministic"
                );
            }
        }
    }

    /// Above 64 bits the pipeline's `u64` moduli cannot go, but the
    /// solvers the deferred matrices wrap pick their word width from the
    /// modulus: the same four configurations solve a planted system over
    /// 89-, 127- and 255-bit primes, and the solution satisfies every row.
    #[test]
    fn deferred_sparse_configurations_solve_wide_moduli() {
        for bits in [89u32, 127, 255] {
            let modulus = (BigUint::from(1u8) << bits) - 1u8;
            // 2^89-1, 2^127-1 are Mersenne primes; 2^255-1 is composite,
            // so use 2^255-19 for the third.
            let modulus = if bits == 255 {
                (BigUint::from(1u8) << 255usize) - 19u8
            } else {
                modulus
            };
            let cols = 60usize;
            let mut rng = StdRng::seed_from_u64(bits as u64);
            let rand_below = |rng: &mut StdRng| {
                let mut bytes = vec![0u8; (bits as usize) / 8 + 8];
                rng.fill(&mut bytes[..]);
                BigUint::from_bytes_le(&bytes) % &modulus
            };
            let x: Vec<BigUint> = (0..cols).map(|_| rand_below(&mut rng)).collect();
            let mut raw: Vec<(Vec<(u32, BigUint)>, BigUint)> = Vec::new();
            for _ in 0..3 * cols {
                let mut e: Vec<(u32, BigUint)> = Vec::new();
                for _ in 0..3 {
                    let c = rng.gen_range(0..cols) as u32;
                    if e.iter().all(|(d, _)| *d != c) {
                        let mut v = rand_below(&mut rng);
                        if v == BigUint::from(0u8) {
                            v = BigUint::from(1u8);
                        }
                        e.push((c, v));
                    }
                }
                let rhs = e.iter().fold(BigUint::from(0u8), |a, (c, v)| {
                    (a + v * &x[*c as usize]) % &modulus
                });
                raw.push((e, rhs));
            }
            for (core, filtered) in [
                (SparseCoreSolver::BlockWiedemann, true),
                (SparseCoreSolver::BlockLanczos, true),
                (SparseCoreSolver::BlockWiedemann, false),
                (SparseCoreSolver::BlockLanczos, false),
            ] {
                // The options a deferred matrix hands the solver (private
                // fields, read here so the test uses the same ones).
                let probe = SparseCado::with_core(cols, 101, core, filtered);
                let opts = SparseSolveOptions {
                    filter: probe.filter,
                    solver: probe.core,
                    ..SparseSolveOptions::default()
                };
                let mut sr = SparseRows::for_modulus(&modulus).expect("supported width");
                for (e, rhs) in &raw {
                    sr.push_entries(e, rhs);
                }
                let (outcome, report) = sr.solve(cols, &opts);
                let SparseSolveOutcome::Solved(sol) = outcome else {
                    panic!("{bits} bits {core:?} filtered={filtered}: not solved");
                };
                assert!(report.row_ops > 0);
                for (e, rhs) in &raw {
                    let lhs = e.iter().fold(BigUint::from(0u8), |a, (c, v)| {
                        (a + v * &sol[*c as usize]) % &modulus
                    });
                    assert_eq!(&lhs, rhs, "{bits} bits {core:?} filtered={filtered}");
                }
                assert_eq!(sol, x, "{bits} bits {core:?} filtered={filtered}");
            }
        }
    }

    #[test]
    fn the_registry_resolves_every_matrix_it_advertises() {
        for (name, _) in MATRIX_NAMES {
            assert!(matrix_by_name(name, 4, 101).is_ok(), "{name}");
        }
        assert!(matrix_by_name("block-wiedemann", 4, 101).is_err());
        for (name, _) in MATRIX_NAMES {
            assert_eq!(
                is_deferred_sparse(name),
                matches!(
                    *name,
                    "sparse-cado" | "filter+wiedemann" | "filter+lanczos" | "wiedemann" | "lanczos"
                )
            );
        }
    }
}
