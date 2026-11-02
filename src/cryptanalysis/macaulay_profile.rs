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
//! Additive.  `klein_v4` is unchanged apart from `triangularise` becoming
//! visible to the crate.

use std::collections::{HashMap, HashSet};

use serde::Serialize;

use super::gaudry_cubic::am;
use super::gaudry_quartic::{echelon_by_lead_counted, PivotCheck, PivotRule, PivotWork};
use super::klein_v4::{triangularise, wdeg, weighted_monomials_desc};
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
