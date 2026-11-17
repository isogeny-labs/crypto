//! Fixed, input-only pairing selection with all three native Split4 controls.
use crate::{quartic_cells, quartic_oracle, sampler, solver::CALL_BUDGET_MS};
use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Encoding},
    f4_fp::{self, F4Options, Ordering, TracePool},
};
use std::{collections::BTreeSet, time::Duration};

mod model;
mod panel;
pub use model::Report;
use model::{CellResult, Route, Selection};

const PAIRINGS: [[usize; 4]; 3] = [[0, 1, 2, 3], [0, 2, 1, 3], [0, 3, 1, 2]];

fn select(parts: &[Vec<u64>], cell: usize) -> Result<Selection, String> {
    if parts.len() != 4 || parts.iter().any(|p| p.is_empty() || p.len() > 2) {
        return Err("pairing score requires four bounded nonempty domains".into());
    }
    let mut coordinates = BTreeSet::new();
    if parts.iter().flatten().any(|x| !coordinates.insert(*x)) {
        return Err("pairing score requires pairwise-disjoint unique domains".into());
    }
    let candidate_product_scores = PAIRINGS
        .map(|p| parts[p[0]].len() * parts[p[1]].len() + parts[p[2]].len() * parts[p[3]].len());
    let selected_pairing = (0..3)
        .min_by_key(|&i| (candidate_product_scores[i], i))
        .unwrap();
    Ok(Selection {
        cell,
        candidate_product_scores,
        selected_pairing,
        domain_coordinate_visits: coordinates.len(),
        score_candidates: 3,
        score_multiplications: 6,
        score_additions: 3,
    })
}

fn project(witness: &[u64], pairing: [usize; 4], domains: &[Vec<u64>]) -> Result<Vec<u64>, String> {
    if witness.len() != 5
        || domains.len() != 4
        || pairing.into_iter().collect::<BTreeSet<_>>() != BTreeSet::from([0, 1, 2, 3])
    {
        return Err("Split4 projection requires five coordinates and a slot permutation".into());
    }
    let mut original = vec![0; 4];
    for (i, slot) in pairing.into_iter().enumerate() {
        if !domains[slot].contains(&witness[i]) {
            return Err("native witness violates reordered domains".into());
        }
        original[slot] = witness[i];
    }
    original.sort_unstable();
    Ok(original)
}

fn route(f: &sampler::Fixture, seed: u64, pairing: usize, shared: bool) -> Result<Route, String> {
    route_run(f, seed, Some(pairing), shared, None)
}

fn route_selected(
    f: &sampler::Fixture,
    seed: u64,
    selections: &[Selection],
) -> Result<Route, String> {
    route_run(f, seed, None, false, Some(selections))
}

fn route_run(
    f: &sampler::Fixture,
    seed: u64,
    fixed: Option<usize>,
    shared: bool,
    selections: Option<&[Selection]>,
) -> Result<Route, String> {
    let expected_union = quartic_oracle::roots(&f.base, &quartic_cells::universe(f))?;
    let mut pool = TracePool::new(&format!("pairings:{seed}:{fixed:?}"), 8, 100000)?;
    let mut cells = Vec::new();
    let mut union = BTreeSet::new();
    let mut correct = true;
    for (index, cell) in quartic_cells::plan(f)?.into_iter().enumerate() {
        let pairing = match fixed {
            Some(p) => p,
            None => {
                selections
                    .and_then(|s| s.get(index))
                    .ok_or("missing selected-cell schedule")?
                    .selected_pairing
            }
        };
        let permutation = *PAIRINGS.get(pairing).ok_or("invalid pairing index")?;
        let expected = quartic_oracle::roots(&f.base, &quartic_cells::product(&cell.parts)?)?;
        let parts: Vec<_> = permutation.iter().map(|&i| cell.parts[i].clone()).collect();
        let mut observed = BTreeSet::new();
        let mut witnesses = Vec::new();
        let mut projection_error = None;
        let result = aps::build_system(&f.curve, &parts, Encoding::Split4).and_then(|system| {
            let mut domains: Vec<_> = parts.iter().cloned().map(Some).collect();
            domains.push(None);
            let options = F4Options::new(Ordering::Grevlex, 24)
                .with_budget(Duration::from_millis(CALL_BUDGET_MS));
            let mut callback = |point: &[u64]| {
                witnesses.push(point.to_vec());
                if point.len() != 5 || parts.iter().zip(point).any(|(p, x)| !p.contains(x)) {
                    projection_error = Some("native witness violates reordered domains".into());
                } else {
                    match project(point, permutation, &cell.parts) {
                        Ok(tuple) => {
                            observed.insert(tuple);
                        }
                        Err(error) => projection_error = Some(error),
                    }
                }
                false
            };
            if shared {
                f4_fp::solve_visit_reusing(
                    system.equations(),
                    system.variables(),
                    system.modulus(),
                    &options,
                    &domains,
                    &mut pool,
                    &mut callback,
                )
            } else {
                f4_fp::solve_visit(
                    system.equations(),
                    system.variables(),
                    system.modulus(),
                    &options,
                    &domains,
                    &mut callback,
                )
            }
        });
        let (complete, timed_out, error) = match result {
            Ok(report) => (report.complete, report.timed_out, projection_error),
            Err(error) => (false, false, Some(error)),
        };
        correct &= complete && !timed_out && error.is_none() && observed == expected;
        union.extend(observed.iter().cloned());
        cells.push(CellResult {
            cell: index,
            pairing,
            permutation,
            original_parts: cell.parts,
            complete,
            timed_out,
            error,
            expected: expected.into_iter().collect(),
            observed: observed.into_iter().collect(),
            raw_witnesses: witnesses,
        });
    }
    correct &= union == expected_union;
    let observed_roots: Vec<_> = union.into_iter().collect();
    let (recovery, recovery_error) =
        match aps::recover(&f.curve, f.g, f.q, &f.base, &observed_roots) {
            Ok(r) => (r, None),
            Err(error) => {
                correct = false;
                (None, Some(error))
            }
        };
    let informative: usize = expected_union
        .iter()
        .map(|xs| quartic_oracle::incidents(&f.base, xs).map(|x| x.0))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();
    correct &= recovery.is_some() == (informative > 0);
    if let Some(r) = &recovery {
        let curve = Curve::new(101, 59, 6)?;
        let (gx, gy) = f.curve.canonical(f.g);
        let (qx, qy) = f.curve.canonical(f.q);
        correct &= r.b != 0
            && f.curve.mul(Some(f.g), r.logarithm) == Some(f.q)
            && curve.mul(Some(Point { x: gx, y: gy }), r.logarithm) == Some(Point { x: qx, y: qy });
    }
    Ok(Route {
        pairing: fixed,
        permutation: fixed.map(|p| PAIRINGS[p]),
        selected_input_rule: fixed.is_none(),
        shared,
        cells,
        observed_roots,
        recovered_logarithm: recovery.map(|r| r.logarithm),
        recovery_error,
        pool_stats: if shared { Some(pool.stats()) } else { None },
        complete_and_equal: correct,
    })
}

pub fn run() -> Result<Report, String> {
    Ok(panel::collect(sampler::even_fixture))
}

#[cfg(test)]
mod tests;
