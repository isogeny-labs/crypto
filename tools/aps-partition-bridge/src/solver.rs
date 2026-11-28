pub use crate::model::{Mode, ModeReport};
use crate::{
    model::{CellResult, PoolResult},
    oracle,
    sampler::{Fixture, FAMILY_SIZE},
};
use aps_partition_lab::{
    coloring,
    curve::Curve,
    signature::{self, Signature},
};
use crypto_lib::cryptanalysis::{
    aps_single_relation::{self as aps, Encoding},
    f4_fp::{self, F4Options, Ordering, TracePool},
};
use std::{
    collections::btree_map::Entry,
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

pub const CALL_BUDGET_MS: u64 = 2000;
type VisitResult = (bool, bool, BTreeSet<Vec<u64>>);

fn visit(
    f: &Fixture,
    parts: &[Vec<u64>],
    pool: Option<&mut TracePool>,
) -> Result<VisitResult, String> {
    let system = aps::build_system(&f.curve, parts, Encoding::Partitioned3)?;
    let domains: Vec<_> = parts.iter().cloned().map(Some).collect();
    let options =
        F4Options::new(Ordering::Grevlex, 24).with_budget(Duration::from_millis(CALL_BUDGET_MS));
    let mut out = BTreeSet::new();
    let mut callback = |point: &[u64]| {
        let mut tuple = point.to_vec();
        tuple.sort_unstable();
        out.insert(tuple);
        false
    };
    let report = if let Some(pool) = pool {
        f4_fp::solve_visit_reusing(
            system.equations(),
            system.variables(),
            system.modulus(),
            &options,
            &domains,
            pool,
            &mut callback,
        )?
    } else {
        f4_fp::solve_visit(
            system.equations(),
            system.variables(),
            system.modulus(),
            &options,
            &domains,
            &mut callback,
        )?
    };
    Ok((report.complete, report.timed_out, out))
}

pub fn run(f: &Fixture, seed: u64, mode: Mode) -> Result<ModeReport, String> {
    let values: Vec<_> = f.base.points.keys().copied().collect();
    let family = coloring::candidate_family(values.len(), 3, FAMILY_SIZE)?;
    let cover = coloring::audit(values.len(), 3, &family)?;
    let allowed: BTreeSet<_> = cover
        .owners
        .iter()
        .map(|(ids, _)| ids.iter().map(|&id| values[id]).collect::<Vec<_>>())
        .collect();
    let all = oracle::full_roots(&f.base)?;
    let expected_union: BTreeSet<_> = all.intersection(&allowed).cloned().collect();
    let mut shared = TracePool::new(&format!("partition-bridge:{seed}:shared"), 8, 100000)?;
    let mut buckets: BTreeMap<Signature, TracePool> = BTreeMap::new();
    let mut cells = Vec::new();
    let mut empty = Vec::new();
    let mut union = BTreeSet::new();
    let mut correct = true;
    for (index, colors) in family.iter().enumerate() {
        let parts: Vec<Vec<_>> = (0..3)
            .map(|color| {
                values
                    .iter()
                    .enumerate()
                    .filter(|(id, _)| colors[*id] == color)
                    .map(|(_, x)| *x)
                    .collect()
            })
            .collect();
        if parts.iter().any(Vec::is_empty) {
            empty.push(index);
            continue;
        }
        for &c in &parts[2] {
            let cell = vec![parts[0].clone(), parts[1].clone(), vec![c]];
            let feature = signature::signature(&signature::specialize(Curve::new(101, 59, 6)?, c)?);
            let expected = oracle::roots(&f.base, &cell)?;
            let pool = match mode {
                Mode::Cold => None,
                Mode::Shared => Some(&mut shared),
                Mode::SignatureBuckets => {
                    let index = buckets.len();
                    Some(match buckets.entry(feature.clone()) {
                        Entry::Occupied(entry) => entry.into_mut(),
                        Entry::Vacant(entry) => entry.insert(TracePool::new(
                            &format!("partition-bridge:{seed}:signature:{index}"),
                            8,
                            100000,
                        )?),
                    })
                }
            };
            let (complete, timed_out, roots, error) = match visit(f, &cell, pool) {
                Ok((complete, timed_out, roots)) => (complete, timed_out, roots, None),
                Err(e) => (false, false, BTreeSet::new(), Some(e)),
            };
            correct &= complete && !timed_out && error.is_none() && roots == expected;
            union.extend(roots.iter().cloned());
            cells.push(CellResult {
                coloring: index,
                fixed_coordinate: c,
                parts: cell,
                feature,
                complete,
                timed_out,
                expected: expected.into_iter().collect(),
                observed: roots.into_iter().collect(),
                error,
            });
        }
    }
    correct &= union == expected_union;
    let sets: Vec<_> = union.iter().cloned().collect();
    let (recovery, recovery_error) = match aps::recover(&f.curve, f.g, f.q, &f.base, &sets) {
        Ok(recovery) => (recovery, None),
        Err(error) => {
            correct = false;
            (None, Some(error))
        }
    };
    let (informative, zero) = oracle::label_incidents(&f.base, &expected_union)?;
    correct &= recovery.is_some() == (informative > 0);
    if let Some(r) = &recovery {
        let reference = Curve::new(101, 59, 6)?;
        let (gx, gy) = f.curve.canonical(f.g);
        let (qx, qy) = f.curve.canonical(f.q);
        correct &= r.b != 0
            && f.curve.mul(Some(f.g), r.logarithm) == Some(f.q)
            && reference.mul(
                Some(aps_partition_lab::curve::Point { x: gx, y: gy }),
                r.logarithm,
            ) == Some(aps_partition_lab::curve::Point { x: qx, y: qy });
    }
    let pools = match mode {
        Mode::Cold => Vec::new(),
        Mode::Shared => vec![PoolResult {
            feature: None,
            stats: shared.stats(),
        }],
        Mode::SignatureBuckets => buckets
            .into_iter()
            .map(|(feature, p)| PoolResult {
                feature: Some(feature),
                stats: p.stats(),
            })
            .collect(),
    };
    let root_incidents = cells.iter().map(|cell| cell.observed.len()).sum();
    Ok(ModeReport {
        mode,
        empty_colorings: empty,
        cells,
        root_incidents,
        represented_roots: sets,
        unrepresented_roots: all.difference(&expected_union).cloned().collect(),
        recovered_logarithm: recovery.map(|r| r.logarithm),
        recovery_error,
        expected_informative_incidents: informative,
        expected_zero_b_incidents: zero,
        pools,
        complete_and_equal: correct,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler;

    #[test]
    fn production_sampler_and_all_trace_routes_match_complete_group_oracle() {
        for seed in sampler::SEEDS {
            let fixture = sampler::fixture(seed).unwrap();
            let mut roots = None;
            for mode in [Mode::Cold, Mode::Shared, Mode::SignatureBuckets] {
                let report = run(&fixture, seed, mode).unwrap();
                assert!(
                    report.complete_and_equal,
                    "{}",
                    serde_json::to_string(&report).unwrap()
                );
                if let Some(old) = &roots {
                    assert_eq!(old, &report.represented_roots);
                } else {
                    roots = Some(report.represented_roots);
                }
            }
        }
    }
}
