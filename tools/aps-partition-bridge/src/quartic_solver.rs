use crate::{
    quartic_cells::{self, Cell},
    quartic_oracle,
    sampler::Fixture,
    solver::CALL_BUDGET_MS,
};
use aps_partition_lab::curve::{Curve, Point};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Encoding},
    f4_fp::{self, F4Options, Ordering, TracePool, TraceStats},
};
use serde::Serialize;
use std::{collections::BTreeSet, time::Duration};

#[derive(Serialize)]
pub struct CellResult {
    pub cell: Cell,
    pub complete: bool,
    pub timed_out: bool,
    pub error: Option<String>,
    pub expected: Vec<Vec<u64>>,
    pub observed: Vec<Vec<u64>>,
}

#[derive(Serialize)]
pub struct Route {
    pub shared: bool,
    pub cells: Vec<CellResult>,
    pub expected_roots: Vec<Vec<u64>>,
    pub observed_roots: Vec<Vec<u64>>,
    pub safe_informative_incidents: usize,
    pub uncertain_informative_incidents: usize,
    pub zero_b_incidents: usize,
    pub recovered_logarithm: Option<u64>,
    pub recovery_error: Option<String>,
    pub pool_stats: Option<TraceStats>,
    pub complete_and_equal: bool,
}

fn visit(
    f: &Fixture,
    cell: &Cell,
    pool: Option<&mut TracePool>,
) -> Result<(bool, bool, BTreeSet<Vec<u64>>), String> {
    let system = aps::build_system(&f.curve, &cell.parts, Encoding::Expanded4)?;
    let options =
        F4Options::new(Ordering::Grevlex, 24).with_budget(Duration::from_millis(CALL_BUDGET_MS));
    let domains: Vec<_> = cell.parts.iter().cloned().map(Some).collect();
    let mut roots = BTreeSet::new();
    let mut callback = |point: &[u64]| {
        let mut tuple = point.to_vec();
        tuple.sort_unstable();
        roots.insert(tuple);
        false
    };
    let report = match pool {
        Some(pool) => f4_fp::solve_visit_reusing(
            system.equations(),
            system.variables(),
            system.modulus(),
            &options,
            &domains,
            pool,
            &mut callback,
        )?,
        None => f4_fp::solve_visit(
            system.equations(),
            system.variables(),
            system.modulus(),
            &options,
            &domains,
            &mut callback,
        )?,
    };
    Ok((report.complete, report.timed_out, roots))
}

pub fn run(f: &Fixture, seed: u64, shared: bool) -> Result<Route, String> {
    let cells = quartic_cells::plan(f)?;
    let expected = quartic_oracle::roots(&f.base, &quartic_cells::universe(f))?;
    let mut pool = TracePool::new(&format!("quartic-label-cells:{seed}"), 8, 100000)?;
    let mut rows = Vec::new();
    let mut union = BTreeSet::new();
    let mut safe = 0;
    let mut uncertain = 0;
    let mut zero = 0;
    let mut correct = true;
    for cell in cells {
        let tuples = quartic_cells::product(&cell.parts)?;
        let reference = quartic_oracle::roots(&f.base, &tuples)?;
        for tuple in &reference {
            let (informative, zero_b) = quartic_oracle::incidents(&f.base, tuple)?;
            if cell.valuation_safe {
                safe += informative;
            } else {
                uncertain += informative;
            }
            zero += zero_b;
            correct &= !cell.valuation_safe || zero_b == 0;
        }
        let (complete, timed_out, observed, error) =
            match visit(f, &cell, if shared { Some(&mut pool) } else { None }) {
                Ok((complete, timed_out, roots)) => (complete, timed_out, roots, None),
                Err(error) => (false, false, BTreeSet::new(), Some(error)),
            };
        correct &= complete && !timed_out && error.is_none() && observed == reference;
        union.extend(observed.iter().cloned());
        rows.push(CellResult {
            cell,
            complete,
            timed_out,
            error,
            expected: reference.into_iter().collect(),
            observed: observed.into_iter().collect(),
        });
    }
    correct &= union == expected;
    let roots: Vec<_> = union.into_iter().collect();
    let (recovery, recovery_error) = match aps::recover(&f.curve, f.g, f.q, &f.base, &roots) {
        Ok(recovery) => (recovery, None),
        Err(error) => {
            correct = false;
            (None, Some(error))
        }
    };
    correct &= recovery.is_some() == (safe + uncertain > 0);
    if let Some(r) = &recovery {
        let curve = Curve::new(101, 59, 6)?;
        let (gx, gy) = f.curve.canonical(f.g);
        let (qx, qy) = f.curve.canonical(f.q);
        correct &= r.b != 0
            && f.curve.mul(Some(f.g), r.logarithm) == Some(f.q)
            && curve.mul(Some(Point { x: gx, y: gy }), r.logarithm) == Some(Point { x: qx, y: qy });
    }
    Ok(Route {
        shared,
        cells: rows,
        expected_roots: expected.into_iter().collect(),
        observed_roots: roots,
        safe_informative_incidents: safe,
        uncertain_informative_incidents: uncertain,
        zero_b_incidents: zero,
        recovered_logarithm: recovery.map(|r| r.logarithm),
        recovery_error,
        pool_stats: if shared { Some(pool.stats()) } else { None },
        complete_and_equal: correct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler;

    #[test]
    fn expanded4_cells_match_complete_independent_oracle_and_recovery() {
        for seed in sampler::SEEDS {
            let f = sampler::even_fixture(seed).unwrap();
            let cold = run(&f, seed, false).unwrap();
            let shared = run(&f, seed, true).unwrap();
            assert!(
                cold.complete_and_equal,
                "{}",
                serde_json::to_string(&cold).unwrap()
            );
            assert!(
                shared.complete_and_equal,
                "{}",
                serde_json::to_string(&shared).unwrap()
            );
            assert_eq!(cold.observed_roots, shared.observed_roots);
            assert_eq!(cold.cells.len(), 19);
        }
    }
}
