//! **E42 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.54–§9.55): the stored
//! tails of E40's V.**
//!
//! Stage 0 (counts only): [`stage0_tails`] rebuilds one call's
//! weighted-degree-`d` Macaulay matrix exactly as
//! `macaulay_profile::build_profile` builds it for V (row selection on, each
//! row from its lead) and replays A's forward echelon (E33's `Lowest`, the
//! same buckets, pivots and dense/sparse branch rule) while recording, per
//! row and cell: the right extent at build (the column of `TM(fᵢ)·shift`,
//! `TM` the trailing monomial), when fill pushes it further right (the
//! growth a row stored only up to its fill would need, and when it arrives),
//! which tail cells are ever nonzero, which tail cells V's echelon and the
//! reachability/normal-form reads touch, and a lower bound on a sparse
//! format's index work.  It is a diagnostic: nothing here is charged to any
//! solve.

use std::collections::HashMap;

use serde::Serialize;

use super::gaudry_cubic::am;
use super::gaudry_quartic::{echelon_by_lead_counted, PivotCheck, PivotRule, PivotWork};
use super::klein_v4::{triangularise, wdeg, weighted_monomials_desc};
use super::residual_walk::inv_mod;

/// `klein_v4`'s `EIG_VAR`.
const EIG_VAR: usize = 3;
/// Bins of the update column `c` (equal widths of `cols`) for fill arrival.
pub const FILL_BINS: usize = 8;

/// One call's tails, counted (Stage 0, §9.54).  "Extent" of a row is its
/// stored range `[lead, end)`; `G` is a row stored up to its fill: at build
/// to its last coefficient, grown on an update to the pivot row's last
/// nonzero when that lies further right.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TailStage0 {
    pub rows: u64,
    pub cols: u64,
    /// V's stored cells, `Σ_r (cols − lead_r)`.
    pub tail_cells: u64,
    /// `Σ_r (end⁰_r − lead_r)`, `end⁰_r` one past the row's last nonzero
    /// coefficient at build.
    pub build_extent_cells: u64,
    /// `Σ_r (end_r − lead_r)` after the echelon under `G`.
    pub final_extent_cells: u64,
    /// Oracle: `Σ_r (maxnz_r + 1 − lead_r)`, `maxnz_r` the rightmost column
    /// the row is ever nonzero in (a lower bound on any contiguous storage).
    pub ever_extent_cells: u64,
    /// Tail cells ever nonzero (at build or after any update).
    pub ever_nonzero_cells: u64,
    /// Tail cells nonzero after the echelon.
    pub final_nonzero_cells: u64,
    /// Rows whose build end is not one past the column of `TM(fᵢ)·shift`
    /// (expected 0: the order is a monomial order).
    pub end_not_tm_shift: u64,
    /// Coefficient writes, and those with a zero coefficient.
    pub term_writes: u64,
    pub zero_coefficient_writes: u64,
    /// `Σ_i |supp fᵢ|` over the triangularised equations that build rows
    /// (the trailing-monomial scans).
    pub support_terms: u64,
    pub pivots: u64,
    pub zero_rows: u64,
    /// Row updates and their branch (E33's `|nz|·8 > cols − c`).
    pub updates: u64,
    pub dense_updates: u64,
    /// Cells the dense branch's `axpy` runs over: V `Σ (cols − c)`, `G`
    /// `Σ (pivot end − c)`.
    pub dense_update_cells_v: u64,
    pub dense_update_cells_g: u64,
    /// Updates that push the row's end right under `G`, the rows that ever
    /// grow, and the most growths of one row.
    pub growth_events: u64,
    pub rows_grown: u64,
    pub max_growths_per_row: u64,
    /// Cells added by growth, `Σ (new end − old end)`.
    pub growth_cells: u64,
    /// Cells a growth copies if the row keeps its lead (`Σ (old end −
    /// lead)`), and the length of a growth that re-allocates the row from
    /// the update column (`Σ (new end − c)`).
    pub growth_copy_keep_lead: u64,
    pub growth_realloc_from_c: u64,
    /// Growth cells that arrive on the row's first update.
    pub growth_cells_first_update: u64,
    /// Growth cells and events by the update column (bins of `cols`).
    pub growth_cells_by_bin: Vec<u64>,
    pub growth_events_by_bin: Vec<u64>,
    /// Lower bound on a sparse row format's index work: per update the
    /// merge of the row's and the pivot's nonzeros from `c` on.
    pub sparse_merge_lower_bound: u64,
    /// Distinct tail cells read or written by V's build, echelon (first
    /// lead read, normalisation scans, updates, lead scans) and the
    /// reachability pass (which covers the normal forms' reads), and the
    /// same under `G`'s extents.
    pub touched_v: u64,
    pub touched_g: u64,
    /// Reached pivot rows and the cells right of their pivot (V / `G`).
    pub reached: u64,
    pub reach_cells_v: u64,
    pub reach_cells_g: u64,
    /// Rows outside the normal forms' dependency closure and their `G`
    /// extents.
    pub dependency_rows: u64,
    pub outside_dependency_final_cells: u64,
    /// Static reserves (v2): each row allocated at build to `cap_k`, `k = 0`
    /// its build end, `k = 1` the end of the weighted-degree block holding its
    /// build end, `k = 2` the end of the next block; an update whose pivot
    /// ends beyond `cap_k` re-allocates the row to the end of the block
    /// holding the new end (copying the old cells).  Per `k`: cells allocated
    /// `Σ (cap − lead)` (re-allocations included), re-allocations, cells
    /// copied, rows that ever re-allocate.
    pub reserve_cells: [u64; 3],
    pub reserve_reallocs: [u64; 3],
    pub reserve_copy_cells: [u64; 3],
    pub reserve_realloc_rows: [u64; 3],
    /// Two segments (v2): the build range plus an overflow grown exactly;
    /// cells an overflow growth copies (`Σ (old end − end⁰)`), and updates of
    /// a row that already holds an overflow (a split of the update).
    pub overflow_copy_cells: u64,
    pub overflow_split_updates: u64,
    /// Columns out of weighted-degree order (expected 0) and the block sizes.
    pub columns_out_of_degree_order: u64,
    /// Per weighted degree of the lead monomial: `[rows, tail, build extent,
    /// final extent, ever nonzero]`.
    pub by_lead_degree: Vec<[u64; 5]>,
    /// The replay reproduced `echelon_by_lead_counted(…, Lowest, …)`'s
    /// pivots and rows.
    pub echelon_reproduced: bool,
    pub unreachable: bool,
}

/// Stage 0 of E42 on one call's system (`v4_system`'s components) at
/// weighted degree `d`.  `None` when an equation lies above `d`.
pub fn stage0_tails(comps: &[HashMap<[u8; 4], u64>], d: u16, p: u64) -> Option<TailStage0> {
    let mut s = TailStage0::default();
    let cols = weighted_monomials_desc(d);
    let ncols = cols.len();
    s.cols = ncols as u64;
    let col_index: HashMap<[u8; 4], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    // One past the last column of each column's weighted-degree block.
    let mut block_end = vec![ncols; ncols];
    for j in (0..ncols.saturating_sub(1)).rev() {
        let (a, b) = (wdeg(&cols[j]), wdeg(&cols[j + 1]));
        if a < b {
            s.columns_out_of_degree_order += 1;
        }
        block_end[j] = if a == b { block_end[j + 1] } else { j + 1 };
    }
    let reserve = |k: usize, e: usize| -> usize {
        // e = one past the row's last stored column (≥ 1).
        match k {
            0 => e,
            1 => block_end[e - 1],
            _ => {
                let b = block_end[e - 1];
                if b < ncols {
                    block_end[b]
                } else {
                    b
                }
            }
        }
    };
    let mut tri = 0u64;
    let (comps, leads) = triangularise(comps, p, &mut tri);
    let divides = |lm: &[u8; 4], m: &[u8; 4]| (0..4).all(|k| lm[k] <= m[k]);
    let mut rows: Vec<Vec<u16>> = Vec::new();
    let mut lead: Vec<usize> = Vec::new();
    let mut end: Vec<usize> = Vec::new();
    for (i, comp) in comps.iter().enumerate() {
        let nzterms: Vec<[u8; 4]> = comp
            .iter()
            .filter(|(_, &v)| v % p != 0)
            .map(|(m, _)| *m)
            .collect();
        let Some(dg) = nzterms.iter().map(wdeg).max() else {
            continue;
        };
        if dg > d {
            return None;
        }
        s.support_terms += comp.len() as u64;
        // The trailing monomial: the column order is a monomial order, so
        // the last column of a shifted row is that of TM·shift.
        let tm = nzterms
            .iter()
            .max_by_key(|m| col_index[*m])
            .copied()
            .expect("nonzero");
        let lm = leads[i].expect("a nonzero equation has a lead");
        for sh in weighted_monomials_desc(d - dg) {
            if leads
                .iter()
                .take(i)
                .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
            {
                continue;
            }
            let l: [u8; 4] = core::array::from_fn(|k| lm[k] + sh[k]);
            let t: [u8; 4] = core::array::from_fn(|k| tm[k] + sh[k]);
            let mut row = vec![0u16; ncols];
            let mut e0 = 0usize;
            for (&e, &c) in comp {
                let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                let ci = col_index[&m];
                s.term_writes += 1;
                if c % p == 0 {
                    s.zero_coefficient_writes += 1;
                } else {
                    e0 = e0.max(ci + 1);
                }
                row[ci] = am(u64::from(row[ci]), c, p) as u16;
            }
            if e0 != col_index[&t] + 1 {
                s.end_not_tm_shift += 1;
            }
            rows.push(row);
            lead.push(col_index[&l]);
            end.push(e0);
        }
    }
    let nrows = rows.len();
    s.rows = nrows as u64;
    let build_end = end.clone();
    // Cells G has allocated per row (a pivot's truncation frees none).
    let mut alloc = end.clone();
    let mut cap: Vec<[usize; 3]> = end
        .iter()
        .map(|&e| core::array::from_fn(|k| reserve(k, e)))
        .collect();
    let mut reallocated = vec![[false; 3]; nrows];
    let mut ever: Vec<Vec<bool>> = rows
        .iter()
        .map(|r| r.iter().map(|&x| x != 0).collect())
        .collect();
    let mut tv: Vec<Vec<bool>> = rows
        .iter()
        .map(|r| r.iter().map(|&x| x != 0).collect())
        .collect();
    let mut tg = tv.clone();
    let mut growths = vec![0u64; nrows];
    let mut updated = vec![false; nrows];
    s.growth_cells_by_bin = vec![0; FILL_BINS];
    s.growth_events_by_bin = vec![0; FILL_BINS];
    let bin = |c: usize| (c * FILL_BINS / ncols).min(FILL_BINS - 1);
    let mark = |t: &mut Vec<Vec<bool>>, r: usize, a: usize, b: usize| {
        for x in &mut t[r][a..b.max(a)] {
            *x = true;
        }
    };
    // A's echelon (Lowest), replayed on dense rows with G's extents kept.
    let pp = p as u32;
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    for (r, row) in rows.iter().enumerate() {
        if let Some(c) = row.iter().position(|&x| x != 0) {
            // V's (and G's) first lead read: the offset cell.
            tv[r][c] = true;
            tg[r][c] = true;
            buckets[c].push(r as u32);
        }
    }
    let mut by: Vec<Vec<u32>> = vec![Vec::new(); nrows];
    let mut pivot_row = vec![u32::MAX; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        b.sort_unstable();
        let pr = b[0] as usize;
        let mut piv = std::mem::take(&mut rows[pr]);
        // Normalisation scans the pivot's stored range from c.
        mark(&mut tv, pr, c, ncols);
        mark(&mut tg, pr, c, end[pr]);
        let inv = inv_mod(u64::from(piv[c]), p) as u32;
        let mut nz: Vec<usize> = Vec::new();
        for (j, x) in piv.iter_mut().enumerate().skip(c) {
            if *x != 0 {
                *x = ((u32::from(*x) * inv) % pp) as u16;
                nz.push(j);
            }
        }
        let pend = nz.last().map_or(c + 1, |&j| j + 1);
        debug_assert!(pend <= end[pr]);
        // G truncates the pivot to its last nonzero (no cell is built).
        end[pr] = pend;
        let dense = nz.len() * 8 > ncols - c;
        for &r in &b[1..] {
            let r = r as usize;
            s.updates += 1;
            by[r].push(pr as u32);
            let row_nnz = rows[r][c..].iter().filter(|&&x| x != 0).count();
            if end[r] > build_end[r] {
                s.overflow_split_updates += 1;
            }
            for k in 0..3 {
                if pend > cap[r][k] {
                    s.reserve_reallocs[k] += 1;
                    s.reserve_copy_cells[k] += (cap[r][k] - lead[r]) as u64;
                    reallocated[r][k] = true;
                    cap[r][k] = reserve(k.min(1), pend);
                }
            }
            s.sparse_merge_lower_bound += (row_nnz + nz.len()) as u64;
            if pend > end[r] {
                s.growth_events += 1;
                growths[r] += 1;
                s.growth_cells += (pend - end[r]) as u64;
                s.growth_copy_keep_lead += (end[r] - lead[r]) as u64;
                s.overflow_copy_cells += (end[r] - build_end[r]) as u64;
                s.growth_realloc_from_c += (pend - c) as u64;
                s.growth_cells_by_bin[bin(c)] += (pend - end[r]) as u64;
                s.growth_events_by_bin[bin(c)] += 1;
                if !updated[r] {
                    s.growth_cells_first_update += (pend - end[r]) as u64;
                }
                end[r] = pend;
                alloc[r] = pend;
            }
            updated[r] = true;
            if dense {
                s.dense_updates += 1;
                s.dense_update_cells_v += (ncols - c) as u64;
                s.dense_update_cells_g += (pend - c) as u64;
                mark(&mut tv, r, c, ncols);
                mark(&mut tg, r, c, pend);
            } else {
                for &j in &nz {
                    tv[r][j] = true;
                    tg[r][j] = true;
                }
            }
            let row = &mut rows[r];
            let f = pp - u32::from(row[c]);
            for &j in &nz {
                row[j] = ((u32::from(row[j]) + f * u32::from(piv[j])) % pp) as u16;
                if row[j] != 0 {
                    ever[r][j] = true;
                }
            }
            // The lead scan after the update.
            match row[c + 1..].iter().position(|&x| x != 0) {
                Some(k) => {
                    mark(&mut tv, r, c + 1, c + 2 + k);
                    mark(&mut tg, r, c + 1, c + 2 + k);
                    buckets[c + 1 + k].push(r as u32);
                }
                None => {
                    mark(&mut tv, r, c + 1, ncols);
                    mark(&mut tg, r, c + 1, end[r]);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    s.max_growths_per_row = growths.iter().copied().max().unwrap_or(0);
    s.rows_grown = growths.iter().filter(|&&g| g > 0).count() as u64;
    s.pivots = pivot_row.iter().filter(|&&r| r != u32::MAX).count() as u64;
    // The check: the same pivots and rows as E33's counted echelon (A).
    {
        let mut dense_rows: Vec<Vec<u16>> = Vec::with_capacity(nrows);
        // Rebuild the build rows (the replay consumed them).
        for (i, comp) in comps.iter().enumerate() {
            let Some(dg) = comp
                .iter()
                .filter(|(_, &v)| v % p != 0)
                .map(|(m, _)| wdeg(m))
                .max()
            else {
                continue;
            };
            for sh in weighted_monomials_desc(d - dg) {
                if leads
                    .iter()
                    .take(i)
                    .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
                {
                    continue;
                }
                let mut row = vec![0u16; ncols];
                for (&e, &c) in comp {
                    let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                    let ci = col_index[&m];
                    row[ci] = am(u64::from(row[ci]), c, p) as u16;
                }
                dense_rows.push(row);
            }
        }
        let mut w = PivotWork::default();
        let mut muls = 0u64;
        let ech = echelon_by_lead_counted(
            dense_rows,
            pp,
            &mut muls,
            PivotRule::Lowest,
            None,
            PivotCheck::default(),
            &mut w,
        );
        s.zero_rows = ech.zero_rows as u64;
        s.echelon_reproduced = ech.pivot_row == pivot_row && ech.rows == rows;
    }
    // The standard set, the P₂-chains' border targets and the reachability
    // pass (klein_v4::chain_quotient); the normal forms read a subset of
    // the cells the reachability pass reads.
    let is_std: Vec<bool> = (0..ncols)
        .map(|c| pivot_row[c] == u32::MAX && wdeg(&cols[c]) < d)
        .collect();
    let shift = |c: usize, up: bool| -> Option<usize> {
        let mut m = cols[c];
        if up {
            m[EIG_VAR] += 1;
        } else {
            if m[EIG_VAR] == 0 {
                return None;
            }
            m[EIG_VAR] -= 1;
        }
        col_index.get(&m).copied()
    };
    let mut seen = vec![false; ncols];
    let mut stack: Vec<usize> = Vec::new();
    for h in (0..ncols).filter(|&c| is_std[c]) {
        if shift(h, false).is_some_and(|j| is_std[j]) {
            continue;
        }
        let mut c = h;
        let t = loop {
            match shift(c, true) {
                Some(n) if is_std[n] => c = n,
                other => break other,
            }
        };
        match t {
            Some(t) if !seen[t] => {
                seen[t] = true;
                stack.push(t);
            }
            Some(_) => {}
            None => s.unreachable = true,
        }
    }
    let mut dep = vec![false; nrows];
    let mut dstack: Vec<usize> = Vec::new();
    while let Some(c) = stack.pop() {
        let pr = pivot_row[c];
        if pr == u32::MAX {
            s.unreachable = true;
            continue;
        }
        let pr = pr as usize;
        s.reached += 1;
        s.reach_cells_v += (ncols - c - 1) as u64;
        s.reach_cells_g += (end[pr] - c - 1) as u64;
        mark(&mut tv, pr, c + 1, ncols);
        mark(&mut tg, pr, c + 1, end[pr]);
        if !dep[pr] {
            dep[pr] = true;
            dstack.push(pr);
        }
        for j in (c + 1)..ncols {
            if rows[pr][j] != 0 && !is_std[j] && !seen[j] {
                seen[j] = true;
                stack.push(j);
            }
        }
    }
    while let Some(r) = dstack.pop() {
        for &u in &by[r] {
            if !dep[u as usize] {
                dep[u as usize] = true;
                dstack.push(u as usize);
            }
        }
    }
    s.by_lead_degree = vec![[0u64; 5]; d as usize + 1];
    for r in 0..nrows {
        let l = lead[r];
        let tail = (ncols - l) as u64;
        let maxnz = (l..ncols)
            .rev()
            .find(|&j| ever[r][j])
            .map_or(l + 1, |j| j + 1);
        let ev = ever[r][l..].iter().filter(|&&x| x).count() as u64;
        s.tail_cells += tail;
        s.build_extent_cells += (build_end[r] - l) as u64;
        s.final_extent_cells += (alloc[r] - l) as u64;
        for k in 0..3 {
            s.reserve_cells[k] += (cap[r][k] - l) as u64;
            s.reserve_realloc_rows[k] += u64::from(reallocated[r][k]);
        }
        s.ever_extent_cells += (maxnz - l) as u64;
        s.ever_nonzero_cells += ev;
        s.final_nonzero_cells += rows[r][l..].iter().filter(|&&x| x != 0).count() as u64;
        s.touched_v += tv[r][l..].iter().filter(|&&x| x).count() as u64;
        s.touched_g += tg[r][l..].iter().filter(|&&x| x).count() as u64;
        if dep[r] {
            s.dependency_rows += 1;
        } else {
            s.outside_dependency_final_cells += (alloc[r] - l) as u64;
        }
        let e = &mut s.by_lead_degree[wdeg(&cols[l]) as usize];
        e[0] += 1;
        e[1] += tail;
        e[2] += (build_end[r] - l) as u64;
        e[3] += (alloc[r] - l) as u64;
        e[4] += ev;
    }
    Some(s)
}
