//! Read-only instrumentation of the dense MutantXL closure
//! ([`ReducedBasis::from_system_closed_full`]): the matrix as an object.
//!
//! Nothing here changes what the closure computes or what it charges.  It
//! reads a finished basis (or the unreduced Macaulay input) and reports, per
//! degree block of columns (the columns of monomial degree `d`):
//!
//! * `columns`: how many columns have degree `d`;
//! * `lead_rows`: rows whose leading column has degree `d` (for a reduced
//!   basis this is the rank profile: pivots per block);
//! * `nnz`: set entries of all rows that fall in those columns (density is
//!   `nnz / (rows · columns)`);
//! * the same three restricted to **subspace monomials**, those supported
//!   only on the first `subspace_vars` variables (the `m·ℓ` coordinates of
//!   the summands, each `x(P_i)` being a linear form in them), against the
//!   monomials that touch an intermediate-point coordinate;
//! * the weight (set entries) of rows by leading degree: sum and maximum.
//!
//! It also records the closure's rounds (products inserted, word operations
//! of the batch elimination, rank after) through a thread-local trace that
//! is off unless [`trace_closure`] turned it on.

use super::*;
use std::cell::{Cell, RefCell};

/// Per-degree block statistics; index `d` of [`MatrixProfile::blocks`].
#[derive(Clone, Debug, Default)]
pub struct BlockProfile {
    pub columns: u64,
    pub sub_columns: u64,
    pub lead_rows: u64,
    pub sub_lead_rows: u64,
    pub nnz: u64,
    pub sub_nnz: u64,
    /// Sum and maximum of row weights over rows leading in this block.
    pub lead_weight_sum: u64,
    pub lead_weight_max: u64,
}

/// Profile of one matrix (reduced basis or unreduced Macaulay input).
#[derive(Clone, Debug, Default)]
pub struct MatrixProfile {
    pub rows: u64,
    pub columns: u64,
    /// Number of rows with a pivot (reduced basis only; equals `rows` there).
    pub subspace_vars: u32,
    pub blocks: Vec<BlockProfile>,
}

impl MatrixProfile {
    fn new(columns: u64, rows: u64, degree: u32, subspace_vars: u32) -> Self {
        Self {
            rows,
            columns,
            subspace_vars,
            blocks: vec![BlockProfile::default(); degree as usize + 1],
        }
    }

    fn add_column(&mut self, mono: u64) {
        let d = mono.count_ones() as usize;
        let sub = mono & !sub_mask(self.subspace_vars) == 0;
        let b = &mut self.blocks[d];
        b.columns += 1;
        b.sub_columns += sub as u64;
    }

    /// Account one row given as its monomial list (leading monomial first
    /// in descending column order, which the callers guarantee).
    fn add_row(&mut self, monos: &[u64]) {
        let Some(&lead) = monos.first() else { return };
        let ld = lead.count_ones() as usize;
        let lead_sub = lead & !sub_mask(self.subspace_vars) == 0;
        let w = monos.len() as u64;
        let lb = &mut self.blocks[ld];
        lb.lead_rows += 1;
        lb.sub_lead_rows += lead_sub as u64;
        lb.lead_weight_sum += w;
        lb.lead_weight_max = lb.lead_weight_max.max(w);
        for &m in monos {
            let d = m.count_ones() as usize;
            let sub = m & !sub_mask(self.subspace_vars) == 0;
            let b = &mut self.blocks[d];
            b.nnz += 1;
            b.sub_nnz += sub as u64;
        }
    }

    /// One JSON object (no external dependencies).
    pub fn to_json(&self) -> String {
        let arr = |f: &dyn Fn(&BlockProfile) -> u64| {
            format!(
                "[{}]",
                self.blocks
                    .iter()
                    .map(|b| f(b).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        };
        format!(
            r#"{{"rows":{},"columns":{},"subspace_vars":{},"cols":{},"sub_cols":{},"lead_rows":{},"sub_lead_rows":{},"nnz":{},"sub_nnz":{},"lead_weight_sum":{},"lead_weight_max":{}}}"#,
            self.rows,
            self.columns,
            self.subspace_vars,
            arr(&|b| b.columns),
            arr(&|b| b.sub_columns),
            arr(&|b| b.lead_rows),
            arr(&|b| b.sub_lead_rows),
            arr(&|b| b.nnz),
            arr(&|b| b.sub_nnz),
            arr(&|b| b.lead_weight_sum),
            arr(&|b| b.lead_weight_max),
        )
    }
}

fn sub_mask(vars: u32) -> u64 {
    if vars >= 64 {
        u64::MAX
    } else {
        (1u64 << vars) - 1
    }
}

/// One round of [`ReducedBasis::close`].
#[derive(Clone, Copy, Debug, Default)]
pub struct ClosureRound {
    pub products: u64,
    /// Word operations of the batch elimination of this round
    /// (`reduce_word_ops` only; the product construction is separate).
    pub reduce_word_ops: u64,
    pub rows_after: u64,
    pub columns: u64,
}

thread_local! {
    static TRACE_ON: Cell<bool> = const { Cell::new(false) };
    static TRACE: RefCell<Vec<ClosureRound>> = const { RefCell::new(Vec::new()) };
}

/// Turn the closure-round trace on or off for this thread and clear it.
pub fn trace_closure(on: bool) {
    TRACE_ON.with(|t| t.set(on));
    TRACE.with(|t| t.borrow_mut().clear());
}

/// The rounds recorded since [`trace_closure`] was last called.
pub fn take_closure_trace() -> Vec<ClosureRound> {
    TRACE.with(|t| std::mem::take(&mut *t.borrow_mut()))
}

pub(super) fn note_round(round: ClosureRound) {
    if TRACE_ON.with(Cell::get) {
        TRACE.with(|t| t.borrow_mut().push(round));
    }
}

/// Profile of the unreduced Macaulay matrix of `system` at `degree` (the
/// rows XL would eliminate: one generator times one monomial each).
/// `None` past the F4 size caps.
pub fn macaulay_input_profile(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    subspace_vars: u32,
) -> Option<MatrixProfile> {
    let root = RootMatrix::build(system, n_vars, degree, false)?;
    let mut prof = MatrixProfile::new(
        root.columns.len() as u64,
        root.matrix.len() as u64,
        degree,
        subspace_vars,
    );
    for &c in &root.columns {
        prof.add_column(c);
    }
    let mut monos: Vec<u64> = Vec::new();
    for row in &root.matrix {
        monos.clear();
        for (w, &word) in row.iter().enumerate() {
            let mut bits = word;
            while bits != 0 {
                let b = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                monos.push(root.columns[w * 64 + b]);
            }
        }
        prof.add_row(&monos);
    }
    Some(prof)
}

impl ReducedBasis {
    /// Profile of this basis.  Rows are first brought into the current
    /// layout (a closed basis has lazily extended rows); that bookkeeping
    /// is charged to a scratch cost and does not touch the closure's own.
    pub fn profile(&mut self, subspace_vars: u32) -> MatrixProfile {
        self.materialise_all(&mut InheritCost::default());
        let mut prof = MatrixProfile::new(
            self.columns.len() as u64,
            self.rows.len() as u64,
            self.degree,
            subspace_vars,
        );
        for &c in &self.columns {
            prof.add_column(c);
        }
        let mut monos: Vec<u64> = Vec::new();
        for row in &self.rows {
            monos.clear();
            for (i, &word) in row.live().iter().enumerate() {
                let base = (row.lead as usize + i) * 64;
                let mut bits = word;
                while bits != 0 {
                    let b = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    monos.push(self.columns[base + b]);
                }
            }
            prof.add_row(&monos);
        }
        prof
    }

    /// [`ReducedBasis::from_system_closed_full`] with the cost split into
    /// the Macaulay reduction (`base`) and the closure (`total − base`), and
    /// an optional profile of the plain (XL) echelon basis taken before the
    /// closure.  The closure itself is the same code path.
    pub fn closed_full_split(
        system: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        rounds: u32,
        pre_profile: Option<u32>,
    ) -> Option<(Self, InheritCost, InheritCost, Option<MatrixProfile>)> {
        let (mut basis, mut cost) = Self::from_system_with(system, n_vars, degree, false)?;
        let base = cost;
        let pre = pre_profile.map(|v| basis.profile(v));
        let min_fall_degree = 1u32;
        if rounds == 0 || degree < min_fall_degree + 1 || basis.rows.is_empty() || basis.refuted {
            return Some((basis, cost, base, pre));
        }
        basis.closure_rounds = rounds;
        basis.min_fall_degree = min_fall_degree;
        for r in 0..basis.rows.len() {
            if basis.is_fall(r) {
                basis.pending.push(r as u32);
            }
        }
        basis.close(&mut cost);
        Some((basis, cost, base, pre))
    }
}
