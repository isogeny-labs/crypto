//! Report section 6: the derived gain-aware DSU for two-entry LP rows.
//!
//! Status in the report: **derived [pcc:unverified]**. This module implements
//! that derivation and checks it against exact Gaussian elimination in tests.
//!
//! Each LP column `u` carries a potential `x_u = p_u * t_root(u)` with
//! `p_u != 0`. Every component stores a flag `U` meaning its homogeneous root
//! parameter has been forced to zero ("pinned"). For a row `a x_u + b x_v`:
//!
//! | case | update | rank increment |
//! |---|---|---|
//! | different roots | `t_ru = -(b p_v)/(a p_u) t_rv`, flag `U1 or U2` | 1 unless both flagged |
//! | same root | `gamma = a p_u + b p_v`; pin if `gamma != 0` and unflagged | 1 on that first pin |
//! | one nonzero | pin the component | 1 if it was unflagged |
//! | zero row | none | 0 (dependent) |
//!
//! `rank(A_L) = v - #unflagged components` and
//! `dim ker(A_L^T) = R - rank(A_L)`. Cost: `O((v + R) alpha(v))` pointer and
//! field operations (division counted as one field operation).
//!
//! Optional witness retention ([`GainDsu::with_witnesses`]) keeps the
//! accepted rows as an explicit forest so a dependent row can be expanded to a
//! left-kernel vector `y` with `y^T A_L = 0`. Expansion cost is extra and is
//! charged to the caller, as the report requires.

use super::field::Fp;
use std::collections::HashMap;

/// Outcome of inserting one row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowEffect {
    /// The row raised the LP rank by one.
    Independent,
    /// The row is in the span of earlier rows: it closes a dependency.
    Dependent,
}

/// Row in canonical form: at most two distinct LP columns, nonzero coefficients.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoEntryRow {
    pub entries: Vec<(usize, u64)>,
}

/// Gain-aware disjoint-set union over `F_p`.
#[derive(Clone, Debug)]
pub struct GainDsu {
    f: Fp,
    parent: Vec<usize>,
    /// `x_u = ratio[u] * x_parent[u]`; for a root, 1.
    ratio: Vec<u64>,
    size: Vec<usize>,
    pinned: Vec<bool>,
    unflagged: usize,
    rows: usize,
    rank: usize,
    /// Pointer and field operation counters (the report's cost model).
    pub pointer_ops: u64,
    pub field_ops: u64,
    witnesses: Option<WitnessStore>,
}

#[derive(Clone, Debug, Default)]
struct WitnessStore {
    /// All processed rows (canonical), by row index.
    rows: Vec<Vec<(usize, u64)>>,
    /// Rows accepted as independent (a basis of the row space).
    basis_rows: Vec<usize>,
    /// For each component root's members: basis rows touching a column.
    col_rows: HashMap<usize, Vec<usize>>,
}

impl GainDsu {
    /// `columns` = number of LP columns `v`, including isolated ones.
    pub fn new(columns: usize, field: Fp) -> Self {
        Self {
            f: field,
            parent: (0..columns).collect(),
            ratio: vec![1; columns],
            size: vec![1; columns],
            pinned: vec![false; columns],
            unflagged: columns,
            rows: 0,
            rank: 0,
            pointer_ops: 0,
            field_ops: 0,
            witnesses: None,
        }
    }

    /// Also retain rows so dependencies can be expanded to kernel vectors.
    pub fn with_witnesses(mut self) -> Self {
        self.witnesses = Some(WitnessStore::default());
        self
    }

    /// Find with path compression; returns `(root, p_u)` with `x_u = p_u t_root`.
    pub fn find(&mut self, u: usize) -> (usize, u64) {
        // First pass: walk to the root collecting the path.
        let mut path = Vec::new();
        let mut x = u;
        while self.parent[x] != x {
            path.push(x);
            x = self.parent[x];
            self.pointer_ops += 1;
        }
        let root = x;
        // Second pass from the top: compress and accumulate ratios.
        let mut acc = 1u64;
        for &y in path.iter().rev() {
            acc = self.f.mul(self.ratio[y], acc);
            self.field_ops += 1;
            self.ratio[y] = acc;
            self.parent[y] = root;
        }
        let p = if u == root { 1 } else { self.ratio[u] };
        (root, p)
    }

    /// Canonicalize: combine repeated columns, drop zeros.
    pub fn canonicalize(&self, entries: &[(usize, u64)]) -> Vec<(usize, u64)> {
        let mut out: Vec<(usize, u64)> = Vec::new();
        for &(c, a) in entries {
            let a = self.f.reduce(a);
            if let Some(slot) = out.iter_mut().find(|(d, _)| *d == c) {
                slot.1 = self.f.add(slot.1, a);
            } else {
                out.push((c, a));
            }
        }
        out.retain(|&(_, a)| a != 0);
        out.sort_unstable();
        out
    }

    /// Insert a row with at most two distinct LP columns (after
    /// canonicalization). Returns an error for wider rows.
    pub fn add_row(&mut self, entries: &[(usize, u64)]) -> Result<RowEffect, &'static str> {
        let row = self.canonicalize(entries);
        if row.len() > 2 {
            return Err("gain DSU accepts rows with at most two nonzero LP coefficients");
        }
        if row.iter().any(|&(c, _)| c >= self.parent.len()) {
            return Err("column out of range");
        }
        let idx = self.rows;
        self.rows += 1;
        let effect = match row.as_slice() {
            [] => RowEffect::Dependent,
            [(u, _)] => {
                let (r, _) = self.find(*u);
                if self.pinned[r] {
                    RowEffect::Dependent
                } else {
                    self.pinned[r] = true;
                    self.unflagged -= 1;
                    RowEffect::Independent
                }
            }
            [(u, a), (v, b)] => {
                let (ru, pu) = self.find(*u);
                let (rv, pv) = self.find(*v);
                if ru != rv {
                    let both = self.pinned[ru] && self.pinned[rv];
                    let any = self.pinned[ru] || self.pinned[rv];
                    // a pu t_ru + b pv t_rv = 0.
                    let (child, parent, ratio) = if self.size[ru] <= self.size[rv] {
                        // t_ru = -(b pv)/(a pu) t_rv
                        let num = self.f.neg(self.f.mul(*b, pv));
                        let den = self.f.mul(*a, pu);
                        (ru, rv, self.f.div(num, den).expect("nonzero"))
                    } else {
                        let num = self.f.neg(self.f.mul(*a, pu));
                        let den = self.f.mul(*b, pv);
                        (rv, ru, self.f.div(num, den).expect("nonzero"))
                    };
                    self.field_ops += 4;
                    self.parent[child] = parent;
                    self.ratio[child] = ratio;
                    self.size[parent] += self.size[child];
                    let unflag_before =
                        usize::from(!self.pinned[ru]) + usize::from(!self.pinned[rv]);
                    self.pinned[parent] = any;
                    self.unflagged = self.unflagged - unflag_before + usize::from(!any);
                    if both {
                        RowEffect::Dependent
                    } else {
                        RowEffect::Independent
                    }
                } else {
                    let gamma = self.f.add(self.f.mul(*a, pu), self.f.mul(*b, pv));
                    self.field_ops += 3;
                    if gamma != 0 && !self.pinned[ru] {
                        self.pinned[ru] = true;
                        self.unflagged -= 1;
                        RowEffect::Independent
                    } else {
                        RowEffect::Dependent
                    }
                }
            }
            _ => unreachable!(),
        };
        if effect == RowEffect::Independent {
            self.rank += 1;
        }
        if let Some(w) = self.witnesses.as_mut() {
            w.rows.push(row.clone());
            if effect == RowEffect::Independent {
                w.basis_rows.push(idx);
                for &(c, _) in &row {
                    w.col_rows.entry(c).or_default().push(idx);
                }
            }
        }
        Ok(effect)
    }

    pub fn rows_processed(&self) -> usize {
        self.rows
    }
    pub fn columns(&self) -> usize {
        self.parent.len()
    }
    /// `rank(A_L) = v - #unflagged components`.
    pub fn rank(&self) -> usize {
        debug_assert_eq!(self.rank, self.parent.len() - self.unflagged);
        self.parent.len() - self.unflagged
    }
    /// `dim ker(A_L^T) = R - rank(A_L)`.
    pub fn left_nullity(&self) -> usize {
        self.rows - self.rank()
    }
    pub fn unflagged_components(&self) -> usize {
        self.unflagged
    }
    pub fn is_pinned(&mut self, u: usize) -> bool {
        let (r, _) = self.find(u);
        self.pinned[r]
    }

    /// Expand a dependent row (by its insertion index) into a left-kernel
    /// vector over the processed rows: sparse `y` with `y^T A_L = 0` and
    /// `y[row] = 1`. Requires [`GainDsu::with_witnesses`]. The expansion
    /// solves the dependency over the basis rows of the row's component with
    /// sparse elimination; its cost is outside the DSU metadata bound.
    pub fn dependency_witness(&self, row: usize) -> Option<Vec<(usize, u64)>> {
        let w = self.witnesses.as_ref()?;
        let target = w.rows.get(row)?;
        if w.basis_rows.binary_search(&row).is_ok() {
            return None;
        }
        if target.is_empty() {
            return Some(vec![(row, 1)]);
        }
        // Collect basis rows reachable through shared columns (the component).
        let mut cols: Vec<usize> = target.iter().map(|&(c, _)| c).collect();
        let mut seen_cols: std::collections::HashSet<usize> = cols.iter().copied().collect();
        let mut rows_in: Vec<usize> = Vec::new();
        let mut seen_rows = std::collections::HashSet::new();
        while let Some(c) = cols.pop() {
            if let Some(rs) = w.col_rows.get(&c) {
                for &r in rs {
                    if seen_rows.insert(r) {
                        rows_in.push(r);
                        for &(d, _) in &w.rows[r] {
                            if seen_cols.insert(d) {
                                cols.push(d);
                            }
                        }
                    }
                }
            }
        }
        // Solve sum_r y_r row_r = -target over those columns.
        let col_list: Vec<usize> = seen_cols.into_iter().collect();
        let col_idx: HashMap<usize, usize> =
            col_list.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        let k = rows_in.len();
        let m = col_list.len();
        // Matrix M (m x k) with M[c][r] = coef; rhs = -target.
        let mut mat = vec![vec![0u64; k + 1]; m];
        for (j, &r) in rows_in.iter().enumerate() {
            for &(c, a) in &w.rows[r] {
                mat[col_idx[&c]][j] = a;
            }
        }
        for &(c, a) in target {
            mat[col_idx[&c]][k] = self.f.neg(a);
        }
        let sol = super::dense::solve_augmented(self.f, mat, k)?;
        let mut y: Vec<(usize, u64)> = rows_in
            .iter()
            .zip(sol)
            .filter(|&(_, s)| s != 0)
            .map(|(&r, s)| (r, s))
            .collect();
        y.push((row, 1));
        y.sort_unstable();
        Some(y)
    }

    /// Recompute `y^T A_L` over the retained rows; empty when `y` cancels.
    pub fn apply_left(&self, y: &[(usize, u64)]) -> Option<Vec<(usize, u64)>> {
        let w = self.witnesses.as_ref()?;
        let mut acc: HashMap<usize, u64> = HashMap::new();
        for &(r, s) in y {
            for &(c, a) in &w.rows[r] {
                let e = acc.entry(c).or_insert(0);
                *e = self.f.mul_add(*e, s, a);
            }
        }
        let mut out: Vec<(usize, u64)> = acc.into_iter().filter(|&(_, v)| v != 0).collect();
        out.sort_unstable();
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_graph::dense::DenseMatrix;
    use crate::cryptanalysis::ic_graph::field::SplitMix64;

    fn exact_rank(f: Fp, rows: &[Vec<(usize, u64)>], cols: usize) -> usize {
        let mut m = DenseMatrix::zeros(rows.len(), cols);
        for (i, r) in rows.iter().enumerate() {
            for &(c, a) in r {
                m.set(i, c, f.add(m.get(i, c), f.reduce(a)));
            }
        }
        m.rank(f)
    }

    #[test]
    fn triangle_has_no_cancellation_over_odd_field() {
        // Report section 5: rows x+y, y+z, z+x have det 2.
        let f = Fp::new(1_000_003).unwrap();
        let mut d = GainDsu::new(3, f);
        assert_eq!(
            d.add_row(&[(0, 1), (1, 1)]).unwrap(),
            RowEffect::Independent
        );
        assert_eq!(
            d.add_row(&[(1, 1), (2, 1)]).unwrap(),
            RowEffect::Independent
        );
        assert_eq!(
            d.add_row(&[(2, 1), (0, 1)]).unwrap(),
            RowEffect::Independent
        );
        assert_eq!(d.rank(), 3);
        assert_eq!(d.left_nullity(), 0);
    }

    #[test]
    fn balanced_cycle_gives_dependency_with_witness() {
        // x - y, y - z, z - x: balanced, so the third row is dependent.
        let f = Fp::new(101).unwrap();
        let mut d = GainDsu::new(3, f).with_witnesses();
        d.add_row(&[(0, 1), (1, 100)]).unwrap();
        d.add_row(&[(1, 1), (2, 100)]).unwrap();
        assert_eq!(
            d.add_row(&[(2, 1), (0, 100)]).unwrap(),
            RowEffect::Dependent
        );
        let y = d.dependency_witness(2).unwrap();
        assert!(d.apply_left(&y).unwrap().is_empty());
    }

    #[test]
    fn handcuff_bridge_dependency() {
        // Report section 5: triangles abc, def joined by bridge cd, coefficient
        // +1 at both endpoints; the dependency uses the bridge with coefficient 2.
        let f = Fp::new(7).unwrap();
        let (a, b, c, dd, e, ff) = (0, 1, 2, 3, 4, 5);
        let edges = [(a, b), (b, c), (c, a), (dd, e), (e, ff), (ff, dd), (c, dd)];
        let mut g = GainDsu::new(6, f).with_witnesses();
        let mut effects = Vec::new();
        for &(u, v) in &edges {
            effects.push(g.add_row(&[(u, 1), (v, 1)]).unwrap());
        }
        assert_eq!(g.rank(), 6);
        assert_eq!(g.left_nullity(), 1);
        let dep = effects
            .iter()
            .position(|&x| x == RowEffect::Dependent)
            .unwrap();
        let y = g.dependency_witness(dep).unwrap();
        assert!(g.apply_left(&y).unwrap().is_empty());
        // Report's coefficients (1,-1,-1,-1,1,-1,2) are a kernel vector.
        let rep = [1i64, -1, -1, -1, 1, -1, 2];
        let yv: Vec<(usize, u64)> = rep
            .iter()
            .enumerate()
            .map(|(i, &c)| (i, f.reduce_i64(c)))
            .collect();
        assert!(g.apply_left(&yv).unwrap().is_empty());
    }

    #[test]
    fn matches_exact_rank_on_random_two_entry_rows() {
        for &p in &[3u64, 5, 101, 1_000_000_007] {
            let f = Fp::new(p).unwrap();
            let mut rng = SplitMix64::new(p);
            for trial in 0..40 {
                let cols = 2 + rng.below(12) as usize;
                let nrows = rng.below(20) as usize;
                let mut d = GainDsu::new(cols, f).with_witnesses();
                let mut rows = Vec::new();
                for _ in 0..nrows {
                    let kind = rng.below(10);
                    let row: Vec<(usize, u64)> = if kind == 0 {
                        vec![]
                    } else if kind < 3 {
                        vec![(rng.below(cols as u64) as usize, rng.element(f))]
                    } else {
                        vec![
                            (rng.below(cols as u64) as usize, rng.element(f)),
                            (rng.below(cols as u64) as usize, rng.element(f)),
                        ]
                    };
                    let eff = d.add_row(&row).unwrap();
                    rows.push(row);
                    let r = exact_rank(f, &rows, cols);
                    assert_eq!(d.rank(), r, "p {p} trial {trial}");
                    if eff == RowEffect::Dependent {
                        let y = d.dependency_witness(rows.len() - 1).unwrap();
                        assert!(d.apply_left(&y).unwrap().is_empty());
                    }
                }
                assert_eq!(d.left_nullity(), rows.len() - d.rank());
            }
        }
    }

    #[test]
    fn rejects_wide_rows() {
        let f = Fp::new(11).unwrap();
        let mut d = GainDsu::new(4, f);
        assert!(d.add_row(&[(0, 1), (1, 1), (2, 1)]).is_err());
        // Repeated columns combine first: x0 + x0 - 2 x0 is the zero row.
        assert_eq!(
            d.add_row(&[(0, 1), (0, 1), (0, 9)]).unwrap(),
            RowEffect::Dependent
        );
    }
}
