//! **E40 (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.50–§9.51): the native
//! cost of the chain solve's Macaulay matrix.**
//!
//! Stage 0 (counts only): [`stage0`] rebuilds one call's weighted-degree-`d`
//! Macaulay matrix exactly as `klein_v4::build_echelon` builds it for the
//! chain solve (row selection on, E31 A's echelon) and counts where its
//! native work goes: monomial enumeration, row selection, entry writes and
//! column lookups, the zero prefix left of each row's lead, lead scans,
//! zero, duplicate and untouched rows and columns, and which rows the chain
//! solve's normal forms depend on.  It is a diagnostic: nothing it does is
//! charged to any solve.
//!
//! The variant V (`chain-profile`, §9.50): [`build_profile`] builds the same
//! rows as `build_echelon` but stores each row from its lead column on (its
//! tail and an offset), and [`echelon_profile`] is E33's counted echelon with
//! A's rule (`PivotRule::Lowest`) on those tails: the same buckets, pivots,
//! multiplications, lead scans after updates and pivot digest.  Cells left of
//! a row's lead are never stored.  Every use of an offset is charged as one
//! native operation (`QuarticSolve::profile_offset_ops`) and the stored cells
//! are emitted (`QuarticSolve::macaulay_stored_entries`).
//!
//! Additive.  In `klein_v4` the dense path is unchanged; `chain_quotient`
//! reads a row through its offset (zero for dense rows).
//!
//! E43 (§9.58): [`BucketOrder`] selects the pivot of each column's bucket in
//! the storage echelons: `Lowest` (A's rule; V and W, unchanged) or `Fifo`
//! (E36's F, the bucket's first arrival: FV `chain-fifo-profile` and FW
//! `chain-fifo-tails`).  Nothing else changes; the existing entry points keep
//! `Lowest`.

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use rayon::prelude::*;
use serde::Serialize;

use super::gaudry_cubic::am;
use super::gaudry_quartic::{
    axpy, echelon_by_lead_counted, Echelon, PivotCheck, PivotRule, PivotWork, QuarticSolve,
};
use super::klein_v4::{triangularise, wdeg, weighted_monomials_desc, Built, EQUATION_WDEG};
use super::residual_walk::inv_mod;

/// The unknown whose `P₂`-chains the chain solve follows (`klein_v4`'s
/// `EIG_VAR`).
const EIG_VAR: usize = 3;

/// One call's Macaulay matrix, counted (Stage 0, §9.50).  Entry counts are
/// in matrix cells; "tail" is a row from its lead column to the last column.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Stage0 {
    /// Columns: monomials of weighted degree `≤ d`.
    pub cols: u64,
    /// Monomials enumerated per call: the column list plus every equation's
    /// shift list (each list also sorted).
    pub enumerated_monomials: u64,
    /// Shift candidates considered (before row selection).
    pub shift_candidates: u64,
    /// Divisibility tests of the row selection (`any` short-circuit).
    pub selection_tests: u64,
    pub rows_skipped: u64,
    pub rows: u64,
    /// `rows · cols`: the cells the dense build allocates and zeroes.
    pub dense_entries: u64,
    /// Coefficient writes (one column-index lookup each).
    pub term_writes: u64,
    /// `Σ_r lead_r`: cells left of each row's lead (zero at build and never
    /// changed by a forward echelon).
    pub prefix_entries: u64,
    /// `Σ_r (cols − lead_r)`: each row from its lead on.
    pub tail_entries: u64,
    /// Per weighted degree of the lead monomial: `[rows, prefix, tail]`.
    pub by_lead_degree: Vec<[u64; 3]>,
    /// Rows whose built lead is not the column of `LM(fᵢ)·shift` (expected 0).
    pub lead_not_lm_shift: u64,
    /// The dense echelon's first lead scan, `Σ_r (lead_r + 1)`.
    pub initial_lead_scan: u64,
    /// All lead-scan reads of E31 A's echelon (E33's `lead_scan_reads`).
    pub lead_scan: u64,
    pub echelon_muls: u64,
    pub pivots: u64,
    pub zero_rows: u64,
    /// Tails of the rows that reduced to zero.
    pub zero_row_tail_entries: u64,
    /// Columns no row writes, and those of them below degree `d`.
    pub untouched_columns: u64,
    pub untouched_columns_below_d: u64,
    /// Rows equal to an earlier row.
    pub duplicate_rows: u64,
    pub dim: u64,
    pub max_standard_degree: u64,
    /// `P₂`-chains and their border targets.
    pub chains: u64,
    pub border_targets_at_d: u64,
    pub border_max_degree: u64,
    pub unreachable: bool,
    /// Pivot columns the normal forms reach from the border targets.
    pub reached: u64,
    /// Rows whose content reaches a reached pivot row (the reached pivot
    /// rows and, transitively, every pivot row that updated one).
    pub dependency_rows: u64,
    /// Tails of the rows outside that closure.
    pub outside_dependency_tail_entries: u64,
    /// FNV-1a digests of the triangularised leads and of the equations'
    /// monomial supports (equal digests across calls = the same row
    /// templates).
    pub leads_digest: u64,
    pub support_digest: u64,
    /// The recording echelon reproduced E31 A's echelon (pivots and rows).
    pub echelon_reproduced: bool,
}

fn fnv(h: u64, x: u64) -> u64 {
    let mut h = h;
    for b in x.to_le_bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

fn mono_key(m: &[u8; 4]) -> u64 {
    u64::from(u32::from_le_bytes(*m))
}

/// E31 A's forward echelon (lowest-indexed pivot), sequential, recording
/// which pivot row updated which row.
fn lowest_with_updates(
    mut rows: Vec<Vec<u16>>,
    p: u32,
) -> (Vec<Vec<u16>>, Vec<u32>, Vec<Vec<u32>>) {
    let ncols = rows.first().map_or(0, |r| r.len());
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    for (r, row) in rows.iter().enumerate() {
        if let Some(c) = row.iter().position(|&x| x != 0) {
            buckets[c].push(r as u32);
        }
    }
    let mut by: Vec<Vec<u32>> = vec![Vec::new(); rows.len()];
    let mut pivot_row = vec![u32::MAX; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        b.sort_unstable();
        let pr = b[0] as usize;
        let mut piv = std::mem::take(&mut rows[pr]);
        let inv = inv_mod(u64::from(piv[c]), u64::from(p)) as u32;
        for x in piv.iter_mut().skip(c) {
            if *x != 0 {
                *x = ((u32::from(*x) * inv) % p) as u16;
            }
        }
        for &r in &b[1..] {
            let row = &mut rows[r as usize];
            let nf = p - u32::from(row[c]);
            for j in c..ncols {
                if piv[j] != 0 {
                    row[j] = ((u32::from(row[j]) + nf * u32::from(piv[j])) % p) as u16;
                }
            }
            by[r as usize].push(pr as u32);
            if let Some(k) = row[c + 1..].iter().position(|&x| x != 0) {
                buckets[c + 1 + k].push(r);
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    (rows, pivot_row, by)
}

/// Stage 0 of E40 on one call's system (`v4_system`'s components) at
/// weighted degree `d`.  `None` when an equation lies above `d` (the
/// chain solve refuses those calls too).
pub fn stage0(comps: &[HashMap<[u8; 4], u64>], d: u16, p: u64) -> Option<Stage0> {
    let mut s = Stage0::default();
    let cols = weighted_monomials_desc(d);
    let ncols = cols.len();
    s.cols = ncols as u64;
    s.enumerated_monomials = ncols as u64;
    let col_index: HashMap<[u8; 4], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let mut tri = 0u64;
    let (comps, leads) = triangularise(comps, p, &mut tri);
    let mut ld = 0xCBF2_9CE4_8422_2325u64;
    for l in &leads {
        ld = fnv(ld, l.map_or(u64::MAX, |m| mono_key(&m)));
    }
    s.leads_digest = ld;
    let mut sd = 0xCBF2_9CE4_8422_2325u64;
    for comp in &comps {
        let mut sup: Vec<u64> = comp.keys().map(mono_key).collect();
        sup.sort_unstable();
        sd = fnv(sd, sup.len() as u64);
        for k in sup {
            sd = fnv(sd, k);
        }
    }
    s.support_digest = sd;
    let divides = |lm: &[u8; 4], m: &[u8; 4]| (0..4).all(|k| lm[k] <= m[k]);
    s.by_lead_degree = vec![[0u64; 3]; d as usize + 1];
    let mut rows: Vec<Vec<u16>> = Vec::new();
    let mut offs: Vec<usize> = Vec::new();
    let mut touched = vec![false; ncols];
    for (i, comp) in comps.iter().enumerate() {
        let Some(dg) = comp
            .iter()
            .filter(|(_, &v)| v != 0)
            .map(|(m, _)| wdeg(m))
            .max()
        else {
            continue;
        };
        if dg > d {
            return None;
        }
        let shifts = weighted_monomials_desc(d - dg);
        s.enumerated_monomials += shifts.len() as u64;
        s.shift_candidates += shifts.len() as u64;
        for sh in shifts {
            let mut skip = false;
            for lm in leads.iter().take(i).flatten() {
                s.selection_tests += 1;
                if divides(lm, &sh) {
                    skip = true;
                    break;
                }
            }
            if skip {
                s.rows_skipped += 1;
                continue;
            }
            let mut row = vec![0u16; ncols];
            let mut lead = ncols;
            for (&e, &c) in comp {
                let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                let ci = col_index[&m];
                row[ci] = am(u64::from(row[ci]), c, p) as u16;
                s.term_writes += 1;
                touched[ci] = true;
                lead = lead.min(ci);
            }
            let lm_shift = leads[i].map(|l| {
                let m: [u8; 4] = core::array::from_fn(|k| l[k] + sh[k]);
                col_index[&m]
            });
            if lm_shift != Some(lead) || row[lead] == 0 {
                s.lead_not_lm_shift += 1;
            }
            rows.push(row);
            offs.push(lead);
        }
    }
    s.rows = rows.len() as u64;
    s.dense_entries = s.rows * s.cols;
    for &o in &offs {
        s.prefix_entries += o as u64;
        s.tail_entries += (ncols - o) as u64;
        s.initial_lead_scan += o as u64 + 1;
        let e = &mut s.by_lead_degree[wdeg(&cols[o]) as usize];
        e[0] += 1;
        e[1] += o as u64;
        e[2] += (ncols - o) as u64;
    }
    for (c, &t) in touched.iter().enumerate() {
        if !t {
            s.untouched_columns += 1;
            if wdeg(&cols[c]) < d {
                s.untouched_columns_below_d += 1;
            }
        }
    }
    let mut seen_rows: HashSet<&[u16]> = HashSet::new();
    for r in &rows {
        if !seen_rows.insert(r.as_slice()) {
            s.duplicate_rows += 1;
        }
    }
    // E31 A's echelon, counted (E33), and the recording copy.
    let mut w = PivotWork::default();
    let mut muls = 0u64;
    let ech = echelon_by_lead_counted(
        rows.clone(),
        p as u32,
        &mut muls,
        PivotRule::Lowest,
        None,
        PivotCheck::default(),
        &mut w,
    );
    s.lead_scan = w.lead_scan;
    s.echelon_muls = muls;
    s.zero_rows = ech.zero_rows as u64;
    for &z in &ech.zero_ids {
        s.zero_row_tail_entries += (ncols - offs[z as usize]) as u64;
    }
    let (rec_rows, pivot_row, by) = lowest_with_updates(rows, p as u32);
    s.echelon_reproduced = pivot_row == ech.pivot_row && rec_rows == ech.rows;
    s.pivots = pivot_row.iter().filter(|&&r| r != u32::MAX).count() as u64;
    let is_std: Vec<bool> = (0..ncols)
        .map(|c| pivot_row[c] == u32::MAX && wdeg(&cols[c]) < d)
        .collect();
    let standard: Vec<usize> = (0..ncols).filter(|&c| is_std[c]).collect();
    s.dim = standard.len() as u64;
    s.max_standard_degree = standard
        .iter()
        .map(|&c| u64::from(wdeg(&cols[c])))
        .max()
        .unwrap_or(0);
    // The P₂-chains (klein_v4::p2_chains) and their border targets.
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
    let mut targets: Vec<Option<usize>> = Vec::new();
    for &h in &standard {
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
        targets.push(t);
    }
    s.chains = targets.len() as u64;
    for t in targets.iter().flatten() {
        let g = u64::from(wdeg(&cols[*t]));
        s.border_max_degree = s.border_max_degree.max(g);
        if g == u64::from(d) {
            s.border_targets_at_d += 1;
        }
    }
    // Reached pivots (chain_quotient's reachability) and the dependency
    // closure of their rows over "updated by".
    let mut seen = vec![false; ncols];
    let mut stack: Vec<usize> = Vec::new();
    for t in &targets {
        match t {
            Some(t) if !seen[*t] => {
                seen[*t] = true;
                stack.push(*t);
            }
            Some(_) => {}
            None => s.unreachable = true,
        }
    }
    let mut dep = vec![false; offs.len()];
    let mut dstack: Vec<usize> = Vec::new();
    while let Some(c) = stack.pop() {
        let pr = pivot_row[c];
        if pr == u32::MAX {
            s.unreachable = true;
            continue;
        }
        s.reached += 1;
        if !dep[pr as usize] {
            dep[pr as usize] = true;
            dstack.push(pr as usize);
        }
        let row = &rec_rows[pr as usize];
        for j in (c + 1)..ncols {
            if row[j] != 0 && !is_std[j] && !seen[j] {
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
    for (r, &dp) in dep.iter().enumerate() {
        if dp {
            s.dependency_rows += 1;
        } else {
            s.outside_dependency_tail_entries += (ncols - offs[r]) as u64;
        }
    }
    Some(s)
}

// ── E40 V: the profile build and echelon (§9.50) ─────────────────────────

/// E43 (§9.58): which row of a column's bucket becomes the pivot in the
/// storage echelons ([`echelon_profile`], `macaulay_tails::echelon_tails`).
/// Rows are filed in index order at build and re-filed in update order, the
/// arrival order of E36's echelon; `Fifo` takes the first arrival (no sort,
/// no selection work), `Lowest` sorts the bucket (A's rule, as before).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum BucketOrder {
    /// A's rule (E33's `Lowest`): V and W.
    #[default]
    Lowest,
    /// E36's F (`ArrivalRule::Fifo`): FV and FW.
    Fifo,
}

/// Positive controls for V (all off by default; nothing here is charged).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProfileCheck {
    /// P1 (broken storage): every row stored from one column right of its
    /// lead, its lead cell dropped.
    pub drop_lead: bool,
    /// P2 (broken count): the stored-cell counter short by one per row.
    pub undercount: bool,
}

/// E33's counted echelon with A's rule (`Lowest`; E43: or F's, `order`) on
/// rows stored from `offs[r]` on (`rows[r][k]` is column `offs[r] + k`).  The same buckets,
/// pivots, normalisations, branch rule, updates, multiplications, lead scans
/// after updates and digest as `echelon_by_lead_counted(…, Lowest, …)` on the
/// dense rows; the first lead scan starts at the offset.  `ops` counts each
/// use of an offset: one per row placed in its first bucket, one per pivot
/// normalisation, two per row update.
#[allow(clippy::too_many_arguments)]
pub(crate) fn echelon_profile(
    mut rows: Vec<Vec<u16>>,
    offs: &[u32],
    ncols: usize,
    p: u32,
    order: BucketOrder,
    muls: &mut u64,
    work: &mut PivotWork,
    ops: &mut u64,
) -> Echelon {
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut zero_rows = 0usize;
    let mut zero_ids: Vec<u32> = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        match row.iter().position(|&x| x != 0) {
            Some(k) => {
                work.lead_scan += k as u64 + 1;
                *ops += 1;
                buckets[offs[r] as usize + k].push(r as u32);
            }
            None => {
                work.lead_scan += row.len() as u64;
                zero_rows += 1;
                zero_ids.push(r as u32);
            }
        }
    }
    let mut digest = 0xCBF2_9CE4_8422_2325u64;
    let mut pivot_row = vec![u32::MAX; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        if order == BucketOrder::Lowest {
            b.sort_unstable();
        }
        if b.len() > 1 {
            work.contested += 1;
        }
        let pr = b[0] as usize;
        digest = fnv(fnv(digest, c as u64), pr as u64);
        let mut piv = std::mem::take(&mut rows[pr]);
        let op = c - offs[pr] as usize;
        *ops += 1;
        let inv = inv_mod(u64::from(piv[op]), u64::from(p)) as u32;
        // `nz` holds columns relative to `c`.
        let mut nz: Vec<u32> = Vec::new();
        for (k, x) in piv[op..].iter_mut().enumerate() {
            if *x != 0 {
                *x = ((u32::from(*x) * inv) % p) as u16;
                nz.push(k as u32);
            }
        }
        *muls += nz.len() as u64;
        let dense = nz.len() * 8 > ncols - c;
        let mut others: Vec<(u32, Vec<u16>, Option<usize>, u64)> = b[1..]
            .iter()
            .map(|&r| (r, std::mem::take(&mut rows[r as usize]), None, 0))
            .collect();
        *muls += (others.len() * nz.len()) as u64;
        *ops += 2 * others.len() as u64;
        let ps = &piv[op..];
        let nzr = &nz;
        others.par_iter_mut().for_each(|(r, row, lead, scan)| {
            let rs = &mut row[c - offs[*r as usize] as usize..];
            let f = u32::from(rs[0]);
            if dense {
                axpy(rs, ps, f, p);
            } else {
                let nf = p - f;
                for &k in nzr {
                    let k = k as usize;
                    rs[k] = ((u32::from(rs[k]) + nf * u32::from(ps[k])) % p) as u16;
                }
            }
            *lead = rs[1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
            *scan = match *lead {
                Some(l) => (l - c) as u64,
                None => (ncols - c - 1) as u64,
            };
        });
        for (r, row, lead, scan) in others {
            work.lead_scan += scan;
            rows[r as usize] = row;
            match lead {
                Some(l) => buckets[l].push(r),
                None => {
                    zero_rows += 1;
                    zero_ids.push(r);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    work.digest = digest;
    zero_ids.sort_unstable();
    Echelon {
        rows,
        pivot_row,
        zero_rows,
        zero_ids,
    }
}

/// V's build: `klein_v4::build_echelon(…, row_selection = true,
/// Pivot::Counted(Lowest, _))` with each row stored from its lead (the
/// column of `LM(fᵢ)·shift`) on, and [`echelon_profile`].  The same `st`
/// fields as A counted, plus `macaulay_stored_entries` and the build's and
/// echelon's `profile_offset_ops` (per built row: the lead's column lookup
/// and the tail length; per coefficient write: its column relative to the
/// offset; then the echelon's).
pub(crate) fn build_profile(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    st: &mut QuarticSolve,
    check: ProfileCheck,
) -> Result<Built, String> {
    build_profile_ordered(comps, d, p, st, check, BucketOrder::Lowest)
}

/// [`build_profile`] with the bucket order of the echelon (E43: `Fifo` is
/// FV, `chain-fifo-profile`).
pub(crate) fn build_profile_ordered(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    st: &mut QuarticSolve,
    check: ProfileCheck,
    order: BucketOrder,
) -> Result<Built, String> {
    assert!(p < 1 << 15, "rows are u16 and products must fit u32");
    let t0 = Instant::now();
    st.equation_degree = EQUATION_WDEG as u8;
    st.macaulay_degree = d as u8;
    let cols = weighted_monomials_desc(d);
    let ncols = cols.len();
    let col_index: HashMap<[u8; 4], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let mut tri = 0u64;
    let (comps, leads) = triangularise(comps, p, &mut tri);
    st.triangularise_muls += tri;
    let divides = |lm: &[u8; 4], m: &[u8; 4]| (0..4).all(|k| lm[k] <= m[k]);
    let mut rows: Vec<Vec<u16>> = Vec::new();
    let mut offs: Vec<u32> = Vec::new();
    let mut ops = 0u64;
    for (i, comp) in comps.iter().enumerate() {
        let Some(dg) = comp
            .iter()
            .filter(|(_, &v)| v != 0)
            .map(|(m, _)| wdeg(m))
            .max()
        else {
            continue;
        };
        if dg > d {
            return Err("an equation above the Macaulay degree".into());
        }
        let lm = leads[i].expect("a nonzero equation has a lead");
        for sh in weighted_monomials_desc(d - dg) {
            if leads
                .iter()
                .take(i)
                .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
            {
                continue;
            }
            let lead: [u8; 4] = core::array::from_fn(|k| lm[k] + sh[k]);
            let start = col_index[&lead] + usize::from(check.drop_lead);
            ops += 2;
            let mut row = vec![0u16; ncols - start];
            for (&e, &c) in comp {
                let m: [u8; 4] = core::array::from_fn(|k| e[k] + sh[k]);
                let ci = col_index[&m];
                ops += 1;
                if ci < start {
                    // P1 only: the lead cell is not stored.
                    continue;
                }
                let k = ci - start;
                row[k] = am(u64::from(row[k]), c, p) as u16;
            }
            rows.push(row);
            offs.push(start as u32);
        }
    }
    let stored: u64 = rows.iter().map(|r| r.len() as u64).sum();
    st.macaulay_stored_entries += if check.undercount {
        stored - rows.len() as u64
    } else {
        stored
    };
    st.rows = rows.len();
    st.cols = ncols;
    st.build_ms += t0.elapsed().as_secs_f64() * 1e3;
    let t1 = Instant::now();
    let mut ech_muls = 0u64;
    let mut w = PivotWork::default();
    let ech = echelon_profile(
        rows,
        &offs,
        ncols,
        p as u32,
        order,
        &mut ech_muls,
        &mut w,
        &mut ops,
    );
    st.profile_offset_ops += ops;
    st.pivot_init_ops += w.init;
    st.pivot_compare_ops += w.compare;
    st.pivot_update_ops += w.update;
    st.lead_scan_reads += w.lead_scan;
    st.pivot_count_mismatches += w.count_mismatches;
    st.pivot_digest = w.digest;
    st.echelon_muls += ech_muls;
    st.echelon_ms += t1.elapsed().as_secs_f64() * 1e3;
    st.zero_rows = ech.zero_rows;
    st.pivots = ech.pivot_row.iter().filter(|&&r| r != u32::MAX).count();
    let is_std: Vec<bool> = (0..ncols)
        .map(|c| ech.pivot_row[c] == u32::MAX && wdeg(&cols[c]) < d)
        .collect();
    let standard: Vec<usize> = (0..ncols).filter(|&c| is_std[c]).collect();
    st.dim = standard.len();
    let mut std_index = vec![u32::MAX; ncols];
    for (i, &c) in standard.iter().enumerate() {
        std_index[c] = i as u32;
    }
    Ok(Built {
        cols,
        col_index,
        comps,
        ech,
        is_std,
        standard,
        std_index,
        offs,
    })
}

/// The stored-cell check of §9.50 on one call (nothing charged): V's
/// counter, the lengths of the rows it holds after the echelon, and
/// `Σ_r offset_r` (A's lead scans exceed V's by exactly this).
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct ProfileMeasure {
    pub counter: u64,
    pub measured: u64,
    pub offsets: u64,
}

pub fn profile_measure(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    check: ProfileCheck,
) -> Option<ProfileMeasure> {
    profile_measure_ordered(comps, d, p, check, BucketOrder::Lowest)
}

/// [`profile_measure`] with the echelon's bucket order (E43).
pub fn profile_measure_ordered(
    comps: &[HashMap<[u8; 4], u64>],
    d: u16,
    p: u64,
    check: ProfileCheck,
    order: BucketOrder,
) -> Option<ProfileMeasure> {
    let mut st = QuarticSolve::default();
    let b = build_profile_ordered(comps, d, p, &mut st, check, order).ok()?;
    Some(ProfileMeasure {
        counter: st.macaulay_stored_entries,
        measured: b.ech.rows.iter().map(|r| r.len() as u64).sum(),
        offsets: b.offs.iter().map(|&o| u64::from(o)).sum(),
    })
}
