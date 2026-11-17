//! Exact auxiliary support construction; all construction work remains explicit.
use crate::{
    quartic_cells, quartic_oracle,
    sampler::{self, Fixture},
};
use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Encoding},
    f4_fp::{self, F4Options, Ordering, Poly},
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

type PairTable = BTreeMap<u64, Vec<[u64; 2]>>;

mod model;
pub use model::{CellError, CellReport, Outcome, Report, Sample};

fn pairs(f: &Fixture, left: &[u64], right: &[u64]) -> Result<(PairTable, usize), String> {
    let c = Curve::new(101, 59, 6)?;
    let mut table: PairTable = BTreeMap::new();
    let mut additions = 0;
    for &x in left {
        for &y in right {
            if x == y {
                return Err("pair interface requires disjoint abscissas".into());
            }
            let a = f.base.points.get(&x).ok_or("left point absent")?;
            let b = f.base.points.get(&y).ok_or("right point absent")?;
            let a = Point { x: a.x, y: a.y };
            let b = Point { x: b.x, y: b.y };
            if !c.contains(a) || !c.contains(b) || a.y == 0 || b.y == 0 {
                return Err("invalid pair-interface fixture".into());
            }
            for b in [b, c.neg(b)] {
                additions += 1;
                let sum = c
                    .add(Some(a), Some(b))
                    .ok_or("pair sums to infinity despite distinct x")?;
                table.entry(sum.x).or_default().push([x, y]);
            }
        }
    }
    Ok((table, additions))
}

fn join(left: &PairTable, right: &PairTable) -> (BTreeSet<Vec<u64>>, usize) {
    let mut roots = BTreeSet::new();
    let mut matches = 0;
    for (x, pairs) in left {
        if let Some(other) = right.get(x) {
            for a in pairs {
                for b in other {
                    matches += 1;
                    let mut tuple = vec![a[0], a[1], b[0], b[1]];
                    tuple.sort_unstable();
                    roots.insert(tuple);
                }
            }
        }
    }
    (roots, matches)
}

fn vanish(values: &[u64]) -> Poly {
    let mut coefficients = vec![1];
    for &x in values {
        let mut next = vec![0; coefficients.len() + 1];
        for (i, &c) in coefficients.iter().enumerate() {
            next[i] = (next[i] + 101 - c * x % 101) % 101;
            next[i + 1] = (next[i + 1] + c) % 101;
        }
        coefficients = next;
    }
    let terms: Poly = coefficients
        .iter()
        .enumerate()
        .map(|(degree, &c)| {
            let mut m = vec![0; 5];
            m[4] = degree as u32;
            (m, c)
        })
        .collect();
    f4_fp::normalise(&terms, 101, Ordering::Grevlex)
}

fn solve(f: &Fixture, parts: &[Vec<u64>], support: Option<&[u64]>) -> Outcome {
    let mut out = Outcome {
        solver_called: false,
        complete: false,
        timed_out: false,
        error: None,
        roots: BTreeSet::new(),
        raw_witnesses: Vec::new(),
    };
    if support.is_some_and(<[u64]>::is_empty) {
        out.complete = true;
        return out;
    }
    let result = (|| {
        let system = aps::build_system(&f.curve, parts, Encoding::Split4)?;
        let mut equations = system.equations().to_vec();
        let mut domains: Vec<_> = parts.iter().cloned().map(Some).collect();
        domains.push(support.map(<[u64]>::to_vec));
        if let Some(support) = support {
            equations.push(vanish(support));
        }
        out.solver_called = true;
        let options =
            F4Options::new(Ordering::Grevlex, 24).with_budget(Duration::from_millis(2000));
        f4_fp::solve_visit(&equations, 5, 101, &options, &domains, |point| {
            out.raw_witnesses.push(point.to_vec());
            let mut tuple = point[..4].to_vec();
            tuple.sort_unstable();
            out.roots.insert(tuple);
            false
        })
    })();
    match result {
        Ok(report) => {
            out.complete = report.complete;
            out.timed_out = report.timed_out;
        }
        Err(error) => out.error = Some(error),
    }
    out
}

fn cell_report(
    f: &Fixture,
    cell_index: usize,
    cell: quartic_cells::Cell,
) -> Result<CellReport, String> {
    let tuples = quartic_cells::product(&cell.parts)?;
    let (left, lwork) = pairs(f, &cell.parts[0], &cell.parts[1])?;
    let (right, rwork) = pairs(f, &cell.parts[2], &cell.parts[3])?;
    let intersection: Vec<_> = left
        .keys()
        .filter(|x| right.contains_key(x))
        .copied()
        .collect();
    let expected = quartic_oracle::roots(&f.base, &tuples)?;
    let (joined, matches) = join(&left, &right);
    let full = solve(f, &cell.parts, None);
    let restricted = solve(f, &cell.parts, Some(&intersection));
    let correct = joined == expected
        && [&full, &restricted]
            .iter()
            .all(|r| r.complete && !r.timed_out && r.error.is_none() && r.roots == expected);
    Ok(CellReport {
        cell_index,
        parts: cell.parts,
        valuation_safe: cell.valuation_safe,
        left_support: left.keys().copied().collect(),
        right_support: right.keys().copied().collect(),
        auxiliary_vanishing: vanish(&intersection),
        intersection,
        construction_pair_additions: lwork + rwork,
        direct_join_pair_matches: matches,
        direct_join_roots: joined,
        expected_roots: expected,
        full_field: full,
        constrained: restricted,
        complete_and_equal: correct,
    })
}

fn run_cells(f: &Fixture, seed: u64, cells: Vec<quartic_cells::Cell>) -> Sample {
    let planned_cells = cells.len();
    let mut rows = Vec::new();
    let mut cell_errors = Vec::new();
    let mut union = BTreeSet::new();
    let mut completed_cells = 0;
    let mut attempted_cells = 0;
    for (index, cell) in cells.into_iter().enumerate() {
        attempted_cells += 1;
        let parts = cell.parts.clone();
        match cell_report(f, index, cell) {
            Ok(row) => {
                completed_cells += usize::from(row.complete_and_equal);
                union.extend(row.constrained.roots.iter().cloned());
                rows.push(row);
            }
            Err(error) => cell_errors.push(CellError {
                cell_index: index,
                parts,
                error,
            }),
        }
    }
    let (expected, union_error) = match quartic_oracle::roots(&f.base, &quartic_cells::universe(f))
    {
        Ok(roots) => (Some(roots), None),
        Err(error) => (None, Some(error)),
    };
    let correct = planned_cells == 19
        && attempted_cells == 19
        && completed_cells == 19
        && cell_errors.is_empty()
        && expected.as_ref() == Some(&union);
    Sample {
        seed,
        cells: rows,
        cell_errors,
        planned_cells,
        attempted_cells,
        completed_cells,
        failed_or_incomplete_cells: attempted_cells - completed_cells,
        expected_union: expected,
        union_error,
        constrained_union: union,
        complete_and_equal: correct,
    }
}

fn run_sample(seed: u64) -> Result<Sample, String> {
    let f = sampler::even_fixture(seed)?;
    let cells = quartic_cells::plan(&f)?;
    Ok(run_cells(&f, seed, cells))
}

pub fn run() -> Result<Report, String> {
    let mut samples = Vec::new();
    let mut seed_errors = Vec::new();
    let mut attempted_seeds = 0;
    for seed in sampler::SEEDS {
        attempted_seeds += 1;
        match run_sample(seed) {
            Ok(sample) => samples.push(sample),
            Err(error) => seed_errors.push((seed, error)),
        }
    }
    Ok(Report {
        complete_and_equal: seed_errors.is_empty()
            && samples.len() == sampler::SEEDS.len()
            && samples.iter().all(|s| s.complete_and_equal),
        planned_seeds: sampler::SEEDS.len(),
        attempted_seeds,
        seed_errors,
        samples,
        construction_charged_as_diagnostics_only: true,
        performance_trial: false,
    })
}

#[cfg(test)]
#[path = "aux_domains_tests.rs"]
mod tests;
